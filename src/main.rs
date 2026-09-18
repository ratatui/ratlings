//! `ratlings view` — render a student's exercise inside a ratlings frame.
//!
//! The exercise owns the whole terminal and every key. One reserved chord
//! (`Ctrl-G`) hands focus to ratlings, which can then overlay instructions,
//! hints and test feedback without moving or restyling a single cell of the
//! exercise underneath. The viewer opens with the instructions showing, so
//! the first screen is never blank even when the exercise draws nothing yet.

use std::{
    collections::BTreeSet,
    fmt::Write as _,
    io::IsTerminal,
    path::PathBuf,
    process::Command,
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, SystemTime},
};

use ratatui::{
    DefaultTerminal, Frame, Terminal,
    backend::TestBackend,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Flex, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Clear, Padding, Paragraph, Wrap},
};

// ─────────────────────────────── the registry ──────────────────────────────

/// An exercise, as far as the viewer is concerned.
struct Exercise {
    /// The `[[bin]]` name, e.g. `lesson1`. Also the name `cargo test` needs.
    name: &'static str,
    /// What the lesson is called on screen, e.g. `A Bag You Can See`.
    title: &'static str,
    /// Path from `src/`, e.g. `../exercises/01_packing/lesson1.rs`.
    path: &'static str,
    /// The one thing an exercise has to expose.
    render: fn(&mut Frame),
    /// Its source, for the instructions written at the top of the file.
    source: &'static str,
}

impl Exercise {
    /// Lesson 0 is a tour of the viewer itself, so it gets a live checklist.
    fn is_tour(&self) -> bool {
        self.path.contains("/00_intro/")
    }

    /// Where the file lives on disk, for watching it.
    fn file(&self) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(self.path.trim_start_matches("../"))
    }

    fn mtime(&self) -> Option<SystemTime> {
        std::fs::metadata(self.file()).ok()?.modified().ok()
    }
}

/// Exercises are ordinary `[[bin]]` targets, so the viewer includes their
/// source directly. One line per lesson, in course order.
///
/// Under `cfg(test)` the real modules are swapped for stubs, so the viewer's
/// own tests don't drag in every unsolved exercise's failing tests.
macro_rules! exercises {
    ($($name:ident => $title:literal, $path:literal),* $(,)?) => {
        $(
            #[cfg(not(test))]
            #[path = $path]
            #[allow(dead_code)]
            mod $name;
            #[cfg(test)]
            mod $name {
                pub fn render(_: &mut ratatui::Frame) {}
            }
        )*
        const EXERCISES: &[Exercise] = &[
            $(Exercise {
                name: stringify!($name),
                title: $title,
                path: $path,
                render: $name::render,
                source: include_str!($path),
            },)*
        ];
    };
}

exercises! {
    lesson0 => "Welcome to Ratlings", "../exercises/00_intro/lesson0.rs",
    lesson1 => "A Bag You Can See", "../exercises/01_packing/lesson1.rs",
    lesson2 => "Room for Everything", "../exercises/01_packing/lesson2.rs",
}

/// `lesson1` is a fine binary name and a poor title. Show it as `Lesson 1`.
fn display_name(name: &str) -> String {
    let split = name.trim_end_matches(|c: char| c.is_ascii_digit()).len();
    let (word, number) = name.split_at(split);
    let mut out = String::new();
    let mut chars = word.chars();
    if let Some(first) = chars.next() {
        out.extend(first.to_uppercase());
        out.push_str(chars.as_str());
    }
    if !number.is_empty() {
        out.push(' ');
        out.push_str(number);
    }
    out
}

/// Hints live in `info.toml`, exactly where `rustlings` reads them from.
const INFO: &str = include_str!("../info.toml");

// ──────────────────────────────── the lesson ───────────────────────────────

/// The words around an exercise: the story at the top of the file, the TODO
/// inside `render`, and the hint from `info.toml`.
struct Lesson {
    instructions: String,
    hint: String,
}

impl Lesson {
    fn for_exercise(name: &str, source: &str) -> Self {
        Self {
            instructions: instructions(source),
            hint: hint(name).unwrap_or_else(|| "No hint for this one — you've got this.".into()),
        }
    }
}

