use ratatui::style::Color;


pub struct ColorConfig {
    pub foreground: Color,
    pub background: Color,
    pub selected_foreground: Color,
    pub valid_foreground: Color,
    pub invalid_foreground: Color,
    pub cursor_background: Color,
    pub cursor_foreground: Color,
}

pub struct Config {
    pub color: ColorConfig,    
}

impl Config {
    pub fn new() -> Config {
        Config {
            color: ColorConfig {
                foreground: Color::White,
                background: Color::Reset,
                selected_foreground: Color::Yellow,
                valid_foreground: Color::Green,
                invalid_foreground: Color::Red,
                cursor_background: Color::White,
                cursor_foreground: Color::Reset,
            },
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}