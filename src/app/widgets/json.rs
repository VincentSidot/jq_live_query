// This widget displays a JSON object in a tree view.
// In can handle keyboard events to expand and collapse nodes.
// It can also handle mouse events to select nodes.

use ratatui::{
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Direction, Constraint},
    style::Style, text, backend::Backend,
};

use jq_rs;
use serde_json::{self, Value};

use crate::app::widgets::{Drawable, Input};
use crate::app::Config;

pub struct Json<'a> {
    selected: u8,
    config: &'a Config,
    json_payload: String,
    json_filtered: String,
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

impl<'a> Json<'a> {
    pub fn new(json_file_path: String, config: &'a Config) -> Json<'a> {
        match pretty_json(&std::fs::read_to_string(json_file_path).unwrap()) {
            Some(pretty_json) => {
                Json {
                    selected: 0,
                    config,
                    json_payload: pretty_json,
                    json_filtered: String::new(),
                    need_to_clear: false,
                }
            },
            None => {
                panic!("Invalid JSON file")
            }
        }
    }

    pub fn set_selected(&mut self, selected: u8) {
        self.selected = selected;
    }

    pub fn selected(&self) -> u8 {
        self.selected
    }

    pub fn consuume_clear(&mut self) -> bool {
        let need_to_clear = self.need_to_clear;
        self.need_to_clear = false;
        need_to_clear
    }

    pub fn apply_filter<'b> (&mut self, input: &'b mut Input) -> () {
        // I use 'b here because the lifetime of input is shorter than self.
        // We only need input to get the value of the input widget.
        match jq_rs::run(input.value(), &self.json_payload) {
            Ok(result) => {
                match pretty_json(&result) {
                    Some(pretty_json) => {
                        self.json_filtered = pretty_json;
                        input.set_valid(true);
                    },
                    None => {
                        self.json_filtered = format!("Error: Invalid JSON");
                        input.set_valid(false);
                    }
                }
            },
            Err(error) => {
                self.json_filtered = format!("Error: {}", error);
                input.set_valid(false);
                self.need_to_clear = true;
            }
        }
    }
}

impl Drawable for Json<'_> {
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

        let answer_content = Paragraph::new(self.json_filtered.as_str())
            .block(
                Block::default()
                    .title(
                        text::Span::styled(
                            "JQ Output",
                            Style::default()
                                .fg(if self.selected==1 {selected_fg_color} else {fg_color})
                                .bg(bg_color)
                        )
                    )
                    .borders(Borders::ALL)
                    .border_style(
                        Style::default()
                            .fg(fg_color)
                            .bg(bg_color)
                    )
            )
            .style(
                Style::default()
                    .fg(fg_color)
                    .bg(bg_color)
            );

        let base_content = Paragraph::new(self.json_payload.as_str())
            .block(
                Block::default()
                    .title(
                        text::Span::styled(
                            "JSON File",
                            Style::default()
                                .fg(if self.selected==2 {selected_fg_color} else {fg_color})
                                .bg(bg_color)
                        )
                    )
                    .borders(Borders::ALL)
                    .border_style(
                        Style::default()
                            .fg(fg_color)
                            .bg(bg_color)
                    )
            )
            .style(
                Style::default()
                    .fg(fg_color)
                    .bg(bg_color)
            );

        f.render_widget(base_content, layout[1]);
        f.render_widget(answer_content, layout[0]);
        Ok(())
    }
}