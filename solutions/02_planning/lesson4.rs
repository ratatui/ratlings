// The pack is sorted. Now, when do they go?
//
// Colette has the restaurant covered for one long weekend in October and
// not a day more. Ratatui pins a calendar to the kitchen wall so nobody can
// pretend they forgot.
//
// ──────────────────────────────────────────────────────────────────
//
// `Monthly` draws one month of a calendar. It needs two things: the month
// to show, as a `time::Date`, and something that knows how to style
// individual days. `CalendarEventStore` is the simple version of the
// latter: a map from a date to a `Style`, and any day you `add` to it is
// drawn in that style.

use ratatui::{
    DefaultTerminal, Frame,
    style::Style,
    widgets::{
        Block,
        calendar::{CalendarEventStore, Monthly},
    },
};
use std::io::IsTerminal;
use time::{Date, Month};

/// The three days the restaurant can spare.
const TRIP: [u8; 3] = [10, 11, 12];

fn october(day: u8) -> Date {
    Date::from_calendar_date(2026, Month::October, day).expect("a real day in October")
}

/// Mark each day of the trip so it stands out on the calendar.
fn trip_days() -> CalendarEventStore {
    let mut events = CalendarEventStore::default();
    for day in TRIP {
        events.add(october(day), Style::new().reversed());
    }
    events
}

pub fn render(frame: &mut Frame) {
    let calendar = Monthly::new(october(1), trip_days())
        .show_month_header(Style::new().bold())
        .show_weekdays_header(Style::new().italic())
        .block(Block::bordered().title(" when? "));
    frame.render_widget(calendar, frame.area());
}

// ────────────────────────────────────────────────────────────────────
// Hi Ratatui student!
// Everything below this is just scaffolding.
// It opens the terminal, draws `render` until a key is pressed,
// and checks your work. It is the same in every lesson and you never
// need to edit it. Your work is above this line.

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if ratatui::crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn main() -> std::io::Result<()> {
    if !std::io::stdout().is_terminal() {
        return Ok(());
    }
    ratatui::run(app)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{Terminal, backend::TestBackend, style::Modifier};

    /// What is on screen, one string per row, ignoring style.
    fn rows(terminal: &Terminal<TestBackend>) -> Vec<String> {
        let buffer = terminal.backend().buffer();
        (0..buffer.area.height)
            .map(|y| {
                (0..buffer.area.width)
                    .map(|x| buffer[(x, y)].symbol())
                    .collect()
            })
            .collect()
    }

    #[test]
    fn the_trip_is_circled() {
        let mut terminal = Terminal::new(TestBackend::new(23, 9)).expect("test backend");
        terminal.draw(render).expect("render frame");
        assert_eq!(
            rows(&terminal),
            [
                "┌ when? ──────────────┐",
                "│    October 2026     │",
                "│ Su Mo Tu We Th Fr Sa│",
                "│              1  2  3│",
                "│  4  5  6  7  8  9 10│",
                "│ 11 12 13 14 15 16 17│",
                "│ 18 19 20 21 22 23 24│",
                "│ 25 26 27 28 29 30 31│",
                "└─────────────────────┘",
            ]
        );
        let marked = terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .filter(|cell| cell.modifier.contains(Modifier::REVERSED))
            .count();
        assert_eq!(marked, 6, "three two-digit days should be drawn reversed");
    }
}
