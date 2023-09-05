use ratatui::{
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Direction, Constraint},
    style::Style, text, backend::Backend,
};

use crate::app::widgets::{Drawable, Input, Json};
use crate::app::Config;
use crate::app::app::Selected;

use jq_rs;
use serde_json::{self, Value};

pub struct Jq<'a> {
    config: &'a Config,
    pub json_base: Json<'a>,
    pub json_filtered: Json<'a>,
    need_to_clear: bool,
}

fn pretty_json(json: &String) -> Option<String> {
    let serde_json_value: Result<Value, serde_json::Error> = serde_json::from_str(json);
    if serde_json_value.is_ok() {
        let pretty_json = serde_json::to_string_pretty(&serde_json_value.unwrap());
        if pretty_json.is_ok() {
            return Some(pretty_json.unwrap());
        } else {
            return None;
        }
        
    } else {
        return None;        
    }
}

impl<'a> Jq<'a> {
    pub fn new(json_file_path: String, config: &'a Config) -> Jq<'a> {
        match pretty_json(&std::fs::read_to_string(json_file_path).unwrap()) {
            Some(pretty_json) => {
                Jq {
                    config,
                    json_base: Json::new(
                        pretty_json,
                        "JSON File".to_string(),
                        config
                    ),
                    json_filtered: Json::new(
                        String::new(),
                        "JQ Output".to_string(),
                        config
                    ),
                    
                    need_to_clear: false,
                }
            },
            None => {
                panic!("Invalid JSON file")
            }
        }
    }

    pub fn consume_clear(&mut self) -> bool {
        let need_to_clear = self.need_to_clear;
        self.need_to_clear = false;
        need_to_clear
    }

    pub fn set_selected(&mut self, selected: &Selected) {
        match selected {
            Selected::JsonBase => {
                self.json_base.set_selected(true);
                self.json_filtered.set_selected(false);
            },
            Selected::JsonFiltered => {
                self.json_base.set_selected(false);
                self.json_filtered.set_selected(true);
            },
            _ => {
                self.json_base.set_selected(false);
                self.json_filtered.set_selected(false);
            }
        }
    }

    pub fn apply_filter<'b> (&mut self, input: &'b mut Input) -> () {
        // I use 'b here because the lifetime of input is shorter than self.
        // We only need input to get the value of the input widget.
        self.json_filtered.set_json(
            match jq_rs::run(input.value(), &self.json_base.json()) {
                Ok(result) => {
                    match pretty_json(&result) {
                        Some(pretty_json) => {
                            input.set_valid(true);
                            pretty_json
                        },
                        None => {
                            input.set_valid(false);
                            format!("Error: Invalid JSON")
                        }
                    }
                },
                Err(error) => {
                    input.set_valid(false);
                    self.need_to_clear = true;
                    format!("Error: {}", error)
                }
            }
        );
    }
}

impl Drawable for Jq<'_> {
    fn draw<B: Backend>(
        &self,
        f: &mut ratatui::Frame<B>,
        area: ratatui::layout::Rect,
    ) -> Result<(), std::fmt::Error>{

        let bg_color = self.config.color.background;
        let fg_color = self.config.color.foreground;
        let selected_fg_color = self.config.color.selected_foreground;

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(area);

        self.json_filtered.draw(f, layout[0])?;
        self.json_base.draw(f, layout[1])?;
        Ok(())
    }
}