// ─── WRITE ME ─────────────────────────────────────────────────────────────
// SLOT:   Bridging story beat for pack2.
// GOAL:   One or two lines carrying Ratatui from "he has a bag" to "now he has
//         to decide what goes where in it". The section opener already did the
//         scene-setting on pack1, so this only needs to move him along.
// LENGTH: ~1-2 lines.
// ──────────────────────────────────────────────────────────────────────────

// Packing is a layout problem. The pack is a fixed height and most of the gear
// is not negotiable: the rain shell, the stove and the sleeping bag each need
// the same room whatever pack they go in. The food bag is the one that flexes,
// and it has to keep flexing when the pack changes size.
//
// Reserving room for the fixed things and letting one thing claim the rest is
// the whole job of a layout.

use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Rect},
    widgets::Block,
};
use std::io::IsTerminal;

const GEAR: [&str; 4] = ["rain shell", "stove", "food", "sleeping bag"];

/// Divide the pack interior into one compartment per item of gear, top to bottom.
fn compartments(pack: Rect) -> [Rect; 4] {
    // TODO: This pack is packed wrong. Run it and look at it — every
    // compartment is nailed to 3 rows, the food bag has nowhere to grow, and
    // the bottom of the pack is dead air. Give each item of `GEAR` the
    // constraint it actually needs, in the same order.
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
