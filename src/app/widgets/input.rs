use ratatui::{
    text,
    widgets::{Block, Borders, Paragraph},
    style::Style,
};

use crossterm::event;

use crate::app::widgets::Drawable;
use crate::app::Config;

pub struct Input<'a> {
    prompt: &'a str,
    value: String,
    selected: bool,
    config: &'a Config,
}

impl<'a> Input<'a>{
    pub fn new(prompt: &'a str, config: &'a Config) -> Input<'a> {
        Input::<'a> {
            prompt,
            value: String::new(),
            selected: false,
            config,
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_string();
    }

    pub fn clear(&mut self) {
        self.value.clear();
    }

    pub fn set_prompt(&mut self, prompt: &'a str) {
        self.prompt = prompt;
    }

    pub fn prompt(&self) -> &str {
        &self.prompt
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn handle_event(&mut self, event: event::KeyEvent) -> () {
        if self.selected == true {
            match event.code {
                event::KeyCode::Char(c) => {
                    self.value.push(c);
                }
                event::KeyCode::Backspace => {
                    self.value.pop();
                }
                _ => {}
            }
        }
    }
}

impl Drawable for Input<'_> {
    fn draw(
            &self,
            f: &mut ratatui::Frame<impl ratatui::backend::Backend>,
            area: ratatui::layout::Rect
        )
    {
        let bg_color = *self.config.color().background();
        let fg_color = *self.config.color().foreground();
        let selected_fg_color = *self.config.color().selected_foreground();

        let block = Block::default()
            .title(self.prompt)
            .borders(Borders::ALL)
            .border_style(
                Style::default()
                    .bg(bg_color)
                    .fg(if self.selected {selected_fg_color} else {fg_color})
            );
        let inner_area = block.inner(area);

        let text = text::Span::raw(format!("{}", self.value));
        let paragraph = Paragraph::new(text)
            .block(Block::default())
            .style(
                Style::default()
                    .fg(fg_color)
                    .bg(bg_color)
            );
        f.render_widget(block, area);
        f.render_widget(paragraph, inner_area);
    }
}
