mod share_calc;
mod tui;
use tui::AppState;
use tui::run;

fn main() -> Result<(), color_eyre::Report> {
    let mut state = AppState::default();
    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal, &mut state);

    ratatui::restore();
    result
}
