use std::{error::Error, io};

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

// Ratatui first interactive application

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = color_eyre::install();
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
                self.draw(frame);
            })?;
            let _ = self.handle_key_events();
        }
        Ok(())
    }
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let actions = Line::from(vec![
            " Decrement ".into(),
            " <Left> / <Down>".red().bold(),
            " Increment ".into(),
            " <Right> / <Up> ".blue().bold(),
            " Quit ".into(),
            " <Q> ".bold(),
        ]);

        let counter = Text::from(vec![Line::from(vec![
            "Counter: ".into(),
            self.counter.to_string().yellow().bold(),
        ])]);

        let block = Block::bordered()
            .border_set(border::THICK)
            .title_bottom(actions.centered());

        Paragraph::new(counter)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

#[derive(Debug)]
enum AppAction {
    Close,
    Increment,
    Decrement,
    Nothing,
}
