// Ratatui text is built from small pieces: a `Span` is a string with a single
// style, and a `Line` is a row of spans. This lets one line mix several styles.
//
// Complete `status_line` so that it returns a `Line` made of three spans:
// 1. The user name in bold.
// 2. The unstyled text " is ".
// 3. "online" in bold green if `online` is true, otherwise "offline" in red.
//
// Execute `rustlings hint styled_text` or use the `hint` watch subcommand for a hint.

#![allow(unused_imports, unused_variables)]

use ratatui::{style::Stylize, text::Line};

fn status_line(user: &str, online: bool) -> Line<'_> {
    // TODO: Build the `Line` from three styled `Span`s.
    todo!()
}

fn main() {
    // You can optionally experiment here.
    println!("{:?}", status_line("ferris", true));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{
        style::{Color, Modifier, Style},
        text::Span,
    };

    #[test]
    fn test_online_user() {
        let line = status_line("ferris", true);
        assert_eq!(
            line.spans,
            vec![
                Span::styled("ferris", Style::new().add_modifier(Modifier::BOLD)),
                Span::raw(" is "),
                Span::styled(
                    "online",
                    Style::new().fg(Color::Green).add_modifier(Modifier::BOLD),
                ),
            ]
        );
    }

    #[test]
    fn test_offline_user() {
        let line = status_line("ratatui", false);
        assert_eq!(
            line.spans,
            vec![
                Span::styled("ratatui", Style::new().add_modifier(Modifier::BOLD)),
                Span::raw(" is "),
                Span::styled("offline", Style::new().fg(Color::Red)),
            ]
        );
    }
}
