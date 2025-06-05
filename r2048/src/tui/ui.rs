use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    prelude::*,
    style::{Color, Stylize},
    text::Text,
    widgets::*,
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::events::app::{App, State};
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.state {
            State::Menu => {
                let vertical = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(35),
                        Constraint::Length(10),
                        Constraint::Percentage(55),
                    ])
                    .split(area);

                let horizontal = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(20),
                        Constraint::Percentage(60),
                        Constraint::Percentage(20),
                    ])
                    .split(vertical[1]);

                let box_area = horizontal[1];

                let outer = Block::default()
                    .title("r2048")
                    .title_alignment(ratatui::layout::Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded);
                outer.render(box_area, buf);

                let inner_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([
                        Constraint::Length(2),
                        Constraint::Length(1),
                        Constraint::Length(3),
                    ])
                    .split(box_area);

                let title = Paragraph::new("2048 written in Rust!")
                    .style(Style::default().fg(Color::Cyan))
                    .alignment(ratatui::layout::Alignment::Center);
                title.render(inner_chunks[0], buf);

                let button_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(inner_chunks[2]);

                let labels = ["Start", "Quit"];
                for (i, label) in labels.iter().enumerate() {
                    let style = if self.selected_button == i {
                        Style::default()
                            .bg(Color::Green)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };

                    let button = Paragraph::new(*label)
                        .block(Block::default().borders(Borders::ALL))
                        .style(style)
                        .alignment(ratatui::layout::Alignment::Center);

                    button.render(button_chunks[i], buf);
                }
            }

            State::Playing => {
                let game_block = Block::default()
                    .title("Game")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black)); 

                let inner_area = game_block.inner(area);
                game_block.render(area, buf);

                let cell_width = inner_area.width / 4;
                let cell_height = inner_area.height / 4;

                for row in 0..4 {
                    for col in 0..4 {
                        let x = inner_area.x + col * cell_width;
                        let y = inner_area.y + row * cell_height;

                        let value = self.board[row as usize][col as usize];

                        let cell_area = Rect {
                            x,
                            y,
                            width: cell_width,
                            height: cell_height,
                        };

                        Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default().bg(Color::Black))
                            .render(cell_area, buf);

                        if value != 0 {
                            let content = vec![Line::from(Span::styled(
                                value.to_string(),
                                Style::default()
                                    .add_modifier(Modifier::BOLD)
                                    .fg(get_number_color(value))
                                    .bg(Color::Black),
                            ))];

                            let content_height = content.len() as u16;
                            let inner_height = cell_area.height.saturating_sub(2);
                            let top_padding = if inner_height > content_height {
                                (inner_height - content_height) / 2
                            } else {
                                0
                            };

                            let mut padded_content = Vec::new();

                            for _ in 0..top_padding {
                                padded_content.push(Line::from(""));
                            }
                            padded_content.extend(content);
                            while padded_content.len() < inner_height as usize {
                                padded_content.push(Line::from(""));
                            }

                            Paragraph::new(padded_content)
                                .alignment(Alignment::Center)
                                .style(Style::default().bg(Color::Black))
                                .render(
                                    Rect {
                                        x: cell_area.x + 1,
                                        y: cell_area.y + 1,
                                        width: cell_area.width.saturating_sub(2),
                                        height: cell_area.height.saturating_sub(2),
                                    },
                                    buf,
                                );
                        }
                    }
                }
            }

            State::Done => {
                Paragraph::new("Thanks for playing!")
                    .block(Block::default().title("Done").borders(Borders::ALL))
                    .alignment(ratatui::layout::Alignment::Center)
                    .render(area, buf);
            }
        }
    }
}

fn get_number_color(value: u32) -> Color {
    match value {
        2 => Color::Yellow,
        4 => Color::LightYellow,
        8 => Color::LightRed,
        16 => Color::Red,
        32 => Color::LightMagenta,
        64 => Color::Magenta,
        128 => Color::LightCyan,
        256 => Color::Cyan,
        512 => Color::LightBlue,
        1024 => Color::Blue,
        2048 => Color::Green,
        _ => Color::Gray,
    }
}