/// The leading `//` block of the file, then the first `// TODO` block. Blank
/// comment lines become paragraph breaks.
fn instructions(source: &str) -> String {
    let comment = |l: &str| {
        l.trim_start()
            .strip_prefix("//")
            .map(|r| r.strip_prefix(' ').unwrap_or(r).to_string())
    };
    let mut lines = source.lines().peekable();

    let mut story: Vec<String> = Vec::new();
    while let Some(text) = lines.peek().and_then(|l| comment(l)) {
        story.push(text);
        lines.next();
    }

    let mut todo: Vec<String> = Vec::new();
    let mut rest = lines.skip_while(|l| !l.contains("// TODO"));
    while let Some(text) = rest.next().and_then(comment) {
        todo.push(text);
    }

    let mut out = unwrap_paragraphs(&story);
    if !todo.is_empty() {
        out.push_str("\n\n");
        out.push_str(&unwrap_paragraphs(&todo));
    }
    out
}

/// Join hard-wrapped comment lines so `Paragraph` can re-wrap to the panel.
/// Indented lines are preformatted (a key table, say) and keep their row.
fn unwrap_paragraphs(lines: &[String]) -> String {
    let mut out = String::new();
    let mut prev_pre = false;
    for l in lines {
        let pre = l.starts_with(' ');
        if l.trim().is_empty() {
            out.push_str("\n\n");
        } else {
            if !out.is_empty() && !out.ends_with('\n') {
                out.push(if pre || prev_pre { '\n' } else { ' ' });
            }
            out.push_str(l.trim_end());
        }
        prev_pre = pre;
    }
    out
}

/// Pull `hint = """…"""` for `name` out of `info.toml`. The file is ours and
/// its shape is fixed, so a scan is enough — no TOML crate needed.
fn hint(name: &str) -> Option<String> {
    let start = INFO.find(&format!("name = \"{name}\""))?;
    let after = &INFO[start..];
    let body = after.split_once("hint = \"\"\"")?.1;
    let body = body.split_once("\"\"\"")?.0;
    let lines: Vec<String> = body.lines().map(str::to_string).collect();
    Some(unwrap_paragraphs(&lines).trim().to_string())
}

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

impl Report {
    /// Green means something ran and nothing failed. A run that produced no
    /// results at all (the file didn't compile, say) is not a pass.
    fn ok(&self) -> bool {
        self.failed == 0 && self.passed > 0
    }
}

/// Run `cargo test` for one exercise off-thread and summarise the failures.
fn spawn_tests(exercise: &'static str) -> Receiver<Report> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let out = Command::new("cargo")
            .current_dir(env!("CARGO_MANIFEST_DIR"))
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
                // A compile error means no tests ran; say why.
                if line.starts_with("error") && !line.starts_with("error: test failed") {
                    report.messages.push(line.trim().to_string());
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

// ─────────────────────────────── live reload ───────────────────────────────

/// The exercise is compiled *into* the viewer, so after the student saves,
/// the canvas can only change by rebuilding the viewer and starting it again.
/// A poll on the file's mtime kicks that off; `main` finishes the job by
/// exec-ing the fresh binary with the session state in an env var.
#[derive(Default)]
enum Build {
    #[default]
    Idle,
    Running,
    /// `cargo build` failed — usually the student's code doesn't compile yet.
    Failed(String),
}

fn spawn_build() -> Receiver<Result<(), String>> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let out = Command::new("cargo")
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .args(["build", "--bin", "ratlings", "--color", "never"])
            .output();
        let result = match out {
            Ok(o) if o.status.success() => Ok(()),
            Ok(o) => Err(first_error(&String::from_utf8_lossy(&o.stderr))),
            Err(e) => Err(e.to_string()),
        };
        let _ = tx.send(result);
    });
    rx
}

/// The first `error…` block from rustc, up to the first blank line.
fn first_error(stderr: &str) -> String {
    let mut lines = stderr.lines().skip_while(|l| !l.starts_with("error"));
    let mut out = String::new();
    for l in lines.by_ref().take_while(|l| !l.trim().is_empty()) {
        if !out.is_empty() {
            out.push('\n');
        }
        out.push_str(l.trim_end());
    }
    if out.is_empty() {
        "the build failed".into()
    } else {
        out
    }
}

/// Everything worth carrying across a restart, as one env var.
const RESUME: &str = "RATLINGS_RESUME";

// ──────────────────────────────── progress ─────────────────────────────────

/// Which lessons have passed, one name per line in a gitignored file at the
/// repo root. Written the moment a lesson's tests go green.
struct Progress {
    done: BTreeSet<String>,
}

