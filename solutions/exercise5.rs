#![allow(unused_imports, unused_variables, dead_code)]

/// Gustau: The pot is ready, my rat fellow. Let's draw its empty shape beside our list.
///
/// Draw an empty rectangular pot beside the ingredient list before we start cooking.

use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    symbols::Marker,
    widgets::{
        Block, BorderType, Paragraph, Row, Table, TableState,
        canvas::{Canvas, Circle, Rectangle},
    },
};
use ratlings::kitchen::{self, INGREDIENTS, State};

const CHEF_COLOR: Color = Color::Green;

fn draw_pot(frame: &mut Frame, area: Rect, state: &State) {
    let canvas = Canvas::default()
        .marker(Marker::Braille)
        .x_bounds([0.0, 10.0])
        .y_bounds([0.0, 6.0])
        .paint(|ctx| {
            ctx.draw(&Rectangle {
                x: 1.25,
                y: 0.75,
                width: 7.5,
                height: 4.5,
                color: Color::Gray,
            });
        });

    frame.render_widget(canvas, area);
}

pub fn render(frame: &mut Frame, state: &State) {
    let kitchen_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(Color::White))
        .title(" Gustau's Kitchen ");
    let dashboard = kitchen_block.inner(frame.area());

    frame.render_widget(kitchen_block, frame.area());
    let kitchen_layout =
        Layout::horizontal([Constraint::Percentage(55), Constraint::Percentage(45)]);
    let [ingredients_area, pot_area] = dashboard.layout(&kitchen_layout);
    let ingredients_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(Color::White))
        .title(" Ingredients ");
    let ingredients_inner = ingredients_block.inner(ingredients_area);

    frame.render_widget(ingredients_block, ingredients_area);
    let rows = INGREDIENTS
        .iter()
        .enumerate()
        .map(|(i, (name, _))| Row::new([(*name).to_string(), state.remaining[i].to_string()]));
    let table = Table::new(rows, [Constraint::Length(16), Constraint::Length(6)])
        .header(Row::new(["ingredient", "qty"]))
        .row_highlight_style(Style::new().reversed())
        .highlight_symbol("› ");
    let mut selection = TableState::default().with_selected(Some(state.selected));

    frame.render_stateful_widget(table, ingredients_inner, &mut selection);
    let pot_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(Color::White))
        .title(" Pot ");
    let pot_inner = pot_block.inner(pot_area);

    frame.render_widget(pot_block, pot_area);
    draw_pot(frame, pot_inner, state);
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
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn exercise_goal_is_reached() {
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        terminal
            .draw(|frame| render(frame, &State::default()))
            .unwrap();
        assert!(
            terminal.backend().buffer().content.iter().any(|cell| {
                cell.symbol()
                    .chars()
                    .any(|ch| ('\u{2800}'..='\u{28ff}').contains(&ch))
            }),
            "draw the pot on the canvas with a rectangle"
        );
    }
}
