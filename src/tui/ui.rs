use crate::events::app::App;
use figlet_rs::FIGfont;
use r2048::game_logic::State;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    prelude::*,
    style::Color,
    text::{Line, Span},
    widgets::*,
    widgets::{Block, BorderType, Paragraph, Widget},
};

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
                    .title("r2048")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Black))
                    .title_alignment(Alignment::Center);

                let inner_area = game_block.inner(area);
                game_block.render(area, buf);

                let cell_width = inner_area.width / 4;
                let cell_height = inner_area.height / 4;

                for row in 0..4 {
                    for col in 0..4 {
                        let width = if col == 3 {
                            inner_area.width - cell_width * 3
                        } else {
                            cell_width
                        };

                        let height = if row == 3 {
                            inner_area.height - cell_height * 3
                        } else {
                            cell_height
                        };

                        let x = inner_area.x + col * cell_width;
                        let y = inner_area.y + row * cell_height;

                        let cell_area = Rect {
                            x,
                            y,
                            width,
                            height,
                        };

                        let value = self.board.size[row as usize][col as usize];

                        let bg_color = if value != 0 {
                            r2048::decoration::get_background_color(value)
                        } else {
                            Color::Black
                        };

                        Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default().bg(bg_color))
                            .render(cell_area, buf);

                        if value != 0 {
                            let standard_font =
                                FIGfont::from_file("src/fonts/Graceful.flf").unwrap();
                            let figure = standard_font.convert(&value.to_string()).unwrap();

                            let ascii_lines: Vec<Line> = figure
                                .to_string()
                                .lines()
                                .map(|line| {
                                    Line::from(Span::styled(
                                        line.to_string(),
                                        Style::default()
                                            .add_modifier(Modifier::BOLD)
                                            .fg(Color::Black),
                                    ))
                                })
                                .collect();

                            let content_height = ascii_lines.len() as u16;
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
                            padded_content.extend(ascii_lines);
                            while padded_content.len() < inner_height as usize {
                                padded_content.push(Line::from(""));
                            }

                            Paragraph::new(padded_content)
                                .alignment(Alignment::Center)
                                .style(Style::default().bg(bg_color))
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

            State::Won => {
                Clear.render(area, buf);

                let vertical_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(30),
                        Constraint::Percentage(30),
                        Constraint::Percentage(40),
                    ])
                    .split(area);

                let horizontal_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(20),
                        Constraint::Percentage(60),
                        Constraint::Percentage(20),
                    ])
                    .split(vertical_chunks[1]);

                let popup_area = horizontal_chunks[1];

                let popup_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(4), Constraint::Length(3)])
                    .split(popup_area);

                let block = Block::default()
                    .title("You won!")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Green).bg(Color::Black))
                    .border_type(ratatui::widgets::BorderType::Double)
                    .title_alignment(Alignment::Center);

                let score_value = self.board.calculate_score().to_string();
                let lines = vec![
                    Line::styled(
                        "You got 2048 on the board",
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Line::from(vec![
                        Span::styled("SCORE : ", Style::default().fg(Color::White)),
                        Span::styled(
                            score_value,
                            Style::default()
                                .fg(Color::LightMagenta)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]),
                ];

                let paragraph = Paragraph::new(lines)
                    .block(block)
                    .alignment(Alignment::Center)
                    .style(Style::default().bg(Color::Black))
                    .wrap(ratatui::widgets::Wrap { trim: true });

                paragraph.render(popup_chunks[0], buf);

                let buttons = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(popup_chunks[1]);

                let labels = ["One more time?", "Quit"];

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
                        .alignment(Alignment::Center);

                    button.render(buttons[i], buf);
                }
            }
            State::Lost => {
                Clear.render(area, buf);

                let vertical_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(30),
                        Constraint::Percentage(30),
                        Constraint::Percentage(40),
                    ])
                    .split(area);

                let horizontal_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(20),
                        Constraint::Percentage(60),
                        Constraint::Percentage(20),
                    ])
                    .split(vertical_chunks[1]);

                let popup_area = horizontal_chunks[1];

                let popup_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(4), Constraint::Length(3)])
                    .split(popup_area);

                let block = Block::default()
                    .title("Game Over")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Red).bg(Color::Black))
                    .border_type(ratatui::widgets::BorderType::Double)
                    .title_alignment(Alignment::Center);

                let score_value = self.board.calculate_score().to_string();
                let lines = vec![
                    Line::styled(
                        "No more possible moves, you were so close!",
                        Style::default().fg(Color::White),
                    ),
                    Line::from(vec![
                        Span::styled("SCORE : ", Style::default().fg(Color::White)),
                        Span::styled(
                            score_value,
                            Style::default()
                                .fg(Color::LightMagenta)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]),
                ];

                let paragraph = Paragraph::new(lines)
                    .block(block)
                    .alignment(Alignment::Center)
                    .style(Style::default().bg(Color::Black))
                    .wrap(ratatui::widgets::Wrap { trim: true });

                paragraph.render(popup_chunks[0], buf);

                let buttons = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(popup_chunks[1]);

                let labels = ["Maybe try again?", "Quit"];

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
                        .alignment(Alignment::Center);

                    button.render(buttons[i], buf);
                }
            }
        }
    }
}
