// The bag is packed. Ratatui hoists it onto his shoulders, takes one step,
// and sits right back down. Something in there weighs a ton, and he has a
// suspicion it is the cheese.
//
// Before anything gets left behind, the chefs want to _see_ the problem.
// Rémy fetches the kitchen scale.
//
// ──────────────────────────────────────────────────────────────────
//
// A `BarChart` turns a list of numbers into a row of bars. Each bar is a
// `Bar`: a value, and optionally a label underneath it. Hand the chart a
// `Vec<Bar>` and it works out the heights; you only choose how wide each bar
// is and how much gap sits between them.

use ratatui::{
    DefaultTerminal, Frame,
    widgets::{Bar, BarChart, Block},
};
use std::io::IsTerminal;

/// Everything in the pack, and what it weighs in ounces.
const GEAR: [(&str, u64); 4] = [("shell", 9), ("stove", 14), ("food", 38), ("bag", 22)];

/// One bar per item of gear, labelled with its name, as tall as its weight.
fn bars() -> Vec<Bar<'static>> {
    // TODO: The scale is switched on but nothing is on it. Build one `Bar`
    // for each entry in `GEAR`: its weight is the value and its name is the
    // label. `Bar` has a constructor that takes both at once.
    //
    // Docs: https://docs.rs/ratatui/latest/ratatui/widgets/struct.Bar.html
    Vec::new()
}

pub fn render(frame: &mut Frame) {
    let scale = BarChart::vertical(bars())
        .bar_width(5)
        .bar_gap(1)
        .block(Block::bordered().title(" scale (oz) "));
    frame.render_widget(scale, frame.area());
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
    fn every_item_gets_a_bar() {
        assert_eq!(bars().len(), GEAR.len(), "one bar per item of gear");
    }

    #[test]
    fn the_food_is_the_problem() {
        let mut terminal = Terminal::new(TestBackend::new(26, 10)).expect("test backend");
        terminal.draw(render).expect("render frame");
        terminal.backend().assert_buffer_lines([
            "┌ scale (oz) ────────────┐",
            "│            █████       │",
            "│            █████       │",
            "│            █████       │",
            "│            █████ █████ │",
            "│      ▄▄▄▄▄ █████ █████ │",
            "│▅▅▅▅▅ █████ █████ █████ │",
            "│██9██ █14██ █38██ █22██ │",
            "│shell stove food   bag  │",
            "└────────────────────────┘",
        ]);
    }
}
