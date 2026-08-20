// ─── WRITE ME ─────────────────────────────────────────────────────────────
// SLOT:   Exercise brief — the first thing the student reads on opening this
//         file, and the only lesson text rustlings guarantees they will see.
// GOAL:   Set the scene for this outing, then explain the one idea being
//         taught: some gear is a fixed size, and one item absorbs whatever
//         space is left over. Do not explain the API — the docs link does
//         that. Explain why a pack is a layout problem.
// LENGTH: ~4-6 lines.
// ──────────────────────────────────────────────────────────────────────────

use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Rect},
    widgets::Block,
};
use std::io::IsTerminal;

// ─── WRITE ME (optional) ──────────────────────────────────────────────────
// SLOT:   The gear list, top of the pack to the bottom of it.
// GOAL:   These are placeholder names chosen so the packing order is the one
//         a real backpacker would use — quick-access layer at the top, dense
//         food in the middle, sleeping bag crushed into the bottom. Rename
//         them freely, but keep them under 20 characters so they fit the
//         compartment titles, and if you do rename them, update the expected
//         buffer in `the_pack_renders` below.
// ──────────────────────────────────────────────────────────────────────────
const GEAR: [&str; 4] = ["rain shell", "stove", "food", "sleeping bag"];

/// Divide the pack interior into one compartment per item of gear, top to bottom.
fn compartments(pack: Rect) -> [Rect; 4] {
    // TODO: This pack is packed wrong. Run it and look at it: every
    // compartment is nailed to 3 rows, so the food bag cannot grow into the
    // space the rest of the gear leaves behind, and the bottom of the pack is
    // dead air. Three of these items really are a fixed size. One is not.
    // Docs: https://docs.rs/ratatui/latest/ratatui/layout/enum.Constraint.html
    let constraints: [Constraint; 4] = [
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
    ];
    Layout::vertical(constraints).areas(pack)
}

fn render(frame: &mut Frame) {
    let pack = Block::bordered().title(" pack ");
    let interior = pack.inner(frame.area());
    frame.render_widget(pack, frame.area());

    for (area, label) in compartments(interior).into_iter().zip(GEAR) {
        frame.render_widget(Block::bordered().title(label), area);
    }
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if ratatui::crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn main() -> std::io::Result<()> {
    // Rustlings runs this binary with no terminal attached. Only take over the
    // screen when there is a real one to take over.
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
    fn fixed_gear_always_takes_three_rows() {
        let [shell, stove, _food, sleeping_bag] = compartments(Rect::new(1, 1, 22, 14));

        assert_eq!(shell, Rect::new(1, 1, 22, 3), "the rain shell rides on top");
        assert_eq!(stove, Rect::new(1, 4, 22, 3), "the stove sits under it");
        assert_eq!(
            sleeping_bag,
            Rect::new(1, 12, 22, 3),
            "the sleeping bag is crushed into the bottom"
        );
    }

    #[test]
    fn food_takes_the_space_the_other_gear_leaves() {
        // A bigger pack does not mean a bigger stove. Every extra row belongs
        // to the food bag, so its compartment cannot be a fixed size.
        let [_, _, small, _] = compartments(Rect::new(1, 1, 22, 14));
        let [_, _, large, _] = compartments(Rect::new(1, 1, 22, 20));

        assert_eq!(
            small,
            Rect::new(1, 7, 22, 5),
            "14 rows, 9 of them spoken for"
        );
        assert_eq!(
            large,
            Rect::new(1, 7, 22, 11),
            "six more rows in the pack should be six more rows of food"
        );
    }

    #[test]
    fn the_pack_renders() {
        let mut terminal = Terminal::new(TestBackend::new(24, 16)).expect("test backend");
        terminal.draw(render).expect("render frame");
        terminal.backend().assert_buffer_lines([
            "┌ pack ────────────────┐",
            "│┌rain shell──────────┐│",
            "││                    ││",
            "│└────────────────────┘│",
            "│┌stove───────────────┐│",
            "││                    ││",
            "│└────────────────────┘│",
            "│┌food────────────────┐│",
            "││                    ││",
            "││                    ││",
            "││                    ││",
            "│└────────────────────┘│",
            "│┌sleeping bag────────┐│",
            "││                    ││",
            "│└────────────────────┘│",
            "└──────────────────────┘",
        ]);
    }
}
