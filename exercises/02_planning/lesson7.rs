// Everything is ready. The chefs are halfway out the door when Ratatui
// freezes on the threshold with the look of a rat who has remembered
// something.
//
// The matches. Nobody packed the matches.
//
// ──────────────────────────────────────────────────────────────────
//
// Widgets draw on top of whatever is already in the buffer, and most of
// them only touch the cells they have something to put in. A `Paragraph`
// writes its text and a `Block` writes its border, but the space between
// is left exactly as it was. That is fine when the area underneath is
// empty. For a popup over a busy screen, it means the background bleeds
// through.
//
// `Clear` is the fix. It is a widget that does nothing but reset every
// cell in its area, so whatever you render after it starts from a blank
// slate.

use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Flex, Layout, Rect},
    widgets::{Block, Paragraph, Wrap},
};
use std::io::IsTerminal;

/// Pop a question over the top of whatever is on screen.
fn popup(frame: &mut Frame, area: Rect) {
    // TODO: Run it. The question is there, but the forest shows through the
    // inside of the box. Render a `Clear` over `area` before the paragraph
    // so the popup starts from a blank patch of screen.
    //
    // Docs: https://docs.rs/ratatui/latest/ratatui/widgets/struct.Clear.html
    let question = Paragraph::new("Did you pack the matches?")
        .centered()
        .block(Block::bordered().title(" wait "));
    frame.render_widget(question, area);
}

pub fn render(frame: &mut Frame) {
    let forest = Paragraph::new("/\\ ".repeat(400)).wrap(Wrap { trim: true });
    frame.render_widget(forest, frame.area());

    let [area] = Layout::horizontal([Constraint::Length(30)])
        .flex(Flex::Center)
        .areas(frame.area());
    let [area] = Layout::vertical([Constraint::Length(3)])
        .flex(Flex::Center)
        .areas(area);
    popup(frame, area);
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
    fn the_question_is_legible() {
        let mut terminal = Terminal::new(TestBackend::new(40, 7)).expect("test backend");
        terminal.draw(render).expect("render frame");
        terminal.backend().assert_buffer_lines([
            "/\\ /\\ /\\ /\\ /\\ /\\ /\\ /\\ /\\ /\\ /\\ /\\ /\\  ",
            "/\\ /\\ /\\ /\\ /\\ /\\ /\\ /\\ /\\ /\\ /\\ /\\ /\\  ",
            "/\\ /\\┌ wait ──────────────────────┐ /\\  ",
            "/\\ /\\│  Did you pack the matches? │ /\\  ",
            "/\\ /\\└────────────────────────────┘ /\\  ",
            "/\\ /\\ /\\ /\\ /\\ /\\ /\\ /\\ /\\ /\\ /\\ /\\ /\\  ",
            "/\\ /\\ /\\ /\\ /\\ /\\ /\\ /\\ /\\ /\\ /\\ /\\ /\\  ",
        ]);
    }
}
