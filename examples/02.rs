use std::{error::Error, io};

use ratatui::{DefaultTerminal, widgets::Paragraph};

// Ratatui first interactive application

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    ratatui::run(app);
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let hello = format!("Hello world");
            let para = Paragraph::new(hello);
            frame.render_widget(para, frame.area());
        })?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}
