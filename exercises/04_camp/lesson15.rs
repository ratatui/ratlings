// Three days. Somehow, in the way of all good trips, they have blurred into
// one long one, and nobody can agree whether the pancakes were yesterday or
// this morning.
//
// Ratatui draws a line under it. One tab per day, and today marked.
//
// ──────────────────────────────────────────────────────────────────
//
// `Tabs` draws a row of titles with a divider between them. Pass it the
// titles, and `select` the index of the one that is current; that one is
// drawn in the `highlight_style`, reversed by default. The widget only
// draws the bar. Which day's content goes underneath is up to you, and is
// usually a `match` on the same index.

use ratatui::{
    DefaultTerminal, Frame,
    widgets::{Block, Tabs},
};
use std::io::IsTerminal;

const DAYS: [&str; 3] = ["Friday", "Saturday", "Sunday"];
const TODAY: usize = 1;

/// The trip, one tab per day, with today picked out.
fn days() -> Tabs<'static> {
    // TODO: Every day looks the same. `select` the tab at `TODAY` so it is
    // drawn in the highlight style.
    //
    // Docs: https://docs.rs/ratatui/latest/ratatui/widgets/struct.Tabs.html
    Tabs::new(DAYS)
}

pub fn render(frame: &mut Frame) {
    let trip = days().block(Block::bordered().title(" the trip "));
    frame.render_widget(trip, frame.area());
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
    fn today_is_saturday() {
        let mut terminal = Terminal::new(TestBackend::new(32, 3)).expect("test backend");
        terminal.draw(render).expect("render frame");
        assert_eq!(
            rows(&terminal),
            [
                "┌ the trip ────────────────────┐",
                "│ Friday │ Saturday │ Sunday   │",
                "└──────────────────────────────┘",
            ]
        );

        let buffer = terminal.backend().buffer();
        let highlighted: String = (0..buffer.area.width)
            .filter(|&x| buffer[(x, 1)].modifier.contains(Modifier::REVERSED))
            .map(|x| buffer[(x, 1)].symbol())
            .collect();
        assert_eq!(highlighted, "Saturday", "only today should be highlighted");
    }
}
