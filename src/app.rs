use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsMode {
    Integrated,
    Hybrid,
    Nvidia,
}

impl fmt::Display for GraphicsMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphicsMode::Integrated => write!(f, "integrated"),
            GraphicsMode::Hybrid => write!(f, "hybrid"),
            GraphicsMode::Nvidia => write!(f, "nvidia"),
        }
    }
}

impl GraphicsMode {
    pub fn description(&self) -> &str {
        match self {
            GraphicsMode::Integrated => "Use Intel/AMD iGPU exclusively. Nvidia GPU is turned off for power saving.",
            GraphicsMode::Hybrid => "Enable PRIME render offloading. GPU can be dynamically turned off when not in use.",
            GraphicsMode::Nvidia => "Use Nvidia dGPU exclusively. Higher performance, higher power consumption.",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            GraphicsMode::Integrated => "󰍹",
            GraphicsMode::Hybrid => "󰢮",
            GraphicsMode::Nvidia => "󰾲",
        }
    }

    pub fn all() -> Vec<GraphicsMode> {
        vec![GraphicsMode::Integrated, GraphicsMode::Hybrid, GraphicsMode::Nvidia]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rtd3Level {
    Disabled,
    CoarseGrained,
    FineGrained,
    FineGrainedAmpere,
}

impl fmt::Display for Rtd3Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rtd3Level::Disabled => write!(f, "0 - Disabled"),
            Rtd3Level::CoarseGrained => write!(f, "1 - Coarse-grained"),
            Rtd3Level::FineGrained => write!(f, "2 - Fine-grained"),
            Rtd3Level::FineGrainedAmpere => write!(f, "3 - Fine-grained (Ampere+)"),
        }
    }
}

impl Rtd3Level {
    pub fn value(&self) -> u8 {
        match self {
            Rtd3Level::Disabled => 0,
            Rtd3Level::CoarseGrained => 1,
            Rtd3Level::FineGrained => 2,
            Rtd3Level::FineGrainedAmpere => 3,
        }
    }

    pub fn all() -> Vec<Rtd3Level> {
        vec![
            Rtd3Level::Disabled,
            Rtd3Level::CoarseGrained,
            Rtd3Level::FineGrained,
            Rtd3Level::FineGrainedAmpere,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppPanel {
    ModeSelection,
    Options,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Normal,
    Confirming,
    Success,
    Error,
}

pub struct App {
    pub current_mode: Option<GraphicsMode>,
    pub selected_mode_index: usize,
    pub selected_option_index: usize,
    pub active_panel: AppPanel,
    pub state: AppState,
    pub message: String,
    pub rtd3_enabled: bool,
    pub rtd3_level: Rtd3Level,
    pub force_comp: bool,
    pub coolbits_enabled: bool,
    pub coolbits_value: u8,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_mode: None,
            selected_mode_index: 0,
            selected_option_index: 0,
            active_panel: AppPanel::ModeSelection,
            state: AppState::Normal,
            message: String::new(),
            rtd3_enabled: false,
            rtd3_level: Rtd3Level::FineGrained,
            force_comp: false,
            coolbits_enabled: false,
            coolbits_value: 28,
            should_quit: false,
        }
    }

    pub fn selected_mode(&self) -> GraphicsMode {
        GraphicsMode::all()[self.selected_mode_index]
    }

    pub fn next_mode(&mut self) {
        let modes = GraphicsMode::all();
        self.selected_mode_index = (self.selected_mode_index + 1) % modes.len();
    }

    pub fn previous_mode(&mut self) {
        let modes = GraphicsMode::all();
        self.selected_mode_index = if self.selected_mode_index == 0 {
            modes.len() - 1
        } else {
            self.selected_mode_index - 1
        };
    }

    pub fn next_option(&mut self) {
        self.selected_option_index = (self.selected_option_index + 1) % 4;
    }

    pub fn previous_option(&mut self) {
        self.selected_option_index = if self.selected_option_index == 0 {
            3
        } else {
            self.selected_option_index - 1
        };
    }

    pub fn toggle_panel(&mut self) {
        self.active_panel = match self.active_panel {
            AppPanel::ModeSelection => AppPanel::Options,
            AppPanel::Options => AppPanel::ModeSelection,
        };
    }

    pub fn toggle_current_option(&mut self) {
        match self.selected_option_index {
            0 => self.rtd3_enabled = !self.rtd3_enabled,
            1 => {
                let levels = Rtd3Level::all();
                let current_idx = levels.iter().position(|&l| l == self.rtd3_level).unwrap_or(0);
                self.rtd3_level = levels[(current_idx + 1) % levels.len()];
            }
            2 => self.force_comp = !self.force_comp,
            3 => self.coolbits_enabled = !self.coolbits_enabled,
            _ => {}
        }
    }

    pub fn set_success(&mut self, msg: &str) {
        self.state = AppState::Success;
        self.message = msg.to_string();
    }

    pub fn set_error(&mut self, msg: &str) {
        self.state = AppState::Error;
        self.message = msg.to_string();
    }

    pub fn clear_message(&mut self) {
        self.state = AppState::Normal;
        self.message.clear();
    }
}
