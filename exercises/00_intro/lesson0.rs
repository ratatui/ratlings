// Welcome to Ratlings!
//
// Ratlings is a set of small exercises for learning Ratatui, the Rust library
// for building terminal user interfaces. Each lesson is one ordinary Rust file
// like this one. You edit it in your editor, and ratlings tells you when it is
// right.
//
// ──────────────────────────────────────────────────────────────────
//
// The screen behind this pane is the _canvas_: your app, drawn at the real size
// of your terminal. Right now it shows the little welcome sign this file
// renders. Ratlings floats on top of it and never changes a cell of it.
//
// This first lesson is just a tour of the keys. Work through the checklist
// above; it ticks off each one as you use it, and `i` brings this pane back
// whenever you close it.
//
// Reading this in your editor? Open the viewer with `cargo run -- lesson0`.

use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Flex, Layout},
    widgets::{Block, Padding, Paragraph},
};
use std::io::IsTerminal;

// TODO: The sign is addressed to nobody. Put your name here, then press `?`
// to run the tests and see them go green. That edit-and-check loop is the
// whole course; everything after this is just more Ratatui.
const NAME: &str = "???";

pub fn render(frame: &mut Frame) {
    let [area] = Layout::horizontal([Constraint::Length(40)])
        .flex(Flex::Center)
        .areas(frame.area());
    let [area] = Layout::vertical([Constraint::Length(5)])
        .flex(Flex::Center)
        .areas(area);
    let sign = Paragraph::new(format!("Hello, {NAME}!\nWelcome to Ratlings."))
        .centered()
        .block(
            Block::bordered()
                .title(" ratlings ")
                .padding(Padding::vertical(1)),
        );
    frame.render_widget(sign, area);
}

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
    fn the_sign_has_your_name_on_it() {
        assert!(
            !NAME.trim().is_empty() && NAME != "???",
            "NAME is still the placeholder — put your own name in it"
        );
    }

    #[test]
    fn the_sign_says_hello_to_you() {
        let mut terminal = Terminal::new(TestBackend::new(60, 9)).expect("test backend");
        terminal.draw(render).expect("render frame");
        let screen = terminal.backend().to_string();
        assert!(
            screen.contains(&format!("Hello, {NAME}!")),
            "the sign should greet {NAME} by name"
        );
    }
}
