use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, List, ListItem, ListState, Paragraph, Widget},
};

use crate::share_calc::Person;

#[derive(Debug, Default)]
pub struct AppState {
    pub items: Vec<Person>,
    pub list_state: ListState,
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
                event::KeyCode::Char('k') => {
                    app_state.list_state.select_previous();
                }
                event::KeyCode::Char('j') => {
                    app_state.list_state.select_next();
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

    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(border_area);

    Block::bordered()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .fg(Color::Yellow)
        .render(border_area, frame.buffer_mut());

    let list = List::new(
        app_state
            .items
            .iter()
            .map(|x| ListItem::from(x._name.clone())),
    )
    .highlight_symbol(">")
    .highlight_style(Style::default().fg(Color::Green));

    frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);

    //Paragraph::new("Hello World!").render(frame.area(), frame.buffer_mut());
}