impl Progress {
    fn file() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".ratlings-progress")
    }

    fn load() -> Self {
        let text = std::fs::read_to_string(Self::file()).unwrap_or_default();
        Self::parse(&text)
    }

    fn parse(text: &str) -> Self {
        Self {
            done: text
                .lines()
                .map(str::trim)
                .filter(|l| !l.is_empty() && !l.starts_with('#'))
                .map(String::from)
                .collect(),
        }
    }

    fn is_done(&self, name: &str) -> bool {
        self.done.contains(name)
    }

    fn mark_done(&mut self, name: &str) {
        if self.done.insert(name.to_string()) {
            let mut out = String::from("# ratlings progress — one finished lesson per line\n");
            for n in &self.done {
                out.push_str(n);
                out.push('\n');
            }
            // Best effort: losing progress is a nuisance, not an error.
            let _ = std::fs::write(Self::file(), out);
        }
    }

    /// The first lesson not yet passed, or the last one if all are.
    fn next_index(&self) -> usize {
        EXERCISES
            .iter()
            .position(|e| !self.is_done(e.name))
            .unwrap_or(EXERCISES.len() - 1)
    }
}

// ──────────────────────────────── the tour ─────────────────────────────────

/// What the student has tried so far. Lesson 0 shows this as a checklist
/// that ticks off live; every other lesson ignores it.
#[derive(Default)]
#[allow(clippy::struct_excessive_bools)] // it *is* a checklist
struct Tour {
    closed_pane: bool,
    to_canvas: bool,
    back_to_ratlings: bool,
    hint: bool,
    ran_tests: bool,
    passed: bool,
}

impl Tour {
    fn encode(&self) -> String {
        [
            self.closed_pane,
            self.to_canvas,
            self.back_to_ratlings,
            self.hint,
            self.ran_tests,
            self.passed,
        ]
        .iter()
        .map(|b| if *b { '1' } else { '0' })
        .collect()
    }

    fn decode(s: &str) -> Self {
        let b: Vec<bool> = s.chars().map(|c| c == '1').collect();
        let at = |i: usize| b.get(i).copied().unwrap_or(false);
        Self {
            closed_pane: at(0),
            to_canvas: at(1),
            back_to_ratlings: at(2),
            hint: at(3),
            ran_tests: at(4),
            passed: at(5),
        }
    }

    fn checklist(&self) -> String {
        let items = [
            (
                self.closed_pane,
                "esc",
                "close this pane and look at the canvas",
            ),
            (self.to_canvas, "C-g", "hand the keyboard to your app"),
            (self.back_to_ratlings, "C-g", "bring ratlings back"),
            (self.hint, "h", "read the hint"),
            (self.ran_tests, "?", "run the tests"),
            (
                self.passed,
                "",
                "make them pass (edit the file, then `?` again)",
            ),
        ];
        let mut out = String::from("Your tour:");
        for (done, key, what) in items {
            let mark = if done { "✓" } else { "○" };
            // Pad outside the backticks so the markup stays a clean run.
            let pad = " ".repeat(4usize.saturating_sub(key.chars().count()));
            let key = if key.is_empty() {
                String::new()
            } else {
                format!("`{key}`")
            };
            let _ = write!(out, "\n {mark} {key}{pad} {what}");
        }
        out
    }
}

// ─────────────────────────────── the viewer ────────────────────────────────

#[derive(PartialEq, Clone, Copy)]
enum Focus {
    Canvas,
    Ratlings,
}

/// Which overlay is open while ratlings has focus. At most one at a time.
#[derive(Debug, PartialEq, Clone, Copy)]
enum Panel {
    None,
    Instructions,
    Hint,
    Feedback,
}

struct Viewer {
    /// Index into `EXERCISES`.
    index: usize,
    lesson: Lesson,
    tour: Tour,
    focus: Focus,
    panel: Panel,
    /// Rows scrolled in the reading pane. Reset whenever a panel opens.
    scroll: u16,
    quit: bool,
    tests: Tests,
    rx: Option<Receiver<Report>>,
    /// The exercise file's mtime when we last looked.
    seen: Option<SystemTime>,
    build: Build,
    build_rx: Option<Receiver<Result<(), String>>>,
    /// Set once a rebuild succeeds: `run` returns and `main` execs the new binary.
    restart: bool,
    progress: Progress,
}

const AMBER: Color = Color::Rgb(240, 190, 110);
/// Inline code and key names. Deliberately not the border's amber, so keys
/// stand out from the chrome around them.
const CODE: Color = Color::Rgb(130, 200, 220);

