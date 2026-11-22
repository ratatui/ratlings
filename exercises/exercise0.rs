#![allow(unused_imports, unused_variables, dead_code)]

//! Gustau: Welcome, little rat chef! Choose a color for your chef badge and make it yours.
//!
//! Choose a badge color so Gustau knows which little chef is in the kitchen.

use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Paragraph},
};
use ratlings::kitchen::{self, INGREDIENTS, State};

// TODO: Set CHEF_COLOR to a Ratatui Color you like.
// Docs: https://docs.rs/ratatui/latest/ratatui/style/enum.Color.html
//

const CHEF_COLOR: Color = Color::White;

pub fn render(frame: &mut Frame, _state: &State) {
    frame.render_widget(
        Paragraph::new("Little rat chef").style(Style::new().fg(CHEF_COLOR)),
        frame.area(),
    );
}

pub fn update(_state: &mut State, _key: KeyEvent) {}

fn main() -> std::io::Result<()> {
    kitchen::run(render, update)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Color;

    #[test]
    fn exercise_goal_is_reached() {
        assert_ne!(
            CHEF_COLOR,
            Color::White,
            "choose a badge color other than the default white"
        );
    }
}
