// Ratatui and his fellow chef friends are going going on an adventure!
//
// After years of working away, spending long hours in the kitchen, and late nights
// coming up with the perfect recipes, they've earned some much needed rest and
// relaxation. But this hairy bunch isn't headed to a summer home, nor a nice,
// relaxing beach. No, _they're going camping_!
//
// Only... there's one problem. Ratatui has never gone camping before. It's
// time to pack, but he has no idea what he needs to bring, or how to organize
// his bag!
//
// Everything you see in a Ratatui app is a widget drawn into an area of the
// frame. `render_widget` is the call that puts one there, and `frame.area()`
// is the whole terminal. A `Block` is the simplest useful widget there is: a
// box, optionally with borders and a title.

use ratatui::{DefaultTerminal, Frame, widgets::Block};
use std::io::IsTerminal;

fn render(frame: &mut Frame) {
    // TODO: Ratatui is rendering a block, but a plain `Block::new()` draws
    // nothing at all — run it and you get an empty screen. Give him a bag he
    // can actually see: a block with borders, titled " pack ".
    // Docs: https://docs.rs/ratatui/latest/ratatui/widgets/struct.Block.html
    frame.render_widget(Block::new(), frame.area());
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
    fn the_pack_is_a_bag_you_can_see() {
        let mut terminal = Terminal::new(TestBackend::new(24, 5)).expect("test backend");
        terminal.draw(render).expect("render frame");
        terminal.backend().assert_buffer_lines([
            "┌ pack ────────────────┐",
            "│                      │",
            "│                      │",
            "│                      │",
            "└──────────────────────┘",
        ]);
    }
}
