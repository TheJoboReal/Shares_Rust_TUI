mod share_calc;
mod tui;
use tui::AppState;
use tui::run;

fn main() -> Result<(), color_eyre::Report> {
    let mut state = AppState::default();
    color_eyre::install()?;

    state.items.push(share_calc::Person {
        _name: String::from("Kasper"),
        _expences: 100.0,
        _debt: 0.0,
        _owed: 0.0,
        _settled: false,
    });

    state.items.push(share_calc::Person {
        _name: String::from("Runa"),
        _expences: 100.0,
        _debt: 0.0,
        _owed: 0.0,
        _settled: false,
    });

    state.items.push(share_calc::Person {
        _name: String::from("Camilla"),
        _expences: 100.0,
        _debt: 0.0,
        _owed: 0.0,
        _settled: false,
    });

    let terminal = ratatui::init();
    let result = run(terminal, &mut state);

    ratatui::restore();
    result
}
