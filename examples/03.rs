use std::{collections::HashMap, error::Error, io, rc::Rc};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::{Backend, CrosstermBackend},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
};

// Create a more complex application to use every concept that we have learned so far
// Create a JSON Editor

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install()?;
    // override default terminal config
    enable_raw_mode();
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture);
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;
    // Create the app
    let mut app = App::default();
    let res = app.run_app(&mut terminal);

    // restore terminal to basic config
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    );
    terminal.show_cursor()?;

    // print the json that was created - if we do not have any error and we return `true`
    if let ExitState::Print(value) = app.exit {
        if value {
            app.print_json()?;
        }
    }

    Ok(())
}

// Since we will have mutilple view. Main, Edit and Exiting
// we will define those as an enum and they will be set as our app state

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Screen {
    #[default]
    Main,
    Editing,
    Exiting,
}
// Ratatui does not remember anything about previous state ( value, etc ). Everything should be
// handled manually.

// This will allow us to know if the user is currently on the key part or the value part of the
// screen, This is another state inside one of the state of the `Screen` enum
#[derive(Debug, Default, Clone, Copy)]
enum EditingScreen {
    #[default]
    Key,
    Value,
}

#[derive(Debug, Default)]
enum ExitState {
    Print(bool),
    #[default]
    Nothing,
}

// This represent the whole app state
#[derive(Debug, Default)]
struct App {
    key_input: String,
    value_input: String,
    pairs: HashMap<String, String>,
    screen: Screen,
    editing_screen: Option<EditingScreen>,
    exit: ExitState,
}

impl App {
    pub fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()>
    where
        io::Error: From<B::Error>,
    {
        loop {
            terminal.draw(|frame| {
                let chunks = self.app_layout(frame);

                let title = self.title_section();
                frame.render_widget(title, chunks[0]);

                let mut list_items = Vec::<ListItem>::new();

                for key in self.pairs.keys() {
                    list_items.push(ListItem::new(Line::from(Span::styled(
                        format!("{:25} : {}", key, self.pairs.get(key).unwrap()),
                        Style::default().fg(Color::Yellow),
                    ))));
                }
                let list = List::new(list_items);
                frame.render_widget(list, chunks[1]);

                let current_mode_text = match self.screen {
                    Screen::Main => Span::styled("Normal mode", Style::default().fg(Color::Green)),
                    Screen::Editing => {
                        Span::styled("Editing mode", Style::default().fg(Color::Yellow))
                    }
                    Screen::Exiting => {
                        Span::styled("Exiting", Style::default().fg(Color::LightRed))
                    }
                };
                let divider = Span::styled(" | ", Style::default().fg(Color::White));
                let hint = if let Some(editing_screen) = &self.editing_screen {
                    match editing_screen {
                        EditingScreen::Key => {
                            Span::styled("Editing Json Key", Style::default().fg(Color::Green))
                        }
                        EditingScreen::Value => Span::styled(
                            "Editing Json Value",
                            Style::default().fg(Color::LightGreen),
                        ),
                    }
                } else {
                    Span::styled(
                        "Not  Editing Anything",
                        Style::default().fg(Color::DarkGray),
                    )
                };
                let current_navigation_text = vec![current_mode_text, divider, hint];
                let mode_footer = Paragraph::new(Line::from(current_navigation_text))
                    .block(Block::default().borders(Borders::ALL));
                let current_keys_hint = {
                    match self.screen {
                        Screen::Main => Span::styled(
                            "(q) to quit / (e) to make new pair",
                            Style::default().fg(Color::Red),
                        ),
                        Screen::Editing => Span::styled(
                            "(ESC) to cancel/(Tab) to switch boxes/enter to complete",
                            Style::default().fg(Color::Red),
                        ),
                        Screen::Exiting => Span::styled(
                            "(q) to quit / (e) to make new pair",
                            Style::default().fg(Color::Red),
                        ),
                    }
                };

                let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
                    .block(Block::default().borders(Borders::ALL));
                let footer_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(chunks[2]);
                frame.render_widget(mode_footer, footer_chunks[0]);
                frame.render_widget(key_notes_footer, footer_chunks[1]);
            })?;

            // handle all the event, and will modify the state of our app
            self.handle_events();

            // check if we need to exit the app
            if let ExitState::Print(_) = self.exit {
                return Ok(());
            }
        }
    }

