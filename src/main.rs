mod events {
    pub mod app;
    pub mod event;
}
mod tui {
    pub mod ui;
}
use events::app::App;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}
