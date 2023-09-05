use ratatui::style::Color;


pub struct _Color {
    pub foreground: Color,
    pub background: Color,
    pub selected_foreground: Color,
    pub valid_foreground: Color,
    pub invalid_foreground: Color,
    pub cursor_background: Color,
    pub cursor_foreground: Color,
}

pub struct _Json {
    pub json_key: Color,
    pub json_string: Color,
    pub json_number: Color,
    pub json_boolean: Color,
    pub json_null: Color,
    pub indent: usize,
}

pub struct Config {
    pub color: _Color,
    pub json: _Json,
}

impl Config {
    pub fn new() -> Config {
        Config {
            color: _Color {
                foreground: Color::White,
                background: Color::Reset,
                selected_foreground: Color::Yellow,
                valid_foreground: Color::Green,
                invalid_foreground: Color::Red,
                cursor_background: Color::White,
                cursor_foreground: Color::Reset,
            },
            json: _Json {
                json_key: Color::Cyan,
                json_string: Color::Green,
                json_number: Color::Blue,
                json_boolean: Color::Yellow,
                json_null: Color::Red,
                indent: 2,
            },
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}