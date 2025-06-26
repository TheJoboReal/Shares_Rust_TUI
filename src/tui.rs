use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
    widgets::{Paragraph, Widget},
};

use crate::share_calc::Person;

#[derive(Debug, Default)]
struct AppState {
    items: Vec<Person>,
}

pub fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        // Rendering
        terminal.draw(render)?;

        // Input handling
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Esc => {
                    break;
                }
                _ => {
                    // Handle other keys
                }
            }
        }
    }
    Ok(())
}

pub fn render(frame: &mut Frame) {
    Paragraph::new("Hello World!").render(frame.area(), frame.buffer_mut());
}
