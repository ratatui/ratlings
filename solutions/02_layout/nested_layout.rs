use ratatui::layout::{Constraint, Layout, Rect};

#[derive(Debug, PartialEq, Eq)]
struct Dashboard {
    header: Rect,
    sidebar: Rect,
    content: Rect,
}

fn dashboard_layout(area: Rect) -> Dashboard {
    let [header, main] = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);
    let [sidebar, content] = Layout::horizontal([Constraint::Percentage(25), Constraint::Fill(1)])
        .spacing(1)
        .areas(main);
    Dashboard {
        header,
        sidebar,
        content,
    }
}

fn main() {
    println!("{:?}", dashboard_layout(Rect::new(0, 0, 81, 20)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dashboard_layout() {
        let dashboard = dashboard_layout(Rect::new(0, 0, 81, 20));
        assert_eq!(
            dashboard,
            Dashboard {
                header: Rect::new(0, 0, 81, 3),
                sidebar: Rect::new(0, 3, 20, 17),
                content: Rect::new(21, 3, 60, 17),
            }
        );
    }

    #[test]
    fn test_dashboard_layout_small() {
        let dashboard = dashboard_layout(Rect::new(0, 0, 41, 8));
        assert_eq!(
            dashboard,
            Dashboard {
                header: Rect::new(0, 0, 41, 3),
                sidebar: Rect::new(0, 3, 10, 5),
                content: Rect::new(11, 3, 30, 5),
            }
        );
    }
}
