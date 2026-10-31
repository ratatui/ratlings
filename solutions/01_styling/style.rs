use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone, Copy)]
enum Level {
    Info,
    Warning,
    Error,
}

fn level_style(level: Level) -> Style {
    match level {
        Level::Info => Style::new().fg(Color::Blue),
        Level::Warning => Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        Level::Error => Style::new()
            .fg(Color::White)
            .bg(Color::Red)
            .add_modifier(Modifier::BOLD),
    }
}

fn main() {
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
