// These are chefs. Dinner was never going to be a packet of noodles.
//
// Colette has a plan for every meal of the trip and she would like it
// posted where it can be consulted and not argued with. Day down the side,
// lunch and dinner across the top, and no cheating.
//
// ──────────────────────────────────────────────────────────────────
//
// A `Table` lays out `Row`s of cells in columns. Unlike a `List` it needs
// to know how wide each column is, and it takes those as `Constraint`s,
// the same ones `Layout` uses. A `header` is just another `Row`, drawn
// once at the top and kept there when the body scrolls.

use ratatui::{
    DefaultTerminal, Frame,
    layout::Constraint,
    widgets::{Block, Row, Table},
};
use std::io::IsTerminal;

const MENU: [[&str; 3]; 3] = [
    ["fri", "trail mix", "chili"],
    ["sat", "oats", "ratatouille"],
    ["sun", "pancakes", "leftovers"],
];

/// The plan: a labelled column for each meal, wide enough to read.
fn menu() -> Table<'static> {
    let rows = MENU.map(Row::new);
    // TODO: Every column is three characters wide and there is no header, so
    // the menu reads "fri tra chi". Give the table widths that fit (3, then
    // 9, then whatever is left), and a `header` row of "day", "lunch" and
    // "dinner" made `bold` (that method comes from `ratatui::style::Stylize`).
    //
    // Docs: https://docs.rs/ratatui/latest/ratatui/widgets/struct.Table.html
    let widths = [Constraint::Length(3); 3];
    Table::new(rows, widths)
}

pub fn render(frame: &mut Frame) {
    let board = menu().block(Block::bordered().title(" menu "));
    frame.render_widget(board, frame.area());
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
    fn nobody_argues_with_the_menu() {
        let mut terminal = Terminal::new(TestBackend::new(30, 6)).expect("test backend");
        terminal.draw(render).expect("render frame");
        assert_eq!(
            rows(&terminal),
            [
                "┌ menu ──────────────────────┐",
                "│day lunch     dinner        │",
                "│fri trail mix chili         │",
                "│sat oats      ratatouille   │",
                "│sun pancakes  leftovers     │",
                "└────────────────────────────┘",
            ]
        );
        let buffer = terminal.backend().buffer();
        assert!(
            buffer[(1, 1)].modifier.contains(Modifier::BOLD),
            "the header is bold"
        );
        assert!(
            !buffer[(1, 2)].modifier.contains(Modifier::BOLD),
            "the body is not"
        );
    }
}
