use std::{error::Error, io};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, widgets::Paragraph};

// Ratatui first interactive application

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = ratatui::run(app);
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> io::Result<()> {
    App::default().run(terminal)
}

#[derive(Debug, Default)]
struct App {
    counter: u8,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| {
                frame.render_widget(
                    Paragraph::new(format!(" Counter : {}", self.counter)),
                    frame.area(),
                );
            })?;
            let _ = self.handle_key_events();
        }
        Ok(())
    }

    fn handle_key_events(&mut self) -> io::Result<()> {
        if let Event::Key(event_key) = crossterm::event::read()? {
            let app_action = match event_key.kind {
                KeyEventKind::Press => match event_key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => AppAction::Close,
                    KeyCode::Up | KeyCode::Right => AppAction::Increment,
                    KeyCode::Down | KeyCode::Left => AppAction::Decrement,
                    _ => AppAction::Nothing,
                },
                _ => AppAction::Nothing,
            };
            self.handle_app_action(&app_action);
        };
        Ok(())
    }

    fn handle_app_action(&mut self, app_action: &AppAction) {
        match app_action {
            AppAction::Close => self.close(),
            AppAction::Decrement => self.decrement(),
            AppAction::Increment => self.increment(),
            AppAction::Nothing => (),
        }
    }
    fn increment(&mut self) {
        let _ = self.counter = self.counter + 1;
    }
    fn decrement(&mut self) {
        let _ = self.counter = self.counter - 1;
    }
    fn close(&mut self) {
        self.exit = true;
    }
}

#[derive(Debug)]
enum AppAction {
    Close,
    Increment,
    Decrement,
    Nothing,
}
