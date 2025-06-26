use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal,
    crossterm::{
        event::{self, Event},
        terminal,
    },
};

pub fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        // Rendering

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
