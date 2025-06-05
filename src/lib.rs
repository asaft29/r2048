use ratatui::style::Color;

pub mod decoration {
    use super::*;

    pub fn get_background_color(value: u32) -> Color {
        match value {
            2 => Color::Rgb(238, 228, 218),
            4 => Color::Rgb(237, 224, 200),
            8 => Color::Rgb(242, 177, 121),
            16 => Color::Rgb(245, 149, 99),
            32 => Color::Rgb(246, 124, 95),
            64 => Color::Rgb(246, 94, 59),
            128 => Color::Rgb(237, 207, 114),
            256 => Color::Rgb(237, 204, 97),
            512 => Color::Rgb(237, 200, 80),
            1024 => Color::Rgb(237, 197, 63),
            2048 => Color::Rgb(237, 194, 46),
            _ => Color::Black,
        }
    }
}
