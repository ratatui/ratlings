// `Widget::render` takes `self` by value, so rendering a widget consumes it.
// That is fine for widgets built fresh on every frame, but an app usually
// wants to keep its own state around. Implementing `Widget` for a reference
// (`impl Widget for &MyType`) lets you render the same value again and again.
//
// Custom widgets can also render other widgets. Complete `render` so that the
// `UserCard` draws:
// 1. A bordered `Block` with the user name as its title, padded with one space
//    on each side (" Ferris ").
// 2. Inside the block, the role on the first line.
// 3. On the second line, "● online" in green or "○ offline" in dark gray.
//
// Execute `rustlings hint widget_ref` or use the `hint` watch subcommand for a hint.

#![allow(dead_code, unused_imports, unused_variables)]

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
        // TODO: Render the card using a `Block` and a `Paragraph`.
        todo!()
    }
}

fn main() {
    // You can optionally experiment here.
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
