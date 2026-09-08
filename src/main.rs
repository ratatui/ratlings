//! `ratlings view` — render a student's exercise inside a ratlings frame.
//!
//! The exercise owns the whole terminal and every key. One reserved chord
//! (`Ctrl-G`) hands focus to ratlings, which can then overlay test feedback
//! without moving or restyling a single cell of the exercise underneath.

use std::{
    fmt::Write as _,
    io::IsTerminal,
    process::Command,
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};

use ratatui::{
    DefaultTerminal, Frame, Terminal,
    backend::TestBackend,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Flex, Layout, Margin, Rect},
    style::{Color, Style},
    widgets::{Block, Clear, Padding, Paragraph, Wrap},
};

// Exercises are ordinary `[[bin]]` targets, so the viewer includes their
// source directly. The only thing an exercise has to expose is `render`.
#[path = "../exercises/01_packing/pack1.rs"]
#[allow(dead_code)]
mod pack1;
#[path = "../exercises/01_packing/pack2.rs"]
#[allow(dead_code)]
mod pack2;

/// An exercise, as far as the viewer is concerned: a name and a way to draw it.
type Exercise = (&'static str, fn(&mut Frame));

const EXERCISES: [Exercise; 2] = [("pack1", pack1::render), ("pack2", pack2::render)];

// ─────────────────────────────── test runner ───────────────────────────────

#[derive(Default)]
enum Tests {
    #[default]
    Idle,
    Running,
    Done(Report),
}

#[derive(Default)]
struct Report {
    passed: usize,
    failed: usize,
    messages: Vec<String>,
}

/// Run `cargo test` for one exercise off-thread and summarise the failures.
fn spawn_tests(exercise: &'static str) -> Receiver<Report> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let out = Command::new("cargo")
            .args(["test", "--bin", exercise])
            .output();
        let mut report = Report::default();
        if let Ok(out) = out {
            let text = String::from_utf8_lossy(&out.stdout).into_owned()
                + &String::from_utf8_lossy(&out.stderr);
            for line in text.lines() {
                if let Some(rest) = line.strip_prefix("test result: ") {
                    // "ok. 3 passed; 0 failed; …" or "FAILED. 0 passed; 3 failed; …"
                    for part in rest.split(';') {
                        let toks: Vec<&str> = part.split_whitespace().collect();
                        let (Some(count), Some(kind)) = (toks.iter().nth_back(1), toks.last())
                        else {
                            continue;
                        };
                        let n = count.parse().unwrap_or(0);
                        match *kind {
                            "passed" => report.passed += n,
                            "failed" => report.failed += n,
                            _ => {}
                        }
                    }
                }
                // The assertion messages the exercise author wrote.
                if let Some((_, msg)) = line.split_once("failed: ") {
                    report.messages.push(msg.trim().to_string());
                } else if line.contains("buffer contents not equal") {
                    report
                        .messages
                        .push("the rendered output isn't what the exercise expects".into());
                }
            }
        }
        let _ = tx.send(report);
    });
    rx
}

// ─────────────────────────────── the viewer ────────────────────────────────

#[derive(PartialEq, Clone, Copy)]
enum Focus {
    Canvas,
    Ratlings,
}

struct Viewer {
    exercise: &'static str,
    render: fn(&mut Frame),
    focus: Focus,
    panel: bool,
    quit: bool,
    tests: Tests,
    rx: Option<Receiver<Report>>,
}

const AMBER: Color = Color::Rgb(240, 190, 110);

impl Viewer {
    /// Exactly one reserved chord. Every other key belongs to the exercise.
    fn is_prefix(key: KeyEvent) -> bool {
        key.code == KeyCode::Char('g') && key.modifiers.contains(KeyModifiers::CONTROL)
    }

    fn on_key(&mut self, key: KeyEvent) {
        if Self::is_prefix(key) {
            self.focus = match self.focus {
                Focus::Canvas => Focus::Ratlings,
                Focus::Ratlings => Focus::Canvas,
            };
            return;
        }
        match self.focus {
            // The exercise drives itself. A future exercise with its own
            // `update` would receive the key here.
            Focus::Canvas => {}
            Focus::Ratlings => match key.code {
                KeyCode::Char('?') => {
                    self.panel = !self.panel;
                    if self.panel && !matches!(self.tests, Tests::Running) {
                        self.tests = Tests::Running;
                        self.rx = Some(spawn_tests(self.exercise));
                    }
                }
                KeyCode::Char('q') => self.quit = true,
                _ => {}
            },
        }
    }

