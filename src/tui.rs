use color_eyre::eyre::{Ok, Result};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyEvent},
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    text::Span,
    widgets::{Block, BorderType, List, ListItem, ListState, Padding, Paragraph, Widget},
};

use crate::share_calc::_owed_calc;
use crate::share_calc::Person;

#[derive(Debug, Default)]
pub struct AppState {
    pub items: Vec<Person>,
    pub list_state: ListState,
    pub is_add_new: bool,
    pub is_debt_input: bool,
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
        _owed_calc(&mut app_state.items);
        terminal.draw(|f| render(f, app_state))?;

        if let Event::Key(key) = event::read()? {
            if app_state.is_add_new {
                match handle_add_new(key, app_state) {
                    FormAction::Escape => {
                        app_state.is_add_new = false;
                        app_state.input_value.clear();
                    }

                    FormAction::Submit => {
                        app_state.is_add_new = false;
                        app_state.items.push(Person {
                            _settled: false,
                            _expences: 0.0,
                            _debt: 0.0,
                            _owed: 0.0,
                            _name: app_state.input_value.clone(),
                        });
                        app_state.input_value.clear();
                    }
                    FormAction::None => {}
                }
            } else if app_state.is_debt_input {
                match handle_add_new(key, app_state) {
                    FormAction::Escape => {
                        app_state.is_debt_input = false;
                        app_state.input_value.clear();
                    }
                    FormAction::Submit => {
                        app_state.is_debt_input = false;
                        if let Some(index) = app_state.list_state.selected() {
                            if let Some(item) = app_state.items.get_mut(index) {
                                item._expences = app_state.input_value.parse().unwrap_or(0.0);
                            }
                        }
                        app_state.input_value.clear();
                        _owed_calc(&mut app_state.items);
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
        event::KeyCode::Char('r') => {
            app_state.is_debt_input = true;
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
    let [main_area, bottom_area] = Layout::vertical([
        Constraint::Fill(3),    // Main takes up most space
        Constraint::Length(15), // Bottom takes fixed height
    ])
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
            .render(main_area, frame.buffer_mut());
    } else if app_state.is_debt_input {
        Paragraph::new(app_state.input_value.as_str())
            .block(
                Block::bordered()
                    .fg(Color::Green)
                    .title(" Input Debt ")
                    .padding(Padding::uniform(1))
                    .border_type(BorderType::Rounded),
            )
            .render(main_area, frame.buffer_mut());
    } else {
        // Split main_area further for content and styling
        let [inner_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(main_area);

        Block::bordered()
            .border_type(BorderType::Rounded)
            .fg(Color::Yellow)
            .render(main_area, frame.buffer_mut());

        let list = List::new(app_state.items.iter().map(|x| {
            let status = if x._settled {
                Span::raw(x._name.clone()).crossed_out()
            } else {
                Span::raw(x._name.clone())
            };
            ListItem::from(status)
        }))
        .highlight_symbol(">")
        .highlight_style(Style::default().fg(Color::Green));

        frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);
    }

    // Render bottom panel
    Paragraph::new(" What is owed ")
        .block(
            Block::bordered()
                .fg(Color::Cyan)
                .border_type(BorderType::Rounded),
        )
        .render(bottom_area, frame.buffer_mut());

    let debt_list = List::new(
        app_state
            .items
            .iter()
            .map(|x| Span::raw(format!("{} is owed: {}", x._name, x._owed))),
    );

    let list_height = app_state.items.len().min(bottom_area.height as usize) as u16;
    let list_width = bottom_area.width.min(
        app_state
            .items
            .iter()
            .map(|x| format!("{} ({})", x._name, x._owed).len() as u16)
            .max()
            .unwrap_or(0)
            + 20,
    );
    let horizontal_offset = (bottom_area.width.saturating_sub(list_width)) / 2;
    let vertical_offset = (bottom_area.height.saturating_sub(list_height)) / 2;
    let centered_area = ratatui::layout::Rect {
        x: bottom_area.x + horizontal_offset,
        y: bottom_area.y + vertical_offset,
        width: list_width,
        height: list_height,
    };

    _owed_calc(&mut app_state.items);

    frame.render_stateful_widget(debt_list, centered_area, &mut app_state.list_state);
}
