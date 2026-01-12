use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Padding, Paragraph},
    Frame,
};

use crate::app::{App, AppPanel, AppState, GraphicsMode};
use crate::theme::Theme;

pub fn render(frame: &mut Frame, app: &App, theme: &Theme) {
    let area = frame.area();

    frame.render_widget(Block::default().style(Style::default().bg(theme.bg)), area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(area);

    render_header(frame, app, theme, chunks[0]);
    render_main(frame, app, theme, chunks[1]);
    render_footer(frame, theme, chunks[2]);

    if app.state != AppState::Normal {
        render_message(frame, app, theme, area);
    }
}

fn render_header(frame: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    let title = Line::from(vec![
        Span::styled("󰾲 ", Style::default().fg(theme.nvidia_color)),
        Span::styled(
            "Envy",
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "TUI",
            Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
        ),
    ]);

    let current_mode_text = match &app.current_mode {
        Some(mode) => format!("Current Mode: {} {}", mode.icon(), mode),
        None => "Current Mode: Unknown".to_string(),
    };

    let mode_color = app
        .current_mode
        .as_ref()
        .map(|m| theme.mode_color(m))
        .unwrap_or(theme.muted);

    let content = vec![
        Line::from(""),
        title,
        Line::from(Span::styled(
            current_mode_text,
            Style::default().fg(mode_color),
        )),
    ];

    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(theme.border))
        .padding(Padding::horizontal(2));

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

fn render_main(frame: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .margin(1)
        .split(area);

    render_mode_selection(frame, app, theme, chunks[0]);
    render_options(frame, app, theme, chunks[1]);
}

fn render_mode_selection(frame: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    let is_focused = app.active_panel == AppPanel::ModeSelection;
    let border_color = if is_focused {
        theme.border_focused
    } else {
        theme.border
    };

    let block = Block::default()
        .title(" Graphics Mode ")
        .title_style(Style::default().fg(if is_focused {
            theme.accent
        } else {
            theme.muted
        }))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let modes = GraphicsMode::all();
    let mode_height = 4;

    for (i, mode) in modes.iter().enumerate() {
        let is_selected = i == app.selected_mode_index && is_focused;
        let is_current = app.current_mode.as_ref() == Some(mode);
        let mode_color = theme.mode_color(mode);

        let y = inner.y + (i as u16 * mode_height);
        if y + mode_height > inner.y + inner.height {
            break;
        }

        let mode_area = Rect::new(inner.x, y, inner.width, mode_height);

        let bg = if is_selected {
            theme.selection_bg
        } else {
            theme.bg
        };
        let fg = if is_selected { mode_color } else { theme.fg };

        let current_marker = if is_current { " ●" } else { "" };
        let selector = if is_selected { "▶ " } else { "  " };

        let lines = vec![
            Line::from(vec![
                Span::styled(selector, Style::default().fg(theme.accent)),
                Span::styled(format!("{} ", mode.icon()), Style::default().fg(mode_color)),
                Span::styled(
                    format!("{:?}", mode),
                    Style::default().fg(fg).add_modifier(Modifier::BOLD),
                ),
                Span::styled(current_marker, Style::default().fg(theme.success)),
            ]),
            Line::from(Span::styled(
                format!("   {}", mode.description()),
                Style::default().fg(theme.muted),
            )),
        ];

        let paragraph = Paragraph::new(lines).style(Style::default().bg(bg));

        frame.render_widget(paragraph, mode_area);
    }
}

fn render_options(frame: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    let is_focused = app.active_panel == AppPanel::Options;
    let border_color = if is_focused {
        theme.border_focused
    } else {
        theme.border
    };

    let block = Block::default()
        .title(" Options ")
        .title_style(Style::default().fg(if is_focused {
            theme.accent
        } else {
            theme.muted
        }))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .padding(Padding::new(2, 2, 1, 1));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let selected_mode = app.selected_mode();

    let options: Vec<(String, bool, bool)> = match selected_mode {
        GraphicsMode::Hybrid => vec![
            (format!("RTD3 Power Management"), app.rtd3_enabled, true),
            (
                format!("RTD3 Level: {}", app.rtd3_level),
                false,
                app.rtd3_enabled,
            ),
        ],
        GraphicsMode::Nvidia => vec![
            (format!("Force Composition Pipeline"), app.force_comp, true),
            (
                format!("Coolbits (value: {})", app.coolbits_value),
                app.coolbits_enabled,
                true,
            ),
        ],
        GraphicsMode::Integrated => {
            vec![("No additional options available".to_string(), false, false)]
        }
    };

    for (i, (label, is_on, is_toggle)) in options.iter().enumerate() {
        let is_selected = i == app.selected_option_index && is_focused;
        let y = inner.y + (i as u16 * 2);

        if y >= inner.y + inner.height {
            break;
        }

        let option_area = Rect::new(inner.x, y, inner.width, 2);

        let bg = if is_selected {
            theme.selection_bg
        } else {
            theme.bg
        };
        let fg = if is_selected { theme.accent } else { theme.fg };

        let checkbox = if *is_toggle {
            if *is_on {
                "[✓] "
            } else {
                "[ ] "
            }
        } else {
            "    "
        };

        let checkbox_color = if *is_on { theme.success } else { theme.muted };

        let line = Line::from(vec![
            Span::styled(checkbox, Style::default().fg(checkbox_color)),
            Span::styled(label.as_str(), Style::default().fg(fg)),
        ]);

        let paragraph = Paragraph::new(line).style(Style::default().bg(bg));
        frame.render_widget(paragraph, option_area);
    }
}

fn render_footer(frame: &mut Frame, theme: &Theme, area: Rect) {
    let keys = vec![
        ("↑↓/jk", "Navigate"),
        ("Tab", "Switch Panel"),
        ("Enter", "Apply"),
        ("Space", "Toggle"),
        ("r", "Reset"),
        ("q", "Quit"),
    ];

    let spans: Vec<Span> = keys
        .iter()
        .flat_map(|(key, action)| {
            vec![
                Span::styled(
                    format!(" {} ", key),
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(format!("{} ", action), Style::default().fg(theme.muted)),
                Span::styled("│", Style::default().fg(theme.border)),
            ]
        })
        .collect();

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line).alignment(Alignment::Center).block(
        Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(theme.border)),
    );

    frame.render_widget(paragraph, area);
}

fn render_message(frame: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    let width = 50.min(area.width.saturating_sub(4));
    let height = 7;
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;

    let popup_area = Rect::new(x, y, width, height);

    frame.render_widget(Clear, popup_area);

    let (title, border_color, icon) = match app.state {
        AppState::Success => (" Success ", theme.success, " "),
        AppState::Error => (" Error ", theme.error, " "),
        AppState::ConfirmingSwitch | AppState::ConfirmingReboot => {
            (" Confirm ", theme.warning, "󰋼 ")
        }
        AppState::Normal => return,
    };

    let block = Block::default()
        .title(title)
        .title_style(
            Style::default()
                .fg(border_color)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(theme.bg));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    let content = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(icon, Style::default().fg(border_color)),
            Span::styled(&app.message, Style::default().fg(theme.fg)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            match app.state {
                AppState::ConfirmingSwitch | AppState::ConfirmingReboot => {
                    "y/Enter: Sim  |  n/Esc: Não"
                }
                _ => "Pressione qualquer tecla para continuar",
            },
            Style::default().fg(theme.muted),
        )),
    ];

    let paragraph = Paragraph::new(content).alignment(Alignment::Center);

    frame.render_widget(paragraph, inner);
}
