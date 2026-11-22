#![allow(unused_imports, unused_variables, dead_code)]

/// Gustau: Little rat chef, first we need a tidy list of our ingredients.
///
/// Render the ingredient table so we can check each ingredient and its stock.

use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph, Row, Table},
};
use ratlings::kitchen::{self, INGREDIENTS, State};

const CHEF_COLOR: Color = Color::Green;

pub fn render(frame: &mut Frame, state: &State) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(Color::White))
        .title(" Gustau's Kitchen ");
    let area = block.inner(frame.area());

    frame.render_widget(block, frame.area());
    let rows = INGREDIENTS
        .iter()
        .enumerate()
        .map(|(i, (name, _))| Row::new([(*name).to_string(), state.remaining[i].to_string()]));
    let table = Table::new(rows, [Constraint::Length(16), Constraint::Length(6)])
        .header(Row::new(["ingredient", "qty"]))
        .row_highlight_style(Style::new().reversed())
        .highlight_symbol("› ");

    frame.render_widget(table, area);
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
            terminal.backend().to_string().contains("tomatoes"),
            "render the ingredient table with its rows"
        );
    }
}
