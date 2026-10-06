// "Climbs to the camp," Linguini said. He did not say how much.
//
// Rémy has done the reading. He has elevations for every mile of the trail
// and he would like everyone to look at them before, not during. Ratatui
// agrees: nobody should find out about the last two miles the hard way.
//
// ──────────────────────────────────────────────────────────────────
//
// A `Chart` plots one or more `Dataset`s of (x, y) points against two
// `Axis` definitions. The dataset is only half the picture. The axes decide
// which part of the plane is visible, with `bounds`, and what the reader
// sees along each edge, with `labels`. A chart with no axis bounds has
// nowhere to put the points.

use ratatui::{
    DefaultTerminal, Frame,
    style::Color,
    symbols::Marker,
    widgets::{Axis, Block, Chart, Dataset, GraphType},
};
use std::io::IsTerminal;

/// Elevation in feet at each mile of the trail.
const PROFILE: [(f64, f64); 6] = [
    (0.0, 1200.0),
    (1.0, 1500.0),
    (2.0, 1450.0),
    (3.0, 2100.0),
    (4.0, 2600.0),
    (5.0, 2400.0),
];

/// Give the chart its axes: miles along the bottom, feet up the side.
fn axes(chart: Chart<'static>) -> Chart<'static> {
    chart
        .x_axis(
            Axis::default()
                .title("mi")
                .bounds([0.0, 5.0])
                .labels(["0", "2.5", "5"]),
        )
        .y_axis(
            Axis::default()
                .title("ft")
                .bounds([1000.0, 3000.0])
                .labels(["1000", "2000", "3000"]),
        )
}

pub fn render(frame: &mut Frame) {
    let climb = Dataset::default()
        .marker(Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Color::Green)
        .data(&PROFILE);
    let chart = axes(Chart::new(vec![climb]).block(Block::bordered().title(" the climb ")));
    frame.render_widget(chart, frame.area());
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
    fn the_last_two_miles_are_visible() {
        let mut terminal = Terminal::new(TestBackend::new(34, 12)).expect("test backend");
        terminal.draw(render).expect("render frame");
        assert_eq!(
            rows(&terminal),
            [
                "┌ the climb ─────────────────────┐",
                "│3000│ft                         │",
                "│    │                    ⢀⠤⣀⣀   │",
                "│    │                  ⡠⠊⠁   ⠉⠉⠒│",
                "│    │               ⢀⠔⠉         │",
                "│2000│             ⣀⠔⠁           │",
                "│    │     ⣀⣀⣀⡀  ⡠⠊              │",
                "│    │ ⣀⠤⠒⠉   ⠈⠉⠉                │",
                "│1000│⠉                        mi│",
                "│    └───────────────────────────│",
                "│    0            2.5           5│",
                "└────────────────────────────────┘",
            ]
        );
    }
}
