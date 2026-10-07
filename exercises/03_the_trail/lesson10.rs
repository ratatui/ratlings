// A rest stop, finally. Packs come off, boots come off, and Ratatui goes
// through the gear one item at a time, because a rat who has forgotten the
// matches once does not intend to do it twice.
//
// ──────────────────────────────────────────────────────────────────
//
// A `List` draws its items one per row. On its own it has no idea which
// item you are looking at. That knowledge lives in a `ListState`, a little
// struct that remembers the selection and the scroll offset, and the two
// are joined by `render_stateful_widget` instead of `render_widget`.
//
// This split shows up all over Ratatui. The widget describes how things
// look and is rebuilt every frame. The state describes what is going on
// and lives as long as your app does.

use ratatui::{
    DefaultTerminal, Frame,
    style::Style,
    widgets::{Block, List},
};
use std::io::IsTerminal;

const GEAR: [&str; 5] = ["rain shell", "stove", "food", "sleeping bag", "matches"];

/// The item Ratatui is checking right now.
const CHECKING: usize = 3;

pub fn render(frame: &mut Frame) {
    let checklist = List::new(GEAR)
        .block(Block::bordered().title(" rest stop "))
        .highlight_symbol("> ")
        .highlight_style(Style::new().bold());

    // TODO: Nothing is highlighted, because the list has no idea which item
    // Ratatui is on. Make a `ListState` with `CHECKING` selected, and swap
    // `render_widget` for `render_stateful_widget` so the two meet.
    //
    // Docs: https://docs.rs/ratatui/latest/ratatui/widgets/struct.ListState.html
    frame.render_widget(checklist, frame.area());
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
    fn the_sleeping_bag_is_being_checked() {
        let mut terminal = Terminal::new(TestBackend::new(24, 7)).expect("test backend");
        terminal.draw(render).expect("render frame");
        assert_eq!(
            rows(&terminal),
            [
                "┌ rest stop ───────────┐",
                "│  rain shell          │",
                "│  stove               │",
                "│  food                │",
                "│> sleeping bag        │",
                "│  matches             │",
                "└──────────────────────┘",
            ]
        );
        let buffer = terminal.backend().buffer();
        assert!(
            buffer[(3, 4)].modifier.contains(Modifier::BOLD),
            "the selected item should be drawn in the highlight style"
        );
    }
}
