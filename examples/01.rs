use std::io;

use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, style::Stylize, widgets::Paragraph};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let _app_result = run(terminal);
    ratatui::restore();

    Ok(())
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    let mut c = 0;
    loop {
        c += 1;
        terminal.draw(|frame| {
            let greeting = Paragraph::new(format!("Hello world")).red();
            frame.render_widget(greeting, frame.area());
        })?;
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
