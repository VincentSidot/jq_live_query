// This widget displays a JSON object in a tree view.
// In can handle keyboard events to expand and collapse nodes.
// It can also handle mouse events to select nodes.

use ratatui::{
    widgets::{Block, Borders, Paragraph, block::Title},
    text::{Line, Span, Text},
    style::Style, backend::Backend,
    prelude::Alignment
    
};

use crossterm::event;

use std::string::String;

use crate::app::widgets::Drawable;
use crate::app::Config;
use serde_json::{self, Value};

pub struct Json<'a> {
    selected: bool,
    config: &'a Config,
    raw: String,
    json: Text<'a>,
    json_lines_count: usize,
    title: String,
    right_title: Option<String>,
    scroll: usize,
}



impl<'a> Json<'a> {
    pub fn new(json_payload: String, title: String, right_title: Option<String>, config: &'a Config) -> Json<'a> {
        let mut json = Json {
            selected: false,
            config,
            raw: String::from("Loading..."),
            title,
            right_title,
            scroll: 0,
            json: Text::from(Span::styled(
                "Loading...",
                Style::default()
                    .fg(config.color.foreground)
                    .bg(config.color.background)
            )),
            json_lines_count: 0,
        };
        json.set_json(json_payload);
        json
    }
    
    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    fn recursive_parser<'b> (&self, spans: &'b mut Vec<Vec<Span>>,value: Value, indent: usize) {
        // When we get newline we push a new Vec<Span> to spans
        // We work only on the last Vec<Span> of spans

        let style_key = Style::default()
            .fg(self.config.json.json_key)
            .bg(self.config.color.background);
        let style_string = Style::default()
            .fg(self.config.json.json_string)
            .bg(self.config.color.background);
        let style_number = Style::default()
            .fg(self.config.json.json_number)
            .bg(self.config.color.background);
        let style_boolean = Style::default()
            .fg(self.config.json.json_boolean)
            .bg(self.config.color.background);
        let style_null = Style::default()
            .fg(self.config.json.json_null)
            .bg(self.config.color.background);
        let style_default = Style::default()
            .fg(self.config.color.foreground)
            .bg(self.config.color.background);
        let indent_increment = self.config.json.indent;

        match value {
            Value::Null => {
                spans.last_mut().unwrap().push(Span::styled(
                    "null",
                    style_null
                ));
            },
            Value::Bool(b) => {
                spans.last_mut().unwrap().push(Span::styled(
                    b.to_string(),
                    style_boolean
                ));                
            },
            Value::Number(n) => {
                spans.last_mut().unwrap().push(Span::styled(
                    n.to_string(),
                    style_number
                ));
            },
            Value::String(s) => {
                spans.last_mut().unwrap().push(Span::styled(
                    format!("\"{}\"", s),
                    style_string
                ));
            },
            Value::Array(a) => {
                let mut last_span = spans.last_mut().unwrap();
                //Push [
                last_span.push(Span::styled(
                    "[",
                    style_default
                ));
                for (i, v) in a.iter().enumerate() {
                    spans.push(Vec::new());
                    //Push indent
                    spans.last_mut().unwrap().push(Span::styled(
                        format!("{:indent$}", "", indent = indent + indent_increment),
                        style_default
                    ));
                    self.recursive_parser(spans, v.clone(), indent + indent_increment);
                    if i < a.len() - 1 {
                        spans.last_mut().unwrap().push(Span::styled(
                            ",",
                            style_default
                        ));
                    }
                }
                spans.push(Vec::new());
                last_span = spans.last_mut().unwrap();
                //Push indent
                last_span.push(Span::styled(
                    format!("{:indent$}", "", indent = indent),
                    style_default
                ));
                //Push ]
                last_span.push(Span::styled(
                    "]",
                    style_default
                ));
            },
            Value::Object(o) => {
                let mut last_span = spans.last_mut().unwrap();
                //Push {
                last_span.push(Span::styled(
                    "{",
                    style_default
                ));

                for (i, (k, v)) in o.iter().enumerate() {
                    spans.push(Vec::new());
                    //Push indent
                    spans.last_mut().unwrap().push(Span::styled(
                        format!("{:indent$}", "", indent = indent + indent_increment),
                        style_default
                    ));
                    //Push key
                    spans.last_mut().unwrap().push(Span::styled(
                        format!("\"{}\": ", k),
                        style_key
                    ));
                    self.recursive_parser(spans, v.clone(), indent + indent_increment);
                    if i < o.len() - 1 {
                        spans.last_mut().unwrap().push(Span::styled(
                            ",",
                            style_default
                        ));
                    }
                }
                spans.push(Vec::new());
                last_span = spans.last_mut().unwrap();
                //Push indent
                last_span.push(Span::styled(
                    format!("{:indent$}", "", indent = indent),
                    style_default
                ));
                //Push }
                last_span.push(Span::styled(
                    "}",
                    style_default
                ));


            }
        };
    }

