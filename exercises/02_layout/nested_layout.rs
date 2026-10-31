// Real screens are built by nesting layouts: split the screen once, then split
// one of the resulting areas again.
//
// Complete `dashboard_layout` so that it:
// 1. Splits `area` vertically into a 3 row header and a main area that fills
//    the rest.
// 2. Splits the main area horizontally into a sidebar that takes 25% of the
//    width and a content area that fills the rest, with 1 column of spacing
//    between them.
//
// Execute `rustlings hint nested_layout` or use the `hint` watch subcommand for a hint.

#![allow(unused_imports, unused_variables)]

use ratatui::layout::{Constraint, Layout, Rect};

#[derive(Debug, PartialEq, Eq)]
struct Dashboard {
    header: Rect,
    sidebar: Rect,
    content: Rect,
}

fn dashboard_layout(area: Rect) -> Dashboard {
    // TODO: Split `area` twice and return the three areas.
    todo!()
}

fn main() {
    // You can optionally experiment here.
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
