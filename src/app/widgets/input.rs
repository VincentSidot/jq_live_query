use std::fmt::Error;

use ratatui::{
    text::{self, Span, Line},
    widgets::{Block, Borders, Paragraph},
    style::Style, backend::Backend,
};

use crossterm::event;

use crate::app::widgets::Drawable;
use crate::app::Config;
use crate::app::app::Selected;

pub struct Input<'a> {
    prompt: &'a str,
    value: String,
    selected: bool,
    config: &'a Config,
    is_valid: bool,
    cursor_position: usize,
}

impl<'a> Input<'a>{
    pub fn new(prompt: &'a str, default: &'a str, config: &'a Config) -> Input<'a> {
        let mut input: String = String::new();
        if default.len() > 0 {
            input = default.to_string();
        }
        Input::<'a> {
            prompt,
            value: input,
            selected: false,
            config,
            is_valid: true,
            cursor_position: default.len(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn set_selected(&mut self, selected: &Selected) {
        match selected {
            Selected::Input => {
                self.selected = true;
            },
            _ => {
                self.selected = false;
            }
        }
    }

    pub fn set_valid(&mut self, is_valid: bool) {
        self.is_valid = is_valid;
    }

    pub fn render_content(&self) -> Result<Line, Error> {
        // Render the content of the input
        // ie: Adding the cursor
        let mut content: Vec<Span> = Vec::new();
        let text_style = Style::default()
            .fg(self.config.color.foreground)
            .bg(self.config.color.background);
        let cursor_style = Style::default()
            .fg(self.config.color.cursor_foreground)
            .bg(self.config.color.cursor_background);


        if self.selected {
            if self.value.len() == 0 {
                // We only need to 
                content.push(
                    Span::styled(
                        " ",
                        cursor_style
                    )
                );
            } else if self.cursor_position == self.value.len() {
                content.push(
                    Span::styled(
                        &self.value[..],
                        text_style                
                    )
                );
                content.push(
                    Span::styled(
                        " ",
                        cursor_style
                    )
                );
            } else {
                content.push(
                    Span::styled(
                        &self.value[..self.cursor_position],
                        text_style                
                    )
                );
                content.push(
                    Span::styled(
                        &self.value[self.cursor_position..self.cursor_position+1],
                        cursor_style                
                    )
                );
                if self.cursor_position < self.value.len() {
                    content.push(
                        Span::styled(
                            &self.value[self.cursor_position+1..],
                            text_style                
                        )
                    );
                };
            }
        } else {
            // We don't want to show the cursor if the input is not selected
            content.push(
                Span::styled(
                    &self.value[..],
                    text_style                
                )
            );
        }

        Ok(Line::from(content))
        
        
    }

    pub fn handle_event(&mut self, event: event::KeyEvent) -> () {
        if self.selected == true {
            match event {
                event::KeyEvent{ // Handle backspace
                    code: event::KeyCode::Backspace,
                    modifiers: _,
                    kind: _,
                    state: _,
                } => {
                    if self.cursor_position > 0 {
                        self.value.remove(self.cursor_position-1);
                        self.cursor_position -= 1;
                    }
                }
                event::KeyEvent{ // Handle delete
                    code: event::KeyCode::Delete,
                    modifiers: _,
                    kind: _,
                    state: _,
                } => {
                    if self.cursor_position < self.value.len() {
                        self.value.remove(self.cursor_position);
                    }
                }
                event::KeyEvent{ // Handle left arrow
                    code: event::KeyCode::Left,
                    modifiers: _,
                    kind: _,
                    state: _,
                } => {
                    if self.cursor_position > 0 {
                        self.cursor_position -= 1;
                    }
                }
                event::KeyEvent{ // Handle right arrow
                    code: event::KeyCode::Right,
                    modifiers: _,
                    kind: _,
                    state: _,
                } => {
                    if self.cursor_position < self.value.len() {
                        self.cursor_position += 1;
                    }
                }
                event::KeyEvent{ // Handle home
                    code: event::KeyCode::Home,
                    modifiers: _,
                    kind: _,
                    state: _,
                } | event::KeyEvent{ // Handle ctrl + a
                    code: event::KeyCode::Char('a') | event::KeyCode::Char('A'),
                    modifiers: event::KeyModifiers::CONTROL,
                    kind: _,
                    state: _,
                } => {
                    self.cursor_position = 0;
                }
                event::KeyEvent{ // Handle end
                    code: event::KeyCode::End,
                    modifiers: _,
                    kind: _,
                    state: _,
                } | event::KeyEvent{ // Handle ctrl + e
                    code: event::KeyCode::Char('e') | event::KeyCode::Char('E'),
                    modifiers: event::KeyModifiers::CONTROL,
                    kind: _,
                    state: _,
                } => {
                    self.cursor_position = self.value.len();
                }
                event::KeyEvent{ // Handle ctrl + l
                    code: event::KeyCode::Char('l') | event::KeyCode::Char('L'),
                    modifiers: event::KeyModifiers::CONTROL,
                    kind: _,
                    state: _,
                } => {
                    self.value.clear();
                    self.cursor_position = 0;
                }
                event::KeyEvent{ // Handle any char
                    code: event::KeyCode::Char(c),
                    modifiers: _,
                    kind: _,
                    state: _,
                } => {
                    self.value.insert(self.cursor_position, c);
                    self.cursor_position += 1;
                }
                _ => {}
            }
        }
    }

}

impl Drawable for Input<'_> {
    fn draw<B: Backend>(
            &self,
            f: &mut ratatui::Frame<B>,
            area: ratatui::layout::Rect
        ) -> Result<(), Error>
    {
        let bg_color = self.config.color.background;
        let fg_color = self.config.color.foreground;
        let selected_fg_color = self.config.color.selected_foreground;
        let valild_fg_color = self.config.color.valid_foreground;
        let invalid_fg_color = self.config.color.invalid_foreground;

        let block = Block::default()
            .title(
                text::Span::styled(
                    self.prompt,
                    Style::default()
                        .fg(if self.selected {selected_fg_color} else {fg_color})
                        .bg(bg_color)
                )
            )
            .borders(Borders::ALL)
            .border_style(
                Style::default()
                    .bg(bg_color)
                    .fg(if self.is_valid {valild_fg_color} else {invalid_fg_color})
            );

        let paragraph = Paragraph::new(self.render_content()?)
            .block(block);
        f.render_widget(paragraph, area);
        Ok(())
    }
}
