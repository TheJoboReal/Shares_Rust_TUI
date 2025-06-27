use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{
        event::{self, Event},
        style::Color,
    },
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::{Block, Paragraph, Widget},
};

use crate::share_calc::Person;

#[derive(Debug, Default)]
pub struct AppState {
    items: Vec<Person>,
}

pub fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        // Rendering
        terminal.draw(|f| render(f, app_state))?;

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

pub fn render(frame: &mut Frame, app_state: &mut AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());

    Block::bordered()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .fg(Color::Yellow)
        .render(border_area, frame.buffer_mut());

    Paragraph::new("Hello World!").render(frame.area(), frame.buffer_mut());
}