impl Viewer {
    /// Open on `index` with the instructions showing — never on a possibly
    /// empty canvas.
    fn open(index: usize) -> Self {
        let ex = &EXERCISES[index];
        // Check quietly on the way in. If the student solved this lesson while
        // the viewer was closed, the pass arrives in a moment and gets its
        // celebration; otherwise nothing visible happens.
        let rx = (!cfg!(test)).then(|| spawn_tests(ex.name));
        Self {
            index,
            lesson: Lesson::for_exercise(ex.name, ex.source),
            tour: Tour::default(),
            focus: Focus::Ratlings,
            panel: Panel::Instructions,
            scroll: 0,
            quit: false,
            tests: if rx.is_some() {
                Tests::Running
            } else {
                Tests::Idle
            },
            rx,
            seen: ex.mtime(),
            build: Build::default(),
            build_rx: None,
            restart: false,
            progress: Progress::load(),
        }
    }

    /// Pick up where a previous process left off. Format: `index;tour;panel`.
    fn resume(state: &str) -> Option<Self> {
        let mut parts = state.split(';');
        let index: usize = parts.next()?.parse().ok()?;
        EXERCISES.get(index)?;
        let tour = Tour::decode(parts.next()?);
        let panel = match parts.next()? {
            "i" => Panel::Instructions,
            "h" => Panel::Hint,
            "f" => Panel::Feedback,
            _ => Panel::None,
        };
        // `open` already kicked off a test run, which is exactly what the
        // student wants after a save: to see where they stand.
        let mut v = Self::open(index);
        v.tour = tour;
        v.panel = panel;
        Some(v)
    }

    fn state(&self) -> String {
        let panel = match self.panel {
            Panel::Instructions => "i",
            Panel::Hint => "h",
            Panel::Feedback => "f",
            Panel::None => "-",
        };
        format!("{};{};{panel}", self.index, self.tour.encode())
    }

    /// Called every tick. Starts a rebuild when the file changes, and
    /// collects the result when one finishes.
    fn watch(&mut self) {
        let now = self.exercise().mtime();
        if now != self.seen && !matches!(self.build, Build::Running) {
            self.seen = now;
            self.build = Build::Running;
            self.build_rx = Some(spawn_build());
        }
        if let Some(rx) = &self.build_rx
            && let Ok(result) = rx.try_recv()
        {
            self.build_rx = None;
            match result {
                Ok(()) => self.restart = true,
                Err(e) => {
                    self.build = Build::Failed(e);
                    // Surface the compile error where the tests would go.
                    self.focus = Focus::Ratlings;
                    self.panel = Panel::Feedback;
                }
            }
        }
    }

