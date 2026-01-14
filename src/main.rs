mod app;
mod envycontrol;
mod theme;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use app::{App, AppPanel, AppState};
use theme::Theme;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--version" | "-V" => {
                println!("envy-tui {}", VERSION);
                return Ok(());
            }
            "--help" | "-h" => {
                println!("envy-tui {} - TUI manager for EnvyControl", VERSION);
                println!();
                println!("Usage: envy-tui [OPTIONS]");
                println!();
                println!("Options:");
                println!("  -V, --version    Print version information");
                println!("  -h, --help       Print this help message");
                return Ok(());
            }
            _ => {}
        }
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let mut app = App::new();
    let theme = Theme::default();

    if !envycontrol::is_envycontrol_installed() {
        app.set_error("envycontrol is not installed. Please install it first.");
    } else {
        match envycontrol::query_mode() {
            Ok(mode) => {
                app.current_mode = mode;
                if mode != Some(app::GraphicsMode::Integrated) {
                    app.gpu_info = envycontrol::query_gpu_info();
                }
            }
            Err(e) => app.set_error(&format!("Failed to query mode: {}", e)),
        }
    }

    while !app.should_quit {
        terminal.draw(|f| ui::render(f, &app, &theme))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            if app.state == AppState::ConfirmingSwitch {
                match key.code {
                    KeyCode::Char('y') | KeyCode::Char('s') | KeyCode::Enter => {
                        let selected = app.pending_mode.unwrap_or(app.selected_mode());
                        let options = envycontrol::SwitchOptions {
                            mode: selected,
                            rtd3_enabled: app.rtd3_enabled,
                            rtd3_level: app.rtd3_level,
                            force_comp: app.force_comp,
                            coolbits_enabled: app.coolbits_enabled,
                            coolbits_value: app.coolbits_value,
                        };

                        app.set_loading("Applying changes...");

                        let (tx, rx) = mpsc::channel();
                        thread::spawn(move || {
                            let result = envycontrol::switch_mode(options);
                            let _ = tx.send((result, selected));
                        });

                        loop {
                            terminal.draw(|f| ui::render(f, &app, &theme))?;

                            match rx.try_recv() {
                                Ok((result, mode)) => {
                                    match result {
                                        Ok(_) => {
                                            app.current_mode = Some(mode);
                                            app.pending_mode = None;
                                            app.state = AppState::ConfirmingReboot;
                                            app.message = "Mode changed successfully! Do you want to reboot now?".to_string();
                                        }
                                        Err(e) => {
                                            app.pending_mode = None;
                                            app.set_error(&e.to_string());
                                        }
                                    }
                                    break;
                                }
                                Err(mpsc::TryRecvError::Empty) => {
                                    app.tick_spinner();
                                    thread::sleep(Duration::from_millis(100));
                                }
                                Err(mpsc::TryRecvError::Disconnected) => {
                                    app.set_error("Command failed unexpectedly");
                                    break;
                                }
                            }
                        }
                    }
                    KeyCode::Char('n') | KeyCode::Esc => {
                        app.pending_mode = None;
                        app.clear_message();
                    }
                    _ => {}
                }
                continue;
            }

            if app.state == AppState::ConfirmingReboot {
                match key.code {
                    KeyCode::Char('y') | KeyCode::Char('s') | KeyCode::Enter => {
                        if let Err(e) = envycontrol::reboot() {
                            app.set_error(&format!("Failed to reboot: {}", e));
                        }
                    }
                    KeyCode::Char('n') | KeyCode::Esc => {
                        app.set_success(
                            "Changes applied. Reboot the computer for changes to take effect.",
                        );
                    }
                    _ => {}
                }
                continue;
            }

            if app.state != AppState::Normal {
                app.clear_message();
                continue;
            }

            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    app.should_quit = true;
                }
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.should_quit = true;
                }
                KeyCode::Tab => {
                    app.toggle_panel();
                }
                KeyCode::Up | KeyCode::Char('k') => match app.active_panel {
                    AppPanel::ModeSelection => app.previous_mode(),
                    AppPanel::Options => app.previous_option(),
                },
                KeyCode::Down | KeyCode::Char('j') => match app.active_panel {
                    AppPanel::ModeSelection => app.next_mode(),
                    AppPanel::Options => app.next_option(),
                },
                KeyCode::Char(' ') => {
                    if app.active_panel == AppPanel::Options {
                        app.toggle_current_option();
                    }
                }
                KeyCode::Enter => {
                    let selected = app.selected_mode();
                    app.pending_mode = Some(selected);
                    app.state = AppState::ConfirmingSwitch;
                    app.message = format!("Switch to {} mode? (y/n)", selected);
                }
                KeyCode::Char('r') => {
                    app.set_loading("Resetting...");

                    let (tx, rx) = mpsc::channel();
                    thread::spawn(move || {
                        let result = envycontrol::reset();
                        let _ = tx.send(result);
                    });

                    loop {
                        terminal.draw(|f| ui::render(f, &app, &theme))?;

                        match rx.try_recv() {
                            Ok(result) => {
                                match result {
                                    Ok(msg) => {
                                        app.current_mode = None;
                                        app.set_success(&msg);
                                    }
                                    Err(e) => app.set_error(&e.to_string()),
                                }
                                break;
                            }
                            Err(mpsc::TryRecvError::Empty) => {
                                app.tick_spinner();
                                thread::sleep(Duration::from_millis(100));
                            }
                            Err(mpsc::TryRecvError::Disconnected) => {
                                app.set_error("Command failed unexpectedly");
                                break;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}
