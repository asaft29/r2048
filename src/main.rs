use r2048::events::app::App;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    App::new().run(terminal)?;
    ratatui::restore();
    Ok(())
}
