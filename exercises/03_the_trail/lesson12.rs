// Rémy brought a guidebook. Of course he did. It is small, it is dense, and
// the page about Bear Creek goes on for longer than the screen is tall.
//
// He can scroll it, but with nothing to show where he is, he keeps reading
// the same paragraph about lichen.
//
// ──────────────────────────────────────────────────────────────────
//
// `Paragraph` can `scroll` by a number of rows, but it does not show that
// it has. A `Scrollbar` does. It is another stateful widget: the
// `Scrollbar` says which edge to draw on and with which symbols, while a
// `ScrollbarState` carries the two numbers that matter, how long the
// content is and where in it you are.

use ratatui::{
    DefaultTerminal, Frame,
    layout::{Margin, Rect},
    widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};
use std::io::IsTerminal;

const GUIDEBOOK: &str = "BEAR CREEK
A moderate trail of eight miles
with one sustained climb. The
footbridge at mile two is the
last reliable water. Beyond the
meadow the trail is rocky and
exposed; start early. Lichen on
the north faces is of several
species and worth a pause. The
camp sits in a stand of pines
with a fire ring. Bears are
present. Hang your food.";

/// How many rows the reader has scrolled down.
const SCROLLED: u16 = 5;

/// Show where in the book the reader is, down the right-hand edge.
fn scrollbar(frame: &mut Frame, area: Rect) {
    // TODO: The scrollbar draws nothing, because its state says the book is
    // zero rows long. Build the `ScrollbarState` from the number of lines in
    // `GUIDEBOOK`, with `SCROLLED` as the position.
    //
    // Docs: https://docs.rs/ratatui/latest/ratatui/widgets/struct.ScrollbarState.html
    let mut state = ScrollbarState::default();
    let bar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
    frame.render_stateful_widget(bar, area, &mut state);
}

pub fn render(frame: &mut Frame) {
    let page = Paragraph::new(GUIDEBOOK)
        .scroll((SCROLLED, 0))
        .block(Block::bordered().title(" guidebook "));
    frame.render_widget(page, frame.area());

    let edge = frame.area().inner(Margin::new(0, 1));
    scrollbar(frame, edge);
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
    fn the_reader_is_past_the_lichen() {
        let mut terminal = Terminal::new(TestBackend::new(34, 8)).expect("test backend");
        terminal.draw(render).expect("render frame");
        terminal.backend().assert_buffer_lines([
            "┌ guidebook ─────────────────────┐",
            "│meadow the trail is rocky and   ▲",
            "│exposed; start early. Lichen on ║",
            "│the north faces is of several   █",
            "│species and worth a pause. The  ║",
            "│camp sits in a stand of pines   ║",
            "│with a fire ring. Bears are     ▼",
            "└────────────────────────────────┘",
        ]);
    }
}
