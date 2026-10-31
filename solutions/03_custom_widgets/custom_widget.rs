use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};

struct ProgressBar {
    percent: u16,
}

impl ProgressBar {
    // Number of cells to fill for a bar that is `width` cells wide.
    fn filled_cells(&self, width: u16) -> u16 {
        let filled = u32::from(width) * u32::from(self.percent.min(100)) / 100;
        u16::try_from(filled).unwrap_or(width)
    }
}

impl Widget for ProgressBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let filled = self.filled_cells(area.width);
        for x in area.left()..area.right() {
            let cell = &mut buf[(x, area.top())];
            if x - area.left() < filled {
                cell.set_symbol("█").set_fg(Color::Green);
            } else {
                cell.set_symbol("░").set_fg(Color::DarkGray);
            }
        }
    }
}

fn main() {
    let area = Rect::new(0, 0, 20, 1);
    let mut buf = Buffer::empty(area);
    ProgressBar { percent: 40 }.render(area, &mut buf);
    println!("{buf:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    fn expected_bar(width: u16, filled: u16) -> Buffer {
        let mut expected = Buffer::with_lines([format!(
            "{}{}",
            "█".repeat(filled.into()),
            "░".repeat((width - filled).into())
        )]);
        expected.set_style(Rect::new(0, 0, filled, 1), Style::new().fg(Color::Green));
        expected.set_style(
            Rect::new(filled, 0, width - filled, 1),
            Style::new().fg(Color::DarkGray),
        );
        expected
    }

    #[test]
    fn test_half_full() {
        let area = Rect::new(0, 0, 10, 1);
        let mut buf = Buffer::empty(area);
        ProgressBar { percent: 50 }.render(area, &mut buf);
        assert_eq!(buf, expected_bar(10, 5));
    }

    #[test]
    fn test_empty_and_full() {
        let area = Rect::new(0, 0, 8, 1);

        let mut buf = Buffer::empty(area);
        ProgressBar { percent: 0 }.render(area, &mut buf);
        assert_eq!(buf, expected_bar(8, 0));

        let mut buf = Buffer::empty(area);
        ProgressBar { percent: 100 }.render(area, &mut buf);
        assert_eq!(buf, expected_bar(8, 8));
    }

    #[test]
    fn test_renders_only_inside_area() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 6, 3));
        ProgressBar { percent: 50 }.render(Rect::new(1, 1, 4, 1), &mut buf);

        let mut expected = Buffer::with_lines([
            "      ", //
            " ██░░ ",
            "      ",
        ]);
        expected.set_style(Rect::new(1, 1, 2, 1), Style::new().fg(Color::Green));
        expected.set_style(Rect::new(3, 1, 2, 1), Style::new().fg(Color::DarkGray));
        assert_eq!(buf, expected);
    }
}
