#![allow(unused_imports, unused_variables, dead_code)]

//! Gustau: Little rat chef, let's give our sauce a dancing simmer!
//!
//! Add a red sparkline under the pot to show the sauce simmering over time.

use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::Marker,
    text::Line as TextLine,
    widgets::{
        Block, BorderType, Clear, LineGauge, Paragraph, Row, Sparkline, Table, TableState, Wrap,
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
                let x = 2.75
                    + f64::from(u32::try_from(i % 4).expect("ingredient position fits in u32"))
                        * 1.25;
                let y = 1.5 + f64::from(u32::try_from(i / 4).expect("ingredient row fits in u32"));
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

fn simmer(tick: u64, width: usize) -> Vec<u64> {
    let tick = f64::from(u32::try_from(tick % 628).unwrap_or_default()) / 10.0;
    (0..width)
        .map(|i| {
            let position = f64::from(u32::try_from(i).unwrap_or(u32::MAX)) / 4.0;
            let height = ((position + tick).sin() + 1.0) * 5.0;
            // The sine expression stays between zero and ten.
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let height = height as u64;
            height
        })
        .collect()
}

pub fn render(frame: &mut Frame, state: &State) {
    let kitchen_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(Color::White))
        .title(" Gustau's Kitchen ")
        .title_bottom(TextLine::from("↑↓ Move · Enter Add · p Market").centered());
    let dashboard = kitchen_block.inner(frame.area());

    frame.render_widget(kitchen_block, frame.area());
    let dashboard_layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]);
    let [main_area, gauge_area] = dashboard.layout(&dashboard_layout);
    let kitchen_layout =
        Layout::horizontal([Constraint::Percentage(55), Constraint::Percentage(45)]);
    let [ingredients_area, cooking_area] = main_area.layout(&kitchen_layout);
    let cooking_layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(4)]);
    let [pot_area, spark_area] = cooking_area.layout(&cooking_layout);
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
    let gauge = LineGauge::default()
        .filled_symbol("⣿")
        .unfilled_symbol("⣿")
        .filled_style(Style::new().fg(Color::LightYellow))
        .unfilled_style(Style::new().fg(Color::DarkGray))
        .ratio(state.progress)
        .label(format!(
            "in the pot: {}/{}",
            state.pot.len(),
            kitchen::TOTAL_INGREDIENTS
        ));

    frame.render_widget(
        gauge.block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(Style::new().fg(Color::White))
                .title(" Cooking "),
        ),
        gauge_area,
    );
    let heat_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(Color::White))
        .title(" Heat ");
    let heat_inner = heat_block.inner(spark_area);

    frame.render_widget(heat_block, spark_area);
    let wave = simmer(state.tick, heat_inner.width.into());
    let sparkline = Sparkline::default()
        .data(wave)
        .max(10)
        .style(Color::LightRed);

    frame.render_widget(sparkline, heat_inner);

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
            state.progress = f64::from(u32::try_from(state.pot.len()).unwrap_or_default())
                / f64::from(u32::try_from(kitchen::TOTAL_INGREDIENTS).unwrap_or_default());
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
        assert_ne!(
            simmer(0, 32),
            simmer(8, 32),
            "the simmer data should change when the animation tick advances"
        );
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        terminal
            .draw(|frame| render(frame, &State::default()))
            .unwrap();
        assert!(
            terminal.backend().buffer().content.iter().any(|cell| {
                cell.symbol()
                    .chars()
                    .any(|ch| matches!(ch, '█' | '▇' | '▆' | '▅' | '▄' | '▃' | '▂' | '▁'))
            }),
            "render the cooking heat as a visible sparkline"
        );
    }
}
