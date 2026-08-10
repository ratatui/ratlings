use ratatui::{DefaultTerminal, Frame};
use std::io::IsTerminal;

// Do not modify this line
fn main() -> std::io::Result<()> {
    // TODO: run the TUI app.
    // ratatui::run(app)
    todo!("run the TUI app.")
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if !std::io::stdout().is_terminal() || ratatui::crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn renders_hello_world() {
        let mut terminal =
            Terminal::new(TestBackend::new(15, 1)).expect("test backend initialization");
        terminal.draw(render).expect("render frame");
        terminal.backend().assert_buffer_lines(["hello world    "]);
    }
}
