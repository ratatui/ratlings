#![allow(unused_imports, unused_variables, dead_code)]

/// Gustau: These are my cooking notes, little rat chef. Give them a little flair.
///
/// Style and wrap Gustau's market note so the cooking advice is easy to read.

use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::Marker,
    widgets::{
        Block, BorderType, Clear, Paragraph, Row, Table, TableState, Wrap,
        calendar::{CalendarEventStore, Monthly},
        canvas::{Canvas, Circle, Line, Rectangle},
    },
};
use ratlings::kitchen::{self, INGREDIENTS, State};
use time::{Date, Month};

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
            for (i, &ingredient) in state.pot.iter().enumerate() {
                let x = 2.75 + (i % 4) as f64 * 1.25;
                let y = 1.5 + (i / 4) as f64;
                match ingredient {
                    0 => ctx.draw(&Circle::new(x, y, 0.4, Color::Red)),
                    1 => ctx.draw(&Rectangle {
                        x: x - 0.45,
                        y: y - 0.2,
                        width: 0.9,
                        height: 0.4,
                        color: Color::LightGreen,
                    }),
                    2 => ctx.draw(&Line::new(
                        x - 0.4,
                        y - 0.4,
                        x + 0.4,
                        y + 0.4,
                        Color::Magenta,
                    )),
                    3 => {
                        let color = Color::Yellow;
                        ctx.draw(&Line::new(x, y + 0.45, x + 0.4, y, color));
                        ctx.draw(&Line::new(x + 0.4, y, x, y - 0.45, color));
                        ctx.draw(&Line::new(x, y - 0.45, x - 0.4, y, color));
                        ctx.draw(&Line::new(x - 0.4, y, x, y + 0.45, color));
                    }
                    _ => {}
                }
            }
        });

    frame.render_widget(canvas, area);
}

fn market_day() -> Date {
    Date::from_calendar_date(2026, Month::November, 28).expect("market day")
}

fn draw_popup(frame: &mut Frame, popup_area: Rect) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(Color::White))
        .title(" Gustau's Market Note ");
    let inner = block.inner(popup_area);
    let popup_layout = Layout::horizontal([Constraint::Percentage(55), Constraint::Percentage(45)]);
    let [calendar_area, notes_area] = inner.layout(&popup_layout);

    frame.render_widget(Clear, popup_area);

    frame.render_widget(block, popup_area);
    let mut events = CalendarEventStore::default();
    events.add(market_day(), Style::new().reversed());

    frame.render_widget(Monthly::new(market_day(), events), calendar_area);
    let notes = Paragraph::new("Buy tomatoes.\nThen let the sauce simmer.")
        .wrap(Wrap { trim: true })
        .style(Style::new().fg(Color::Yellow).bold());

    frame.render_widget(notes, notes_area);
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
    if state.popup {
        let popup_area = Rect::new(
            frame.area().width / 5,
            frame.area().height / 4,
            frame.area().width * 3 / 5,
            frame.area().height / 2,
        );
        draw_popup(frame, popup_area);
    }
}

pub fn update(state: &mut State, key: KeyEvent) {
    match key.code {
        KeyCode::Up => state.selected = state.selected.saturating_sub(1),
        KeyCode::Down => state.selected = (state.selected + 1).min(INGREDIENTS.len() - 1),
        KeyCode::Enter if state.remaining[state.selected] > 0 => {
            state.remaining[state.selected] -= 1;
            state.pot.push(state.selected);
            state.progress = state.pot.len() as f64 / kitchen::TOTAL_INGREDIENTS as f64;
        }
        KeyCode::Char('p') => state.popup = !state.popup,
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
        let mut state = State::default();
        state.popup = true;
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        terminal.draw(|frame| render(frame, &state)).unwrap();
        assert!(
            terminal
                .backend()
                .buffer()
                .content
                .iter()
                .any(|cell| cell.fg == Color::Yellow),
            "style the market note text with the requested emphasis"
        );
    }
}
