use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;

use crate::app::{
    Config,
    widgets::{
        Input,
        Jq,
        Drawable
    }
};

pub enum Selected {
    Input,
    JsonFiltered,
    JsonBase,
}

impl Selected {
    fn next(&mut self) -> &Selected{
        match self {
            Selected::Input => {
                *self = Selected::JsonFiltered;
            },
            Selected::JsonFiltered => {
                *self = Selected::JsonBase;
            },
            Selected::JsonBase => {
                *self = Selected::Input;
            },
        }
        self
    }
    fn prev(&mut self) -> &Selected{
        match self {
            Selected::Input => {
                *self = Selected::JsonBase;
            },
            Selected::JsonFiltered => {
                *self = Selected::Input;
            },
            Selected::JsonBase => {
                *self = Selected::JsonFiltered;
            },
        }
        self
    }
}

pub struct App<'a> {
    terminal: Box<Terminal<CrosstermBackend<io::Stdout>>>,
    input: Input<'a>,
    json_output: Jq<'a>,
    selected: Selected,
}

impl App<'_> {
    pub fn new<'a>(json_file_path: String, config:&'a Config) -> Result<App<'a>, io::Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = Box::new(Terminal::new(backend).unwrap());

        let selected = Selected::Input;
        
        let mut input = Input::new("Input", ".", config);
        input.set_selected(&selected);
        let json_output = Jq::new(json_file_path, config);

        Ok(App{
            terminal,
            input,
            json_output,
            selected,
        })
    }

    fn clean_up(&mut self) -> Result<(), io::Error>{
        // restore terminal
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    pub fn run(&mut self) -> Result<i32, io::Error> {
        let result = self.run_internal();
        self.clean_up()?;
        return match result {
            Ok(_) => Ok(0),
            Err(_) => Ok(1),
        };
    }

    fn run_internal(&mut self) -> Result<(), io::Error> {
            let mut is_running: bool = true;
            self.json_output.apply_filter(&mut self.input);
            self.render()?;
            while is_running {
                match event::read().unwrap() {
                    event::Event::Key(event::KeyEvent {
                        code: event::KeyCode::Char('c'),
                        modifiers: event::KeyModifiers::CONTROL,
                        kind: _,
                        state: _,
                    }) => {
                        is_running = false;
                    }
                    event::Event::Key(event::KeyEvent {
                        code: event::KeyCode::Tab,
                        modifiers: event::KeyModifiers::CONTROL | event::KeyModifiers::SHIFT | event::KeyModifiers::SUPER,
                        kind: _,
                        state: _,
                    }) => {
                        self.selected.prev();
                        self.input.set_selected(&self.selected);
                        self.json_output.set_selected(&self.selected);
                    }                    
                    event::Event::Key(event::KeyEvent {
                        code: event::KeyCode::Tab,
                        modifiers: event::KeyModifiers::NONE,
                        kind: _,
                        state: _,
                    }) => {
                        self.selected.next();
                        self.input.set_selected(&self.selected);
                        self.json_output.set_selected(&self.selected);
                    }
                    event::Event::Key(event) => {
                        match self.selected {
                            Selected::Input => {
                                self.input.handle_event(&event);
                                self.json_output.apply_filter(&mut self.input);
                            },
                            Selected::JsonFiltered | Selected::JsonBase => {
                                self.json_output.handle_event(&event);
                            },
                        }
                    }
                    _ => {}
                }
                self.render()?;
            }
        Ok(())
    }

    fn render(&mut self) -> Result<(), io::Error> {
        // Clear the screen
        if self.json_output.consume_clear() {
            self.terminal.clear()?;
        }

        let size = self.terminal.size()?;
        // input widget should be 3 rows high
        let input_area = ratatui::layout::Rect::new(
            size.x,
            size.y,
            size.width,
            3,
        );
        let output_area = ratatui::layout::Rect::new(
            size.x,
            size.y + 3,
            size.width,
            size.height - 3,
        );

        self.terminal.draw(|f| {
            match self.input.draw(f, input_area) {
                Ok(_) => {},
                Err(_) => {
                    panic!("Error while drawing input widget")
                }
            }
            match self.json_output.draw(f, output_area) {
                Ok(_) => {},
                Err(_) => {
                    panic!("Error while drawing json widget")
                }
            }
            // self.input.draw(f, input_area);
            // self.json_output.draw(f, output_area);
        })?;
        Ok(())
    }
}
