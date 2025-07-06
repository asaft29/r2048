mod events;
mod tui;
use events::app::App;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let _ = App::new().run(terminal)?;
    ratatui::restore();
    Ok(())
}
