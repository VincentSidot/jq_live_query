use ratatui::style::Color;


pub struct ColorConfig {
    foreground: Color,
    background: Color,
    selected_foreground: Color,
}

pub struct Config {
    color: ColorConfig,    
}

impl Config {
    pub fn new() -> Config {
        Config {
            color: ColorConfig {
                foreground: Color::White,
                background: Color::Reset,
                selected_foreground: Color::Yellow,
            },
        }
    }    

    pub fn color(&self) -> &ColorConfig {
        &self.color
    }

    pub fn set_color(&mut self, color: ColorConfig) {
        self.color = color;
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorConfig {
    pub fn foreground(&self) -> &Color {
        &self.foreground
    }

    pub fn background(&self) -> &Color {
        &self.background
    }

    pub fn selected_foreground(&self) -> &Color {
        &self.selected_foreground
    }

    pub fn set_foreground(&mut self, color: Color) {
        self.foreground = color;
    }

    pub fn set_background(&mut self, color: Color) {
        self.background = color;
    }

    pub fn set_selected_foreground(&mut self, color: Color) {
        self.selected_foreground = color;
    }
}