    fn exercise(&self) -> &'static Exercise {
        &EXERCISES[self.index]
    }

    fn passed(&self) -> bool {
        matches!(&self.tests, Tests::Done(r) if r.ok())
    }

    fn has_next(&self) -> bool {
        self.index + 1 < EXERCISES.len()
    }

    /// Exactly one reserved chord. Every other key belongs to the exercise.
    fn is_prefix(key: KeyEvent) -> bool {
        key.code == KeyCode::Char('g') && key.modifiers.contains(KeyModifiers::CONTROL)
    }

    fn on_key(&mut self, key: KeyEvent) {
        if Self::is_prefix(key) {
            self.focus = match self.focus {
                Focus::Canvas => {
                    self.tour.back_to_ratlings = true;
                    Focus::Ratlings
                }
                Focus::Ratlings => {
                    self.tour.to_canvas = true;
                    Focus::Canvas
                }
            };
            return;
        }
        match self.focus {
            // The exercise drives itself. A future exercise with its own
            // `update` would receive the key here.
            Focus::Canvas => {}
            Focus::Ratlings => match key.code {
                KeyCode::Char('i') => self.toggle(Panel::Instructions),
                KeyCode::Char('h') => {
                    self.tour.hint = true;
                    self.toggle(Panel::Hint);
                }
                KeyCode::Char('?') => {
                    self.tour.ran_tests = true;
                    self.toggle(Panel::Feedback);
                    if self.panel == Panel::Feedback && !matches!(self.tests, Tests::Running) {
                        self.tests = Tests::Running;
                        self.rx = Some(spawn_tests(self.exercise().name));
                    }
                }
                KeyCode::Char('n') if self.passed() && self.has_next() => {
                    *self = Self::open(self.index + 1);
                }
                KeyCode::Char('j') | KeyCode::Down => self.scroll = self.scroll.saturating_add(1),
                KeyCode::Char('k') | KeyCode::Up => self.scroll = self.scroll.saturating_sub(1),
                KeyCode::Esc => self.toggle(Panel::None),
                KeyCode::Char('q') => self.quit = true,
                _ => {}
            },
        }
    }

    fn toggle(&mut self, panel: Panel) {
        if self.panel == Panel::Instructions {
            self.tour.closed_pane = true;
        }
        self.panel = if self.panel == panel {
            Panel::None
        } else {
            panel
        };
        self.scroll = 0;
    }

    fn draw(&self, frame: &mut Frame) {
        // The canvas is the entire terminal, so the exercise's constraints
        // resolve against the real size. Chrome floats; it never reflows.
        canvas(frame, frame.area(), self.exercise().render);

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
                .title(format!(
                    " student canvas · {}{} ",
                    display_name(self.exercise().name),
                    if self.progress.is_done(self.exercise().name) {
                        " ✓"
                    } else {
                        ""
                    }
                ))
                .border_style(Style::new().fg(AMBER))
                .title_style(Style::new().fg(AMBER).bold()),
            area,
        );

        let bar = Rect::new(0, area.height - 1, area.width, 1);
        frame.render_widget(Clear, bar);
        let next = if self.passed() && self.has_next() {
            "n next lesson   "
        } else {
            ""
        };
        let building = if matches!(self.build, Build::Running) {
            "rebuilding…   "
        } else {
            ""
        };
        frame.render_widget(
            Paragraph::new(format!(
                "  ratlings   i instructions   h hint   ? feedback   {next}{building}q quit   C-g back to the exercise  "
            ))
            .style(Style::new().bg(AMBER).fg(Color::Rgb(30, 26, 22)).bold()),
            bar,
        );

        match self.panel {
            Panel::None => {}
            Panel::Instructions => {
                let mut text = String::new();
                if self.exercise().is_tour() {
                    text.push_str(&self.tour.checklist());
                    text.push_str("\n\n");
                }
                text.push_str(&self.lesson.instructions);
                self.draw_reading(
                    frame,
                    &format!(
                        " ratlings · {} · {} ",
                        display_name(self.exercise().name),
                        self.exercise().title
                    ),
                    &text,
                );
            }
            Panel::Hint => self.draw_reading(frame, " ratlings · hint ", &self.lesson.hint),
            Panel::Feedback => self.draw_feedback(frame),
        }
    }

    /// A centred reading pane for prose: instructions or a hint.
    fn draw_reading(&self, frame: &mut Frame, title: &str, text: &str) {
        let area = frame.area();
        let outer = area.inner(Margin::new(2, 2));
        let width = outer.width.min(72);
        let [a] = Layout::horizontal([Constraint::Length(width)])
            .flex(Flex::Center)
            .areas(outer);
        // Size to the wrapped text, plus border and padding, capped to the screen.
        let inner_w = width.saturating_sub(2 + 4) as usize;
        let paragraph = Paragraph::new(markup(text)).wrap(Wrap { trim: false });
        // A line of `─` in the source is a rule between story and code; draw
        // it exactly as wide as the pane whatever length the author typed.
        let rule = "─".repeat(inner_w);
        let text: String = text
            .lines()
            .map(|l| if is_rule(l) { rule.as_str() } else { l })
            .collect::<Vec<_>>()
            .join("\n");
        let text = text.as_str();
        // Estimate on what will actually be drawn: markup markers take no room.
        let plain: String = markup(text)
            .lines
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n");
        let lines = u16::try_from(wrapped_lines(&plain, inner_w)).unwrap_or(u16::MAX);
        let height = (lines + 2 + 2).min(outer.height);
        let [a] = Layout::vertical([Constraint::Length(height)])
            .flex(Flex::Center)
            .areas(a);
        // Only scroll as far as there is text hidden below the pane.
        let visible = height.saturating_sub(2 + 2);
        let scroll = self.scroll.min(lines.saturating_sub(visible));
        let footer = if lines > visible {
            " j/k scroll · esc close "
        } else {
            " esc close "
        };
        frame.render_widget(Clear, a);
        frame.render_widget(
            paragraph
                .scroll((scroll, 0))
                .block(
                    Block::bordered()
                        .title(title)
                        .title_bottom(footer)
                        .border_style(Style::new().fg(AMBER))
                        .padding(Padding::new(2, 2, 1, 1)),
                )
                .style(
                    Style::new()
                        .bg(Color::Rgb(38, 30, 28))
                        .fg(Color::Rgb(235, 225, 210)),
                ),
            a,
        );
    }

    fn draw_feedback(&self, frame: &mut Frame) {
        let area = frame.area();
        {
            let outer = area.inner(Margin::new(4, 3));
            let [a] = Layout::horizontal([Constraint::Length(46)])
                .flex(Flex::End)
                .areas(outer);
            let rows = if matches!(self.build, Build::Failed(_)) {
                16
            } else {
                10
            };
            let [a] = Layout::vertical([Constraint::Length(rows)])
                .flex(Flex::End)
                .areas(a);
            frame.render_widget(Clear, a);

            let body = match (&self.build, &self.tests) {
                (Build::Failed(e), _) => format!("this file doesn't compile yet:\n\n{e}"),
                (_, Tests::Idle | Tests::Running) => "running `cargo test`…".to_string(),
                (_, Tests::Done(r)) if r.ok() && self.has_next() => {
                    format!(
                        "{} passed — this one's done ✓\n\npress `n` for the next lesson",
                        r.passed
                    )
                }
                (_, Tests::Done(r)) if r.ok() => {
                    format!("{} passed — that was the last one 🎉", r.passed)
                }
                (_, Tests::Done(r)) if r.passed + r.failed == 0 => {
                    let mut s = String::from("the tests couldn't run\n");
                    for m in r.messages.iter().take(3) {
                        let _ = write!(s, "\n· {m}");
                    }
                    s
                }
                (_, Tests::Done(r)) => {
                    let mut s = format!("{} of {} failing\n", r.failed, r.passed + r.failed);
                    for m in r.messages.iter().take(3) {
                        let _ = write!(s, "\n· {m}");
                    }
                    s
                }
            };
            let tint = match (&self.build, &self.tests) {
                (Build::Failed(_), _) => Color::Rgb(245, 190, 180),
                (_, Tests::Done(r)) if r.ok() => Color::Rgb(170, 220, 160),
                _ => Color::Rgb(245, 190, 180),
            };
            frame.render_widget(
                Paragraph::new(markup(&body))
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

/// The two bits of inline markup lesson prose uses: `_italics_` and
/// `` `code` ``. Anything else is plain text.
fn markup(text: &str) -> Text<'static> {
    text.lines().map(markup_line).collect::<Vec<_>>().into()
}

fn markup_line(line: &str) -> Line<'static> {
    let mut spans = Vec::new();
    let mut plain = String::new();
    let mut rest = line;
    while let Some(i) = rest.find(['_', '`']) {
        let marker = rest.as_bytes()[i] as char;
        // A marker only opens a run if there is a matching close later on
        // and it isn't buried inside a word like `render_widget`.
        let opens = i == 0 || !rest.as_bytes()[i - 1].is_ascii_alphanumeric();
        let close = rest[i + 1..].find(marker).map(|j| i + 1 + j);
        match (opens, close) {
            (true, Some(j)) if j > i + 1 => {
                plain.push_str(&rest[..i]);
                if !plain.is_empty() {
                    spans.push(Span::raw(std::mem::take(&mut plain)));
                }
                let inner = rest[i + 1..j].to_string();
                spans.push(match marker {
                    '_' => Span::raw(inner).italic(),
                    _ => Span::raw(inner).fg(CODE).bold(),
                });
                rest = &rest[j + 1..];
            }
            _ => {
                plain.push_str(&rest[..=i]);
                rest = &rest[i + 1..];
            }
        }
    }
    plain.push_str(rest);
    if !plain.is_empty() {
        spans.push(Span::raw(plain));
    }
    Line::from(spans)
}

/// Three or more `─` and nothing else.
fn is_rule(line: &str) -> bool {
    let t = line.trim();
    t.chars().count() >= 3 && t.chars().all(|c| c == '─')
}

/// Roughly how many rows `text` takes when word-wrapped to `width`. Close
/// enough to size a panel; `Paragraph` does the real wrapping.
fn wrapped_lines(text: &str, width: usize) -> usize {
    let width = width.max(1);
    text.lines()
        .map(|line| {
            let mut rows = 1;
            let mut col = 0;
            for word in line.split_whitespace() {
                let w = word.chars().count();
                if col > 0 && col + 1 + w > width {
                    rows += 1;
                    col = 0;
                }
                if col > 0 {
                    col += 1;
                }
                // A word wider than the pane (a URL, say) is broken mid-word.
                if w > width {
                    rows += (w - 1) / width;
                    col = (w - 1) % width + 1;
                } else {
                    col += w;
                }
            }
            rows
        })
        .sum()
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

/// Returns the state to resume from if the viewer should restart, `None` on quit.
fn run(terminal: &mut DefaultTerminal, mut viewer: Viewer) -> std::io::Result<Option<String>> {
    loop {
        terminal.draw(|f| viewer.draw(f))?;
        viewer.watch();
        if viewer.restart {
            return Ok(Some(viewer.state()));
        }

        if let Some(rx) = &viewer.rx
            && let Ok(report) = rx.try_recv()
        {
            if report.ok() {
                viewer.tour.passed = true;
                let name = viewer.exercise().name;
                if !viewer.progress.is_done(name) {
                    // First time green: make sure they see it.
                    viewer.progress.mark_done(name);
                    viewer.focus = Focus::Ratlings;
                    viewer.panel = Panel::Feedback;
                }
            }
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
            return Ok(None);
        }
    }
}

fn main() -> std::io::Result<()> {
    let wanted = std::env::args().nth(1);
    let index = match wanted.as_deref() {
        None => Progress::load().next_index(),
        Some(w) => {
            let Some(i) = EXERCISES.iter().position(|e| e.name == w) else {
                let names: Vec<&str> = EXERCISES.iter().map(|e| e.name).collect();
                eprintln!("unknown exercise. available: {}", names.join(", "));
                std::process::exit(1);
            };
            i
        }
    };

    if !std::io::stdout().is_terminal() {
        eprintln!("ratlings view needs a terminal");
        return Ok(());
    }

    let viewer = std::env::var(RESUME)
        .ok()
        .and_then(|s| Viewer::resume(&s))
        .unwrap_or_else(|| Viewer::open(index));

    let resume = ratatui::run(|t| run(t, viewer))?;
    if let Some(state) = resume {
        restart(&state)?;
    }
    Ok(())
}

/// Replace this process with the freshly built viewer, same args, plus state.
fn restart(state: &str) -> std::io::Result<()> {
    let exe = std::env::current_exe()?;
    let mut cmd = Command::new(exe);
    cmd.args(std::env::args().skip(1)).env(RESUME, state);
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        Err(cmd.exec())
    }
    #[cfg(not(unix))]
    {
        let status = cmd.status()?;
        std::process::exit(status.code().unwrap_or(0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instructions_take_the_story_and_the_todo() {
        let src = "// Once upon a time.\n// A rat.\n//\n// Second paragraph.\n\nuse x;\n\nfn render() {\n    // TODO: do the thing\n    // on two lines.\n    let y = 1;\n}\n";
        assert_eq!(
            instructions(src),
            "Once upon a time. A rat.\n\nSecond paragraph.\n\nTODO: do the thing on two lines."
        );
    }

    #[test]
    fn every_exercise_has_a_hint_in_info_toml() {
        for ex in EXERCISES {
            let h = hint(ex.name).unwrap_or_default();
            assert!(!h.is_empty(), "{} has no hint", ex.name);
        }
    }

    #[test]
    fn the_viewer_opens_on_the_instructions() {
        let viewer = Viewer::open(1);
        let mut t = Terminal::new(TestBackend::new(90, 30)).unwrap();
        t.draw(|f| viewer.draw(f)).unwrap();
        let screen = t.backend().to_string();
        assert!(screen.contains("Lesson 1 · A Bag You Can See"));
        let v0 = Viewer::open(0);
        let mut t0 = Terminal::new(TestBackend::new(100, 40)).unwrap();
        t0.draw(|f| v0.draw(f)).unwrap();
        let screen0 = t0.backend().to_string();
        assert!(screen0.contains("Your tour:"));
        assert!(screen0.contains("○ esc"));
        assert!(screen.contains("i instructions"));
        assert!(screen.contains("TODO"));
    }

    fn press(v: &mut Viewer, code: KeyCode, mods: KeyModifiers) {
        v.on_key(KeyEvent::new(code, mods));
    }

    #[test]
    fn the_tour_ticks_off_keys_as_they_are_used() {
        let mut v = Viewer::open(0);
        assert!(v.exercise().is_tour());
        assert!(!v.tour.closed_pane);
        press(&mut v, KeyCode::Esc, KeyModifiers::NONE);
        press(&mut v, KeyCode::Char('g'), KeyModifiers::CONTROL);
        press(&mut v, KeyCode::Char('g'), KeyModifiers::CONTROL);
        press(&mut v, KeyCode::Char('h'), KeyModifiers::NONE);
        assert!(v.tour.closed_pane && v.tour.to_canvas && v.tour.back_to_ratlings && v.tour.hint);
        assert!(!v.tour.ran_tests);
        let list = v.tour.checklist();
        assert!(list.contains("✓ `esc`") && list.contains("○ `?`"));
    }

    #[test]
    fn n_moves_on_only_once_the_tests_pass() {
        let mut v = Viewer::open(0);
        press(&mut v, KeyCode::Char('n'), KeyModifiers::NONE);
        assert_eq!(v.index, 0);
        v.tests = Tests::Done(Report {
            passed: 1,
            failed: 0,
            messages: vec![],
        });
        press(&mut v, KeyCode::Char('n'), KeyModifiers::NONE);
        assert_eq!(v.index, 1);
        assert_eq!(v.panel, Panel::Instructions);
        let mut t = Terminal::new(TestBackend::new(90, 30)).unwrap();
        t.draw(|f| v.draw(f)).unwrap();
        assert!(t.backend().to_string().contains("Lesson 1"));
    }

    #[test]
    fn display_names_read_like_titles() {
        assert_eq!(display_name("lesson1"), "Lesson 1");
        assert_eq!(display_name("lesson12"), "Lesson 12");
        assert_eq!(display_name("intro"), "Intro");
    }

    #[test]
    fn markup_styles_italics_and_code_but_not_snake_case() {
        let line = markup_line("No, _they're going camping_! Call `render_widget` on a_b.");
        let texts: Vec<&str> = line.spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(
            texts,
            [
                "No, ",
                "they're going camping",
                "! Call ",
                "render_widget",
                " on a_b."
            ]
        );
        assert!(
            line.spans[1]
                .style
                .add_modifier
                .contains(ratatui::style::Modifier::ITALIC)
        );
        assert_eq!(line.spans[3].style.fg, Some(CODE));
    }

    #[test]
    fn indented_comment_lines_stay_on_their_own_rows() {
        let lines: Vec<String> = ["Keys:", "  esc  close", "  C-g  swap", "Done."]
            .map(String::from)
            .into();
        assert_eq!(
            unwrap_paragraphs(&lines),
            "Keys:\n  esc  close\n  C-g  swap\nDone."
        );
    }

    #[test]
    fn state_survives_a_restart() {
        let mut v = Viewer::open(0);
        press(&mut v, KeyCode::Esc, KeyModifiers::NONE);
        press(&mut v, KeyCode::Char('h'), KeyModifiers::NONE);
        let state = v.state();
        assert_eq!(state, "0;100100;h");
        let r = Viewer::resume(&state).unwrap();
        assert_eq!(r.index, 0);
        assert_eq!(r.panel, Panel::Hint);
        assert!(r.tour.closed_pane && r.tour.hint && !r.tour.to_canvas);
        assert!(Viewer::resume("99;0;i").is_none());
        assert!(Viewer::resume("garbage").is_none());
    }

    #[test]
    fn first_error_takes_the_first_rustc_block() {
        let err = "   Compiling x\nerror[E0425]: cannot find value `NAM`\n --> src/x.rs:3:1\n  |\n\nerror: aborting\n";
        assert_eq!(
            first_error(err),
            "error[E0425]: cannot find value `NAM`\n --> src/x.rs:3:1\n  |"
        );
    }

    #[test]
    fn progress_picks_the_first_unfinished_lesson() {
        assert_eq!(Progress::parse("").next_index(), 0);
        assert_eq!(Progress::parse("# note\nlesson0\n").next_index(), 1);
        assert_eq!(Progress::parse("lesson1\n").next_index(), 0);
        let all: String = EXERCISES
            .iter()
            .map(|e| e.name)
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(Progress::parse(&all).next_index(), EXERCISES.len() - 1);
    }

    #[test]
    fn an_empty_run_is_not_a_pass() {
        assert!(!Report::default().ok());
        assert!(
            Report {
                passed: 2,
                failed: 0,
                messages: vec![]
            }
            .ok()
        );
        assert!(
            !Report {
                passed: 2,
                failed: 1,
                messages: vec![]
            }
            .ok()
        );
    }

    #[test]
    fn rules_are_only_box_drawing_dashes() {
        assert!(is_rule("──────"));
        assert!(is_rule("  ─────  "));
        assert!(!is_rule("──"));
        assert!(!is_rule("── hi ──"));
    }

    #[test]
    fn wrapping_counts_rows() {
        assert_eq!(wrapped_lines("aaa bbb ccc", 7), 2);
        assert_eq!(wrapped_lines("one\n\ntwo", 80), 3);
        assert_eq!(
            wrapped_lines("Docs: https://example.com/a/very/long/path", 20),
            3
        );
        assert_eq!(wrapped_lines("abcdefghij", 5), 2);
    }
}