    // This method is mandatory because render does not want a mutable self, but the mut is needed
    // to change the state of our app -> used in our handle_events function
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    // This return nothing, but maybe an error
    // this method is there to mutate the app state ( should consider a builder - need to read best
    // practice about this )
    fn handle_events(&mut self) -> io::Result<()> {
        let event = crossterm::event::read()?;
        if let Event::Key(ev) = event {
            match self.screen {
                Screen::Main => {
                    let res = self.main_screen_event(&ev);
                    self.screen = res;
                    if res == Screen::Editing {
                        self.editing_screen = Some(EditingScreen::Key);
                    }
                }
                Screen::Editing => self.editing_screen_event(&ev),
                Screen::Exiting => self.exit = self.exiting_sreen_event(&ev),
            };
        };
        Ok(())
    }
    // this should be called when we are in the exit state of our app / screen
    fn exiting_sreen_event(&self, event: &KeyEvent) -> ExitState {
        match event.kind {
            KeyEventKind::Press => match event.code {
                KeyCode::Char('y') => ExitState::Print(true),
                KeyCode::Char('n') => ExitState::Print(false),
                _ => ExitState::Nothing,
            },
            _ => ExitState::Nothing,
        }
    }
    fn main_screen_event(&self, event: &KeyEvent) -> Screen {
        match event.kind {
            KeyEventKind::Press => match event.code {
                KeyCode::Char('e') => Screen::Editing,
                KeyCode::Char('q') => Screen::Exiting,
                _ => self.screen,
            },
            _ => self.screen,
        }
    }
    fn editing_screen_event(&mut self, event: &KeyEvent) {
        match event.kind {
            KeyEventKind::Press => match event.code {
                KeyCode::Enter => {
                    if let Some(editing_screen) = self.editing_screen {
                        match editing_screen {
                            EditingScreen::Key => {
                                self.editing_screen = Some(EditingScreen::Value);
                            }
                            EditingScreen::Value => {
                                self.save_key_value();
                                self.screen = Screen::Main;
                            }
                        }
                    }
                }
                KeyCode::Backspace => {
                    if let Some(editing_screen) = self.editing_screen {
                        match editing_screen {
                            EditingScreen::Key => {
                                self.key_input.pop();
                            }
                            EditingScreen::Value => {
                                self.value_input.pop();
                            }
                        }
                    }
                }
                KeyCode::Tab => {
                    self.toggle_editing();
                }
                KeyCode::Char(value) => {
                    if let Some(editing_screen) = self.editing_screen {
                        match editing_screen {
                            EditingScreen::Key => self.key_input.push(value),
                            EditingScreen::Value => self.value_input.push(value),
                        }
                    }
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
    pub fn save_key_value(&mut self) {
        // Insert input value in our hashmap
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());
        // reset all the inputs
        self.value_input = String::new();
        self.key_input = String::new();
        // reset editing screen state
        self.editing_screen = None;
    }

    pub fn toggle_editing(&mut self) {
        match &self.editing_screen {
            None => self.editing_screen = Some(EditingScreen::Key),
            Some(screen) => match screen {
                EditingScreen::Key => self.editing_screen = Some(EditingScreen::Value),
                EditingScreen::Value => self.editing_screen = Some(EditingScreen::Key),
            },
        };
    }

    pub fn print_json(&mut self) -> serde_json::Result<()> {
        let value = serde_json::to_string(&self.pairs)?;
        println!("{value}");
        Ok(())
    }

    fn title_section(&self) -> Paragraph<'_> {
        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        Paragraph::new(Text::styled(
            "Create Json",
            Style::default().fg(Color::Green),
        ))
        .block(title_block)
    }

    fn app_layout(&self, frame: &mut Frame) -> Rc<[Rect]> {
        Layout::default()
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(frame.area())
    }
}

// Here I want to use the widget trait so that I can render the whole app
impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let title = Paragraph::new(Text::styled(
            "Create Json",
            Style::default().fg(Color::Green),
        ))
        .block(title_block);

        title.render(area, buf);
    }
}
