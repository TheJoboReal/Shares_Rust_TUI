mod tui;
use tui::run;

fn main() -> Result<()> {
    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal);

    ratatui::restore();

    Ok(())
}
