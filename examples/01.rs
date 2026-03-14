use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
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
            let greeting = Paragraph::new(format!("Hello world"))
                .red()
                .on_green()
                .bold();
            frame.render_widget(greeting, frame.area());
        })?;
        if let Ok(action) = handle_events() {
            if action == Actionlist::Quit {
                return Ok(());
            }
        }
    }
}

fn handle_events() -> std::io::Result<Actionlist> {
    let action = match event::read()? {
        Event::Key(key) => handle_key_event(&key),
        _ => Actionlist::Nothing,
    };

    Ok(action)
}

fn handle_key_event(event: &KeyEvent) -> Actionlist {
    match event.kind {
        KeyEventKind::Press => handle_key_code(&event.code),
        _ => Actionlist::Nothing,
    }
}

fn handle_key_code(key_code: &KeyCode) -> Actionlist {
    match key_code {
        KeyCode::Char('q') => Actionlist::Quit,
        _ => Actionlist::Nothing,
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Actionlist {
    Quit,
    Nothing,
}

impl Actionlist {
    pub fn handle(&self) {
        match self {
            Self::Quit => (),
            Self::Nothing => (),
        }
    }
}
