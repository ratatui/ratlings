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

pub fn render(frame: &mut Frame, _state: &State) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(" Gustau's Kitchen ");
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
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn exercise_goal_is_reached() {
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        terminal
            .draw(|frame| render(frame, &State::default()))
            .unwrap();
        assert!(
            terminal.backend().to_string().contains("Gustau's Kitchen"),
            "render a rounded kitchen block titled Gustau's Kitchen"
        );
    }
}
