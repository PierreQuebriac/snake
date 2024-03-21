use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

use ratatui::prelude::*;

use crate::app::{App, Coordinates};

pub fn render_ui(frame: &mut Frame, app: &App) {
    // Layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(20),
            Constraint::Length(3),
        ])
        .split(frame.size());

    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    // Header
    let title = Paragraph::new(Line::styled(
        "Snake",
        Style::default().fg(Color::Green).bold(),
    ))
    .block(Block::default().padding(Padding::uniform(1)));

    let score_display = Paragraph::new(
        Line::from(vec![
            Span::styled("Score: ", Style::default().fg(Color::Green)),
            Span::styled(app.score.to_string(), Style::default().fg(Color::Red)),
        ])
        .right_aligned(),
    )
    .block(Block::default().padding(Padding::uniform(1)));

    frame.render_widget(title, header_chunks[0]);
    frame.render_widget(score_display, header_chunks[1]);

    // Game
    let play_ground = Block::default()
        .borders(Borders::ALL)
        .title("Game")
        .border_type(BorderType::Rounded);

    let play_board: PlayBoard = PlayBoard {
        snake_case: app.snake.clone(),
        fruit_case: Coordinates::new(3, 6),
    };

    // let play_area = constrained_rect(20, 20, chunks[1]);

    frame.render_widget(play_board, chunks[1]);

    frame.render_widget(play_ground, chunks[1]);

    // Footer
    let quit_hint = Span::styled("(q) to quit", Style::default().fg(Color::Red));
    let move_hint = Span::styled("(w/a/s/d) to move", Style::default().fg(Color::Blue));
    let pause_hint = Span::styled("p (pause)", Style::default().fg(Color::Yellow));

    let key_notes_footer = Paragraph::new(
        Line::from(vec![
            quit_hint,
            " | ".into(),
            move_hint,
            " | ".into(),
            pause_hint,
        ])
        .centered(),
    )
    .block(Block::default().borders(Borders::ALL));

    frame.render_widget(key_notes_footer, chunks[2]);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

fn constrained_rect(min_x: u16, min_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Min(min_y),
            Constraint::Min(1),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(1),
            Constraint::Min(min_x),
            Constraint::Min(1),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

pub struct PlayBoard {
    snake_case: Vec<crate::app::Coordinates>,
    fruit_case: crate::app::Coordinates,
}

// '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█',

impl Widget for PlayBoard {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for position in area.positions() {
            if self
                .snake_case
                .contains(&Coordinates::new(position.x, position.y))
            {
                buf.get_mut(position.x, position.y)
                    .set_char('█')
                    .set_fg(Color::Green);
            } else if self.fruit_case == Coordinates::new(position.x, position.y) {
                buf.get_mut(position.x, position.y)
                    .set_bg(Color::Red)
                    .set_style(Style::new().rapid_blink());
            }
        }
    }
}

// impl PlayBoard{
//     pub fn new(x: u16, y: u16) -> Coordinates {
//         Coordinates { x, y }
//     }
// }
