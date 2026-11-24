#![allow(unused_imports, unused_variables, dead_code)]

//! Gustau: My rat fellow, let's give this kitchen a proper frame.
//!
//! Give the kitchen a rounded frame and keep the chef badge inside it.

use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
};
use ratlings::kitchen::{self, INGREDIENTS, State};

const CHEF_COLOR: Color = Color::Green;

// TODO: Create a rounded Block titled " Gustau's Kitchen " and render it over the frame.
// Docs: https://docs.rs/ratatui/latest/ratatui/widgets/struct.Block.html
// Hint: use `.border_type(BorderType::Rounded)` to round its corners.

pub fn render(frame: &mut Frame, _state: &State) {
    let block = Block::new();
    let area = block.inner(frame.area());

    frame.render_widget(block, frame.area());

    frame.render_widget(
        Paragraph::new("Little rat chef").style(Style::new().fg(CHEF_COLOR)),
        area,
    );
}

pub fn update(_state: &mut State, _key: KeyEvent) {}

fn main() -> std::io::Result<()> {
    kitchen::run(render, update)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{Terminal, backend::TestBackend, buffer::Cell};

    fn screen(terminal: &Terminal<TestBackend>) -> Vec<String> {
        let buffer = terminal.backend().buffer();
        let width = buffer.area().width as usize;
        buffer
            .content()
            .chunks(width)
            .map(|row| row.iter().map(Cell::symbol).collect())
            .collect()
    }
    #[test]
    fn exercise_goal_is_reached() {
        let mut terminal = Terminal::new(TestBackend::new(24, 4)).unwrap();
        terminal
            .draw(|frame| render(frame, &State::default()))
            .unwrap();
        assert_eq!(
            screen(&terminal),
            [
                "╭ Gustau's Kitchen ────╮",
                "│Little rat chef       │",
                "│                      │",
                "╰──────────────────────╯",
            ],
            "draw a rounded kitchen block with the chef badge inside it"
        );
    }
}
