use crate::board::State;
use crate::events::app::App;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    prelude::*,
    style::Color,
    text::{Line, Span},
    widgets::*,
    widgets::{Block, BorderType, Paragraph, Widget},
};
use tui_big_text::{BigText, PixelSize};

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
                        Constraint::Min(1),
                        Constraint::Length(3),
                    ])
                    .split(box_area);

                let title = Paragraph::new("2048 written in Rust!")
                    .style(Style::default().fg(Color::Cyan))
                    .alignment(ratatui::layout::Alignment::Center);
                title.render(inner_chunks[0], buf);

                let highest_score_paragraph = Paragraph::new(format!(
                    "Highest Score: {}",
                    self.board.db.borrow().get_score().unwrap()
                ))
                .style(Style::default().fg(Color::Yellow))
                .alignment(ratatui::layout::Alignment::Center);
                highest_score_paragraph.render(inner_chunks[1], buf);

                let button_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(inner_chunks[3]);

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

                let grid_width = cell_width * 4;
                let grid_height = cell_height * 4;
                let grid_start_x = inner_area.x + (inner_area.width - grid_width) / 2;
                let grid_start_y = inner_area.y + (inner_area.height - grid_height) / 2;

                for row in 0..4 {
                    for col in 0..4 {
                        let cell_area = Rect {
                            x: grid_start_x + col * cell_width,
                            y: grid_start_y + row * cell_height,
                            width: cell_width,
                            height: cell_height,
                        };

                        let value = self.board.size[row as usize][col as usize];
                        let bg_color = if value != 0 {
                            crate::decoration::get_background_color(value)
                        } else {
                            Color::Black
                        };

                        Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default().bg(bg_color))
                            .render(cell_area, buf);

                        if value != 0 {
                            let pixel_size = if cell_area.width < 8 || cell_area.height < 4 {
                                PixelSize::Quadrant
                            } else {
                                PixelSize::Full
                            };

                            let big_text = BigText::builder()
                                .centered()
                                .pixel_size(pixel_size)
                                .style(match value {
                                    2 | 4 => Style::new().blue(),
                                    _ => Style::new().black().bold(),
                                })
                                .lines(vec![Line::from(value.to_string())])
                                .build();

                            let text_area = Rect {
                                x: cell_area.x + 1,
                                y: cell_area.y + 1,
                                width: cell_area.width.saturating_sub(2),
                                height: cell_area.height.saturating_sub(2),
                            };

                            big_text.render(text_area, buf);
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

                let score_value = self.board.calculate_score();
                self.board
                    .db
                    .borrow_mut()
                    .update_score(score_value)
                    .unwrap();

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
                            score_value.to_string(),
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

                let labels = ["One more time?", "Main Menu"];

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

                let score_value = self.board.calculate_score();
                self.board
                    .db
                    .borrow_mut()
                    .update_score(score_value)
                    .unwrap();

                let lines = vec![
                    Line::styled(
                        "No more possible moves, you were so close!",
                        Style::default().fg(Color::White),
                    ),
                    Line::from(vec![
                        Span::styled("SCORE : ", Style::default().fg(Color::White)),
                        Span::styled(
                            score_value.to_string(),
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

                let labels = ["Maybe try again?", "Main Menu"];

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
