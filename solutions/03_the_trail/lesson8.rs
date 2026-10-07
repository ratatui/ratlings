// The trailhead sign says eight miles to camp. Two miles in, the sun is
// high, the pack is heavy, and Ratatui has been drinking a lot more water
// than he planned to.
//
// Someone should keep an eye on the bottle.
//
// ──────────────────────────────────────────────────────────────────
//
// A `Gauge` is a filled bar that shows how much of something there is. You
// tell it how full to be, either as a `percent` or a `ratio` between 0.0
// and 1.0, and it draws that fraction of its width filled in. The `label`
// sits in the middle and can say whatever is actually useful.

use ratatui::{
    DefaultTerminal, Frame,
    widgets::{Block, Gauge},
};
use std::io::IsTerminal;

const BOTTLE_ML: u16 = 1000;
const WATER_ML: u16 = 350;

/// How much is left, as a bar filled to match and a label that says how much.
fn water_left() -> Gauge<'static> {
    Gauge::default()
        .ratio(f64::from(WATER_ML) / f64::from(BOTTLE_ML))
        .label(format!("{WATER_ML} ml"))
}

pub fn render(frame: &mut Frame) {
    let bottle = water_left().block(Block::bordered().title(" bottle "));
    frame.render_widget(bottle, frame.area());
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
    fn the_bottle_is_a_third_full() {
        let mut terminal = Terminal::new(TestBackend::new(24, 3)).expect("test backend");
        terminal.draw(render).expect("render frame");
        terminal.backend().assert_buffer_lines([
            "┌ bottle ──────────────┐",
            "│████████350 ml        │",
            "└──────────────────────┘",
        ]);
    }
}
