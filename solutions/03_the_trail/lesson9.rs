// Mile five. The footbridge is behind them, the meadow is behind them, and
// the trail has started doing the thing Rémy's chart warned about.
//
// "How much further?" asks Emile, for the fourth time. Ratatui decides the
// answer should be on the screen where everyone can see it.
//
// ──────────────────────────────────────────────────────────────────
//
// `LineGauge` is a `Gauge` that takes up a single row. Instead of a filled
// block it draws a line, with a different symbol or style for the part that
// is done and the part that is not. Same `ratio`, same `label`, a lot less
// room.

use ratatui::{
    DefaultTerminal, Frame,
    widgets::{Block, LineGauge},
};
use std::io::IsTerminal;

const TRAIL_MILES: f64 = 8.0;
const HIKED_MILES: f64 = 5.0;

/// How far along the trail they are, and how far in miles.
fn progress() -> LineGauge<'static> {
    LineGauge::default()
        .filled_symbol("━")
        .unfilled_symbol("╌")
        .ratio(HIKED_MILES / TRAIL_MILES)
        .label(format!("{HIKED_MILES} of {TRAIL_MILES} mi"))
}

pub fn render(frame: &mut Frame) {
    let trail = progress().block(Block::bordered().title(" to camp "));
    frame.render_widget(trail, frame.area());
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
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn five_of_eight_miles() {
        let mut terminal = Terminal::new(TestBackend::new(30, 3)).expect("test backend");
        terminal.draw(render).expect("render frame");
        terminal.backend().assert_buffer_lines([
            "┌ to camp ───────────────────┐",
            "│5 of 8 mi ━━━━━━━━━━━╌╌╌╌╌╌╌│",
            "└────────────────────────────┘",
        ]);
    }
}
