use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyEvent},
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, List, ListItem, ListState, Paragraph, Widget, BorderType, Padding},
};

use crate::share_calc::Person;

#[derive(Debug, Default)]
pub struct AppState {
    pub items: Vec<Person>,
    pub list_state: ListState,
    pub is_add_new: bool,
    pub input_value: String,
}

enum FormAction {
    None,
    Submit,
    Escape,
}

pub fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        // Rendering
        terminal.draw(|f| render(f, app_state))?;

        if let Event::Key(key) = event::read()? {
            if app_state.is_add_new {
                match handle_add_new(key, app_state) {
                    FormAction::Escape => {
                        app_state.is_add_new = false;
                        app_state.input_value.clear();},

                    FormAction::Submit => {
                        app_state.is_add_new = false;
                        app_state.items.push(Person { 
                            _settled: false,
                            _expences: 0.0,
                            _debt: 0.0,
                            _owed: 0.0,
                            _name: app_state.input_value.clone()
                        });
                        app_state.input_value.clear();
                    }
                    FormAction::None => {}
                }
            } else if handle_key(key, app_state) {
                break;
            }
        }

        // Input handling
    }
    Ok(())
}

fn handle_add_new(key: KeyEvent, app_state: &mut AppState) -> FormAction {
    match key.code {
        event::KeyCode::Char(c) => {
            app_state.input_value.push(c);
        }
        event::KeyCode::Backspace => {
            app_state.input_value.pop();
        }
        event::KeyCode::Esc => {
            return FormAction::Escape;
        }
        event::KeyCode::Enter => {
            return FormAction::Submit;
        }
        _ => {
            // Handle other keys for adding new item
        }
    }
    FormAction::None
}

fn handle_key(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        event::KeyCode::Esc => {
            return true;
        }
        event::KeyCode::Enter => {
            if let Some(index) = app_state.list_state.selected() {
                if let Some(item) = app_state.items.get_mut(index) {
                    item._settled = !item._settled;
                }
            }
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
        event::KeyCode::Char('A') => {
            app_state.is_add_new = true;
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
    if app_state.is_add_new {
        Paragraph::new(app_state.input_value.as_str())
            .block(
                Block::bordered()
                .fg(Color::Green)
                .title(" Input Name ")
                .padding(Padding::uniform(1))
                .border_type(BorderType::Rounded),
                )
                .render(border_area, frame.buffer_mut());
    } else {

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
    }
}
