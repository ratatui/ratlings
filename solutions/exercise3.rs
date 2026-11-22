#![allow(unused_imports, unused_variables, dead_code)]

//! Gustau: My little rat chef, use the arrow keys to choose what goes into the pot.
//!
//! Track the highlighted ingredient and move through the table with the arrow keys.

use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph, Row, Table, TableState},
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
    let mut selection = TableState::default().with_selected(Some(state.selected));

    frame.render_stateful_widget(table, area, &mut selection);
}

pub fn update(state: &mut State, key: KeyEvent) {
    match key.code {
        KeyCode::Up => state.selected = state.selected.saturating_sub(1),
        KeyCode::Down => state.selected = (state.selected + 1).min(INGREDIENTS.len() - 1),
        _ => {}
    }
}

fn main() -> std::io::Result<()> {
    kitchen::run(render, update)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{
        Terminal,
        backend::TestBackend,
        crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    };

    #[test]
    fn exercise_goal_is_reached() {
        let mut state = State::default();
        update(&mut state, KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        assert_eq!(
            state.selected, 1,
            "move the selection down when Down is pressed"
        );
        let mut terminal = Terminal::new(TestBackend::new(40, 8)).unwrap();
        terminal.draw(|frame| render(frame, &state)).unwrap();
        let buffer = terminal.backend().buffer();
        assert!(
            buffer.content.iter().any(|cell| cell.symbol() == "›"),
            "show the selection marker beside the selected ingredient"
        );
    }
}
