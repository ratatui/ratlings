// Dates on the wall, bag by the door. There is just the small matter of
// where they are actually going.
//
// Linguini unfolds a map across the pass. It is mostly coffee stains. The
// trail starts at the trailhead, crosses a footbridge, skirts a meadow, and
// climbs to the camp. Ratatui would like that drawn somewhere he can read.
//
// ──────────────────────────────────────────────────────────────────
//
// `Canvas` is the widget for drawing shapes rather than text. You give it a
// coordinate system with `x_bounds` and `y_bounds`, and a `paint` function
// that receives a `Context`. The context has `draw` for shapes such as
// `Line`, `Rectangle` and `Circle`, and `print` for labels. The canvas maps
// your coordinates onto terminal cells for you.

use ratatui::{
    DefaultTerminal, Frame,
    symbols::Marker,
    widgets::{
        Block,
        canvas::{Canvas, Context},
    },
};
use std::io::IsTerminal;

/// Waypoints along the trail, as (x, y) on a map that runs 0..10 by 0..6.
const ROUTE: [(f64, f64); 4] = [(0.0, 0.0), (3.0, 2.0), (6.0, 1.0), (10.0, 6.0)];

/// Draw the trail as a line from each waypoint to the next.
fn trail(ctx: &mut Context) {
    // TODO: The labels are on the map but there is no trail between them.
    // Walk `ROUTE` two waypoints at a time (`windows(2)` on a slice does
    // exactly this) and `draw` a `canvas::Line` from each one to the next,
    // in whatever `Color` you like. Both types still need importing.
    //
    // Docs: https://docs.rs/ratatui/latest/ratatui/widgets/canvas/struct.Line.html
    ctx.print(0.0, 0.0, "trailhead");
    ctx.print(7.0, 6.0, "camp");
}

pub fn render(frame: &mut Frame) {
    let map = Canvas::default()
        .block(Block::bordered().title(" map "))
        .marker(Marker::Braille)
        .x_bounds([0.0, 10.0])
        .y_bounds([0.0, 6.0])
        .paint(trail);
    frame.render_widget(map, frame.area());
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
    fn the_trail_connects_the_dots() {
        let mut terminal = Terminal::new(TestBackend::new(32, 10)).expect("test backend");
        terminal.draw(render).expect("render frame");
        assert_eq!(
            rows(&terminal),
            [
                "┌ map ─────────────────────────┐",
                "│                    camp    ⡠⠊│",
                "│                          ⢠⠊  │",
                "│                        ⢀⠔⠁   │",
                "│                      ⢀⠔⠁     │",
                "│                    ⢀⠔⠁       │",
                "│       ⣀⠤⠒⠤⠤⣀⡀     ⡠⠊         │",
                "│   ⢀⡠⠒⠉      ⠈⠉⠒⠒⠤⠊           │",
                "│trailhead                     │",
                "└──────────────────────────────┘",
            ]
        );
    }
}
