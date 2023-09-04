// This widget displays a JSON object in a tree view.
// In can handle keyboard events to expand and collapse nodes.
// It can also handle mouse events to select nodes.

use ratatui::{
    widgets::{Block, Borders, Paragraph},
    style::Style,
};

use jq_rs;
use serde_json::{self, Value};

use crate::app::widgets::{Drawable, Input};
use crate::app::Config;

pub struct Json<'a> {
    selected: bool,
    config: &'a Config,
    json_payload: String,
    json_filtered: String,
}

impl<'a> Json<'a> {
    pub fn new(json_file_path: String, config: &'a Config) -> Json<'a> {
        let json_payload = std::fs::read_to_string(json_file_path).unwrap();
        
        Json {
            selected: false,
            config,
            json_payload,
            json_filtered: String::new(),
        }
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn apply_filter<'b> (&mut self, input: &'b Input) -> () {
        // I use 'b here because the lifetime of input is shorter than self.
        // We only need input to get the value of the input widget.

        // Inihbit stderr printing in this function
        {
            // Redirect stderr to /dev/null
            // TODO
            eprintln!("Error: {}", "This is an error");
            let result = jq_rs::run(input.value(), &self.json_payload);
            if result.is_ok() {
                let serde_json_value: Result<Value, serde_json::Error> = serde_json::from_str(&result.unwrap());
                if serde_json_value.is_ok() {
                    let value = serde_json::to_string_pretty(&serde_json_value.unwrap()).unwrap();
                    self.json_filtered = format!("{}", value);
                } else {
                    self.json_filtered = format!("Error: {}", serde_json_value.err().unwrap());
                }
            } else {
                //self.json_filtered = format!("Error: {}", result.err().unwrap());
                self.json_filtered = format!("CC");
            }
        }
    }
}

impl Drawable for Json<'_> {
    fn draw(
        &self,
        f: &mut ratatui::Frame<impl ratatui::backend::Backend>,
        area: ratatui::layout::Rect,
    ) {

        let bg_color = *self.config.color().background();
        let fg_color = *self.config.color().foreground();
        let selected_fg_color = *self.config.color().selected_foreground();


        let block = Block::default()
            .title("JQ Result")
            .borders(Borders::ALL)
            .border_style(
                Style::default()
                    .fg(if self.selected {selected_fg_color} else {fg_color})
                    .bg(bg_color)
            );

        let paragraph = Paragraph::new(self.json_filtered.as_str())
            .block(block)
            .style(
                Style::default()
                    .fg(fg_color)
                    .bg(bg_color)
            );
        f.render_widget(paragraph, area);
    }
}