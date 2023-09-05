// This widget displays a JSON object in a tree view.
// In can handle keyboard events to expand and collapse nodes.
// It can also handle mouse events to select nodes.

use ratatui::{
    widgets::{Block, Borders, Paragraph},
    style::Style, text, backend::Backend,
};

use crate::app::widgets::Drawable;
use crate::app::Config;

pub struct Json<'a> {
    selected: bool,
    config: &'a Config,
    pub json: String,
    title: String,
}

impl<'a> Json<'a> {
    pub fn new(json_payload: String, title: String, config: &'a Config) -> Json<'a> {
        Json {
            selected: false,
            config,
            json: json_payload,
            title,
        }
    }
    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn json(&self) -> &str {
        &self.json
    }

    pub fn set_json(&mut self, json: String) {
        self.json = json;
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
        
        let content = Paragraph::new(self.json.as_str())
            .block(Block::default()
                .title(
                    text::Span::styled(
                        self.title.as_str(),
                        Style::default()
                            .fg(if self.selected {selected_fg_color} else {fg_color})
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
            .style(Style::default()
                .fg(fg_color)
                .bg(bg_color)
            );
        
        f.render_widget(content, area);
        Ok(())
        
    }
}