    fn pretty_json(&mut self) -> Option<Text<'a>> {
        match serde_json::from_str(&self.raw) {
            Ok(serde_json_value) => {
                
                let mut spans: Vec<Vec<Span>> = Vec::new();
                spans.push(Vec::new());
                self.recursive_parser(&mut spans, serde_json_value, 0);
                let mut lines: Vec<Line> = Vec::new();
                for span in spans {
                    lines.push(Line::from(span));
                }
                self.json_lines_count = lines.len();
                Some(Text::from(lines))
            },
            Err(_) => {
                return None
            }
        }
    }

    pub fn json(&self) -> &str {
        &self.raw
    }

    pub fn set_json(&mut self, json: String) {
        let json_text: Text;
        self.raw = json;
        self.json = match self.pretty_json() {
            Some(pretty_json) => pretty_json,
            None => {
                let is_error = {
                    // Check if the string starts with "Error: "
                    let error_prefix = "Error: ";
                    self.raw.starts_with(error_prefix)
                };
                json_text = Text::from(Span::styled(
                        self.raw.clone(),
                        Style::default()
                            .fg(
                                if is_error {self.config.color.invalid_foreground}
                                else {self.config.color.valid_foreground}
                            )
                            .bg(self.config.color.background)
                    ));
                json_text
            },
        };
    }

    pub fn handle_event(&mut self, event: event::KeyEvent) -> () {
        match event {
            event::KeyEvent { //Handle keyboard up
                code: event::KeyCode::Up,
                modifiers: _,
                kind: _,
                state: _,
            } => {
                if self.scroll > 0 {
                    self.scroll -= 1;
                }
            },
            event::KeyEvent { //Handle keyboard down
                code: event::KeyCode::Down,
                modifiers: _,
                kind: _,
                state: _,
            } => {
                if self.scroll < self.json_lines_count - 1 {
                    self.scroll += 1;
                }
            },
            _ => {}
        }
    }
}

impl Drawable for Json<'_> {
    fn draw<B: Backend>(
            &self,
            f: &mut ratatui::Frame<B>,
            area: ratatui::layout::Rect,
        ) -> Result<(), std::fmt::Error> {

        let bg_color = self.config.color.background;
        let fg_color = self.config.color.foreground;
        let selected_fg_color = self.config.color.selected_foreground;

        let right_title: String = match &self.right_title {
            Some(right_title) => right_title.clone(),
            None => String::new()
        }
        ;
        
        let content = Paragraph::new(self.json.clone())
            .block(Block::default()
                .title(
                    Span::styled(
                        self.title.as_str(),
                        Style::default()
                            .fg(if self.selected {selected_fg_color} else {fg_color})
                            .bg(bg_color)
                    )
                )
                .title(
                    Title::from(
                        Span::styled(
                            right_title.as_str(),
                            Style::default()
                                .fg(if self.selected {selected_fg_color} else {fg_color})
                                .bg(bg_color)
                        )
                    )
                    .alignment(Alignment::Right)
                )
                .borders(Borders::ALL)
                .border_style(
                    Style::default()
                        .fg(fg_color)
                        .bg(bg_color)
                )
            )
            .style(Style::default()
                .fg(fg_color)
                .bg(bg_color)
            );
        
        f.render_widget(content, area);
        Ok(())
        
    }
}