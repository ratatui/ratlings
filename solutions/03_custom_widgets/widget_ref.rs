use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

struct UserCard {
    name: String,
    role: String,
    online: bool,
}

impl Widget for &UserCard {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let status = if self.online {
            "● online".green()
        } else {
            "○ offline".dark_gray()
        };
        Paragraph::new(vec![Line::from(self.role.as_str()), Line::from(status)])
            .block(Block::bordered().title(format!(" {} ", self.name)))
            .render(area, buf);
    }
}

fn main() {
    let card = UserCard {
        name: String::from("Ferris"),
        role: String::from("Mascot"),
        online: true,
    };
    let area = Rect::new(0, 0, 16, 4);
    let mut buf = Buffer::empty(area);
    card.render(area, &mut buf);
    println!("{buf:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::{Color, Style};

    fn ferris() -> UserCard {
        UserCard {
            name: String::from("Ferris"),
            role: String::from("Mascot"),
            online: true,
        }
    }

    #[test]
    fn test_render_online_card() {
        let area = Rect::new(0, 0, 16, 4);
        let mut buf = Buffer::empty(area);
        ferris().render(area, &mut buf);

        let mut expected = Buffer::with_lines([
            "┌ Ferris ──────┐",
            "│Mascot        │",
            "│● online      │",
            "└──────────────┘",
        ]);
        expected.set_style(Rect::new(1, 2, 8, 1), Style::new().fg(Color::Green));
        assert_eq!(buf, expected);
    }

    #[test]
    fn test_render_offline_card() {
        let card = UserCard {
            name: String::from("Corro"),
            role: String::from("Maintainer"),
            online: false,
        };
        let area = Rect::new(0, 0, 16, 4);
        let mut buf = Buffer::empty(area);
        card.render(area, &mut buf);

        let mut expected = Buffer::with_lines([
            "┌ Corro ───────┐",
            "│Maintainer    │",
            "│○ offline     │",
            "└──────────────┘",
        ]);
        expected.set_style(Rect::new(1, 2, 9, 1), Style::new().fg(Color::DarkGray));
        assert_eq!(buf, expected);
    }

    #[test]
    fn test_card_can_be_rendered_twice() {
        // Rendering a `&UserCard` doesn't consume the card, so it can be drawn
        // again on the next frame.
        let card = ferris();
        let area = Rect::new(0, 0, 16, 4);
        let mut first = Buffer::empty(area);
        let mut second = Buffer::empty(area);
        card.render(area, &mut first);
        card.render(area, &mut second);
        assert_eq!(first, second);
    }
}