    fn draw(&self, frame: &mut Frame) {
        // The canvas is the entire terminal, so the exercise's constraints
        // resolve against the real size. Chrome floats; it never reflows.
        canvas(frame, frame.area(), self.render);

        if self.focus == Focus::Ratlings {
            self.draw_chrome(frame);
        }
    }

    fn draw_chrome(&self, frame: &mut Frame) {
        let area = frame.area();

        // Style only the border and title — a `.fg()` on the block itself
        // would repaint every cell of the exercise underneath.
        frame.render_widget(
            Block::bordered()
                .title(format!(" student canvas · {} ", self.exercise))
                .border_style(Style::new().fg(AMBER))
                .title_style(Style::new().fg(AMBER).bold()),
            area,
        );

        let bar = Rect::new(0, area.height - 1, area.width, 1);
        frame.render_widget(Clear, bar);
        frame.render_widget(
            Paragraph::new("  ratlings   ? feedback   q quit   C-g back to the exercise  ")
                .style(Style::new().bg(AMBER).fg(Color::Rgb(30, 26, 22)).bold()),
            bar,
        );

        if self.panel {
            let outer = area.inner(Margin::new(4, 3));
            let [a] = Layout::horizontal([Constraint::Length(46)])
                .flex(Flex::End)
                .areas(outer);
            let [a] = Layout::vertical([Constraint::Length(10)])
                .flex(Flex::End)
                .areas(a);
            frame.render_widget(Clear, a);

            let body = match &self.tests {
                Tests::Idle | Tests::Running => "running cargo test…".to_string(),
                Tests::Done(r) if r.failed == 0 => {
                    format!("{} passed — this one's done ✓", r.passed)
                }
                Tests::Done(r) => {
                    let mut s = format!("{} of {} failing\n", r.failed, r.passed + r.failed);
                    for m in r.messages.iter().take(3) {
                        let _ = write!(s, "\n· {m}");
                    }
                    s
                }
            };
            let tint = match &self.tests {
                Tests::Done(r) if r.failed == 0 => Color::Rgb(170, 220, 160),
                _ => Color::Rgb(245, 190, 180),
            };
            frame.render_widget(
                Paragraph::new(body)
                    .wrap(Wrap { trim: false })
                    .block(
                        Block::bordered()
                            .title(" ratlings · feedback ")
                            .padding(Padding::new(2, 2, 1, 0)),
                    )
                    .style(Style::new().bg(Color::Rgb(38, 30, 28)).fg(tint)),
                a,
            );
        }
    }
}

/// Draw a render fn into an off-screen buffer, then copy the cells into
/// `area`. Full style fidelity — it's a cell copy, not a re-serialisation.
fn canvas(frame: &mut Frame, area: Rect, render: fn(&mut Frame)) {
    // Infallible: `TestBackend` has nothing to fail at.
    let mut inner = Terminal::new(TestBackend::new(area.width, area.height))
        .expect("test backend is infallible");
    if inner.draw(render).is_err() {
        return;
    }
    let src: &Buffer = inner.backend().buffer();
    let dst = frame.buffer_mut();
    for y in 0..area.height {
        for x in 0..area.width {
            dst[(area.x + x, area.y + y)] = src[(x, y)].clone();
        }
    }
}

fn run(terminal: &mut DefaultTerminal, mut viewer: Viewer) -> std::io::Result<()> {
    loop {
        terminal.draw(|f| viewer.draw(f))?;

        if let Some(rx) = &viewer.rx
            && let Ok(report) = rx.try_recv()
        {
            viewer.tests = Tests::Done(report);
            viewer.rx = None;
        }

        if event::poll(Duration::from_millis(120))?
            && let Event::Key(key) = event::read()?
            && key.is_press()
        {
            viewer.on_key(key);
        }

        if viewer.quit {
            return Ok(());
        }
    }
}

fn main() -> std::io::Result<()> {
    let wanted = std::env::args().nth(1);
    let Some(&(exercise, render)) = EXERCISES
        .iter()
        .find(|(name, _)| wanted.as_deref().is_none_or(|w| w == *name))
    else {
        eprintln!(
            "unknown exercise. available: {}",
            EXERCISES.map(|(n, _)| n).join(", ")
        );
        std::process::exit(1);
    };

    if !std::io::stdout().is_terminal() {
        eprintln!("ratlings view needs a terminal");
        return Ok(());
    }

    ratatui::run(|t| {
        run(
            t,
            Viewer {
                exercise,
                render,
                focus: Focus::Canvas,
                panel: false,
                quit: false,
                tests: Tests::default(),
                rx: None,
            },
        )
    })
}
