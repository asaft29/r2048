pub mod events {
    pub mod app;
    pub mod event;
}
pub mod tui {
    pub mod ui;
}

pub mod board;
pub mod db;

pub mod decoration {

    use ratatui::style::Color;
    
    #[inline(always)]
    pub fn get_background_color(value: u32) -> Color {
        match value {
            2 => Color::Rgb(238, 228, 218),   // #eee4da
            4 => Color::Rgb(237, 224, 180),   // #ede0b4
            8 => Color::Rgb(242, 177, 121),   // #f2b179
            16 => Color::Rgb(245, 149, 99),   // #f59563
            32 => Color::Rgb(246, 124, 95),   // #f67c5f
            64 => Color::Rgb(246, 94, 59),    // #f65e3b
            128 => Color::Rgb(237, 207, 114), // #edcf72
            256 => Color::Rgb(237, 204, 65),  // #edcc41
            512 => Color::Rgb(231, 170, 40),  // #e7aa28
            1024 => Color::Rgb(214, 166, 0),  // #d6a600
            2048 => Color::Rgb(215, 149, 43), // #d7952b
            _ => Color::Black,
        }
    }
}
