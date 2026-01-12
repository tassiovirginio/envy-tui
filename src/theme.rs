use ratatui::style::Color;

pub struct Theme {
    pub bg: Color,
    pub fg: Color,
    pub accent: Color,
    pub success: Color,
    pub error: Color,
    pub warning: Color,
    pub muted: Color,
    pub integrated_color: Color,
    pub hybrid_color: Color,
    pub nvidia_color: Color,
    pub border: Color,
    pub border_focused: Color,
    pub selection_bg: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bg: Color::Rgb(22, 22, 30),
            fg: Color::Rgb(220, 220, 230),
            accent: Color::Rgb(139, 92, 246),
            success: Color::Rgb(34, 197, 94),
            error: Color::Rgb(239, 68, 68),
            warning: Color::Rgb(234, 179, 8),
            muted: Color::Rgb(100, 100, 120),
            integrated_color: Color::Rgb(59, 130, 246),
            hybrid_color: Color::Rgb(16, 185, 129),
            nvidia_color: Color::Rgb(118, 185, 0),
            border: Color::Rgb(60, 60, 80),
            border_focused: Color::Rgb(139, 92, 246),
            selection_bg: Color::Rgb(40, 40, 60),
        }
    }
}

impl Theme {
    pub fn mode_color(&self, mode: &crate::app::GraphicsMode) -> Color {
        match mode {
            crate::app::GraphicsMode::Integrated => self.integrated_color,
            crate::app::GraphicsMode::Hybrid => self.hybrid_color,
            crate::app::GraphicsMode::Nvidia => self.nvidia_color,
        }
    }
}
