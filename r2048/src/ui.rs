use ratatui::{
    buffer::Buffer, layout::{Alignment, Constraint, Layout, Rect}, style::{Color, Stylize}, text::Text, widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::app::App;

impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("r2048")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let my_text = Paragraph::new(Text::from("2048 written in Rust!"))
            .block(block.clone())
            .fg(Color::Cyan)
            .bg(Color::Black)
            .alignment(Alignment::Center);

        my_text.render(area, buf);

    }
}
