use ratatui::{
    layout::{Layout, Direction, Constraint},
    backend::Backend,
};

use crate::app::widgets::{Drawable, Input, Json};
use crate::app::Config;
use crate::app::app::Selected;

use jq_rs;
pub struct Jq<'a> {
    pub json_base: Json<'a>,
    pub json_filtered: Json<'a>,
    need_to_clear: bool,
}

impl<'a> Jq<'a> {
    pub fn new(json_file_path: String, config: &'a Config) -> Jq<'a> {
        let json_file_path_clone = json_file_path.clone();
        match std::fs::read_to_string(json_file_path) {
            Ok(json) => {
                Jq {
                    json_base: Json::new(
                        json,
                        "JSON File".to_string(),
                        Some(json_file_path_clone),
                        config
                    ),
                    json_filtered: Json::new(
                        String::new(),
                        "JQ Output".to_string(),
                        None,
                        config
                    ),
                    need_to_clear: false,
                }
            },
            Err(_) => {
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

    pub fn apply_filter (&mut self, input: &mut Input) -> () {
        self.json_filtered.set_json(
            match jq_rs::run(input.value(), self.json_base.json()) {
                Ok(result) => {
                    input.set_valid(true);
                    result
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