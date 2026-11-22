use std::{io::IsTerminal, time::Duration};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent},
};

pub const INGREDIENTS: [(&str, u16); 4] = [
    ("tomatoes", 4),
    ("zucchini", 2),
    ("eggplant", 1),
    ("bell pepper", 2),
];
pub const TOTAL_INGREDIENTS: usize = 9;

pub struct State {
    pub selected: usize,
    pub pot: Vec<usize>,
    pub remaining: [u16; INGREDIENTS.len()],
    pub progress: f64,
    pub popup: bool,
    pub tick: u64,
}

impl Default for State {
    fn default() -> Self {
        Self {
            selected: 0,
            pot: Vec::new(),
            remaining: INGREDIENTS.map(|(_, quantity)| quantity),
            progress: 0.0,
            popup: false,
            tick: 0,
        }
    }
}

pub type Render = fn(&mut Frame, &State);
pub type Update = fn(&mut State, KeyEvent);

/// Run the kitchen dashboard until the user presses Escape.
///
/// # Errors
///
/// Returns an I/O error if terminal setup, drawing or event handling fails.
pub fn run(render: Render, update: Update) -> std::io::Result<()> {
    if !std::io::stdout().is_terminal() {
        return Ok(());
    }

    ratatui::run(|terminal: &mut DefaultTerminal| {
        let mut state = State::default();
        loop {
            terminal.draw(|frame| render(frame, &state))?;
            state.tick = state.tick.wrapping_add(1);
            if event::poll(Duration::from_millis(120))?
                && let Event::Key(key) = event::read()?
                && key.is_press()
            {
                if key.code == KeyCode::Esc {
                    break Ok(());
                }
                update(&mut state, key);
            }
        }
    })
}
