use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyEvent},
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, List, ListItem, ListState, Paragraph, Widget},
};

use crate::share_calc::Person;

#[derive(Debug, Default)]
pub struct AppState {
    pub items: Vec<Person>,
    pub list_state: ListState,
    pub is_add_new: bool,
}

pub fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        // Rendering
        terminal.draw(|f| render(f, app_state))?;

        if let Event::Key(key) = event::read()? {
            if handle_key(key, app_state) {
                break;
            }
        }

        // Input handling
    }
    Ok(())
}

fn handle_key(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        event::KeyCode::Esc => {
            return true;
        }
        event::KeyCode::Char('k') => {
            app_state.list_state.select_previous();
        }
        event::KeyCode::Char('j') => {
            app_state.list_state.select_next();
        }
        event::KeyCode::Char('D') => {
            if let Some(index) = app_state.list_state.selected() {
                app_state.items.remove(index);
            }
        }
        _ => {
            // Handle other keys
        }
    }
    false
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
