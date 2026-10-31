use ratatui::{style::Stylize, text::Line};

fn status_line(user: &str, online: bool) -> Line<'_> {
    let status = if online {
        "online".green().bold()
    } else {
        "offline".red()
    };
    Line::from(vec![user.bold(), " is ".into(), status])
}

fn main() {
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
