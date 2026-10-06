// Where the trail leaves the meadow there is a wooden sign, and the sign
// has a lot to say. Linguini reads the first line, loses the rest off the
// edge of the board, and announces that it says "Bear".
//
// It does not just say "Bear".
//
// ──────────────────────────────────────────────────────────────────
//
// `Paragraph` is the widget for text. By default it draws each line as it
// is and lets anything too long run off the right edge. Give it a `Wrap`
// and it will break long lines at word boundaries to fit the area. It can
// also align its lines; `centered` is the one a sign wants.

use ratatui::{
    DefaultTerminal, Frame,
    widgets::{Block, Paragraph, Wrap},
};
use std::io::IsTerminal;

const SIGN: &str = "Bear Creek Trail. Camp 8 miles. Water at the footbridge. \
    Pack out what you pack in, and that includes the cheese.";

/// The sign, laid out so a hungry rat can read all of it.
fn sign() -> Paragraph<'static> {
    Paragraph::new(SIGN).wrap(Wrap { trim: true }).centered()
}

pub fn render(frame: &mut Frame) {
    let board = sign().block(Block::bordered().title(" trail sign "));
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
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn it_does_not_just_say_bear() {
        let mut terminal = Terminal::new(TestBackend::new(32, 7)).expect("test backend");
        terminal.draw(render).expect("render frame");
        terminal.backend().assert_buffer_lines([
            "┌ trail sign ──────────────────┐",
            "│   Bear Creek Trail. Camp 8   │",
            "│      miles. Water at the     │",
            "│ footbridge. Pack out what you│",
            "│pack in, and that includes the│",
            "│            cheese.           │",
            "└──────────────────────────────┘",
        ]);
    }
}
