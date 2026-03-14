use std::{error::Error, io};

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
                frame.render_widget(Paragraph::new("Hello from the other side"), frame.area());
            })?;
            if crossterm::event::read()?.is_key_press() {
                self.exit = true;
            }
        }
        Ok(())
    }
}
