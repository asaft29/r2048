use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    prelude::*,
    style::{Color, Stylize},
    text::Text,
    widgets::*,
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::app::{App, State};
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
                let game_block = Block::default().title("Game").borders(Borders::ALL);
                let inner_area = game_block.inner(area);
                game_block.render(area, buf);

                let cell_width = inner_area.width / 4;
                let cell_height = inner_area.height / 4;

                for row in 0..4 {
                    for col in 0..4 {
                        let x = inner_area.x + col * cell_width;
                        let y = inner_area.y + row * cell_height;

                        let value = self.board[row as usize][col as usize];
                        let content = if value == 0 {
                            vec![Line::from("")]
                        } else {
                            let big_digit = get_big_digit(value);
                            big_digit
                                .into_iter()
                                .map(|line| {
                                    Line::from(Span::styled(
                                        line,
                                        Style::default()
                                            .add_modifier(Modifier::BOLD)
                                            .fg(get_number_color(value)),
                                    ))
                                })
                                .collect()
                        };

                        let cell_area = Rect {
                            x,
                            y,
                            width: cell_width,
                            height: cell_height,
                        };

                        Paragraph::new(content)
                            .block(Block::default().borders(Borders::ALL))
                            .alignment(Alignment::Center)
                            .render(cell_area, buf);
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

fn get_big_digit(value: u32) -> Vec<String> {
    match value {
        2 => vec![
            " █████ ".to_string(),
            "     █ ".to_string(),
            " █████ ".to_string(),
            " █     ".to_string(),
            " █████ ".to_string(),
        ],
        4 => vec![
            " █   █ ".to_string(),
            " █   █ ".to_string(),
            " █████ ".to_string(),
            "     █ ".to_string(),
            "     █ ".to_string(),
        ],
        8 => vec![
            " █████ ".to_string(),
            " █   █ ".to_string(),
            " █████ ".to_string(),
            " █   █ ".to_string(),
            " █████ ".to_string(),
        ],
        _ => vec![
            value.to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ],
    }
}
