// A `Style` describes how text looks: its foreground color, background color
// and modifiers such as bold or italic.
//
// Complete `level_style` so that it returns:
// - `Info`: blue text.
// - `Warning`: bold yellow text.
// - `Error`: bold white text on a red background.
//
// Execute `rustlings hint style` or use the `hint` watch subcommand for a hint.

#![allow(unused_imports, unused_variables)]

use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone, Copy)]
enum Level {
    Info,
    Warning,
    Error,
}

fn level_style(level: Level) -> Style {
    // TODO: Return a different `Style` for each `Level`.
    todo!()
}

fn main() {
    // You can optionally experiment here.
    for level in [Level::Info, Level::Warning, Level::Error] {
        println!("{level:?}: {:?}", level_style(level));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_info_is_blue() {
        let style = level_style(Level::Info);
        assert_eq!(style.fg, Some(Color::Blue));
        assert_eq!(style.bg, None);
        assert_eq!(style.add_modifier, Modifier::empty());
    }

    #[test]
    fn test_warning_is_bold_yellow() {
        let style = level_style(Level::Warning);
        assert_eq!(style.fg, Some(Color::Yellow));
        assert_eq!(style.bg, None);
        assert_eq!(style.add_modifier, Modifier::BOLD);
    }

    #[test]
    fn test_error_is_bold_white_on_red() {
        let style = level_style(Level::Error);
        assert_eq!(style.fg, Some(Color::White));
        assert_eq!(style.bg, Some(Color::Red));
        assert_eq!(style.add_modifier, Modifier::BOLD);
    }
}
