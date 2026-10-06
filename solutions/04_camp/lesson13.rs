// Camp. A ring of stones in a stand of pines, exactly as the book promised.
// The matches were in Ratatui's pocket the whole time.
//
// The fire catches, roars, and over the evening settles into embers. Emile
// takes it upon himself to report the heat every half hour, whether anyone
// asked or not. There has to be a quieter way to show it.
//
// ──────────────────────────────────────────────────────────────────
//
// A `Sparkline` is a chart with no axes and no labels, just a series of
// values drawn as bars one column wide. It is the smallest honest way to
// show how a number has changed. Hand it the data and, if the natural top
// of the scale is not simply the biggest value, a `max`.

use ratatui::{
    DefaultTerminal, Frame,
    widgets::{Block, Sparkline},
};
use std::io::IsTerminal;

/// Emile's readings, every half hour from lighting the fire to turning in.
const EMBERS: [u64; 16] = [3, 7, 9, 9, 8, 8, 7, 6, 6, 5, 4, 4, 3, 2, 2, 1];

/// The whole evening at a glance.
fn fire() -> Sparkline<'static> {
    Sparkline::default().data(EMBERS).max(9)
}

pub fn render(frame: &mut Frame) {
    let ring = fire().block(Block::bordered().title(" fire "));
    frame.render_widget(ring, frame.area());
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
    fn the_fire_dies_down() {
        let mut terminal = Terminal::new(TestBackend::new(18, 4)).expect("test backend");
        terminal.draw(render).expect("render frame");
        terminal.backend().assert_buffer_lines([
            "┌ fire ──────────┐",
            "│ ▄██▆▆▄▂▂       │",
            "│▅█████████▇▇▅▃▃▁│",
            "└────────────────┘",
        ]);
    }
}
