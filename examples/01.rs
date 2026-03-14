use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::Layout,
    style::Stylize,
    widgets::{Block, Paragraph, Tabs},
};

use ratatui_core::layout::Constraint::{Fill, Length, Min};

// Ratatui introduction
// Create an inifite loop to allow us to render the TUI and create interaction
// Pressing `Q` will allow us to kill the app.

// Introduction to layout, style, events

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
            draw(frame);
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

fn draw(frame: &mut Frame) {
    let vertical = Layout::vertical([Length(4), Min(0), Length(1)]);

    let [top_area, main_area, status_area] = vertical.areas(frame.area());

    let horizontal = Layout::horizontal([Fill(1); 2]);
    let [left_area, right_area] = horizontal.areas(main_area);

    let top_layout = Layout::vertical([Length(3), Length(1)]);
    let [title_area, tab_area] = top_layout.areas(top_area);

    let left_panel = Block::bordered().title("Left");
    let text_inside_left_panel = Paragraph::new(format!("Inside left panel"))
        .red()
        .on_green()
        .bold()
        .block(left_panel);
    let tabs = Tabs::default().titles(vec!["Tab 1", "Tab 2"]);

    frame.render_widget(Block::bordered().title("Title Bar"), title_area);
    frame.render_widget(tabs, tab_area);
    frame.render_widget(Block::bordered().title("Status Bar"), status_area);
    frame.render_widget(text_inside_left_panel, left_area);
    frame.render_widget(Block::bordered().title("Right"), right_area);
}
