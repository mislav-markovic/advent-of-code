use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::ops::{Add, AddAssign, SubAssign};

type DayFn = Box<dyn Fn() -> String>;
struct Day {
    name: String,
    exec: DayFn,
}

impl Day {
    fn new(name: String, exec: DayFn) -> Self {
        Self { name, exec }
    }
}

struct Cursor {
    val: usize,
    wrap_trashold: usize,
}

impl Cursor {
    fn new(val: usize, wrap_trashold: usize) -> Self {
        Self { val, wrap_trashold }
    }

    fn with_threshold(wrap_treshold: usize) -> Self {
        Self::new(0, wrap_treshold)
    }

    fn val(&self) -> usize {
        self.val
    }
}

impl AddAssign<usize> for Cursor {
    fn add_assign(&mut self, rhs: usize) {
        self.val = (self.val + rhs) % self.wrap_trashold;
    }
}

impl SubAssign<usize> for Cursor {
    fn sub_assign(&mut self, rhs: usize) {
        if self.val > rhs {
            self.val -= rhs;
        } else {
            self.val = self.wrap_trashold - (rhs - self.val - 1);
        }
    }
}

struct App {
    available_days: Vec<Day>,
    selected_idx: Cursor,
    last_exec_out: String,
}

fn fake_day1() -> String {
    "Executed fake day1".to_string()
}
impl App {
    fn new() -> Self {
        let available_days = vec![Day::new("Day 01".to_string(), Box::new(fake_day1))];
        let selected_idx = Cursor::with_threshold(available_days.len());
        let last_exec_out = String::new();
        Self {
            available_days,
            selected_idx,
            last_exec_out,
        }
    }

    fn move_selection_down(&mut self) {
        self.selected_idx += 1;
    }

    fn move_selection_up(&mut self) {
        self.selected_idx -= 1;
    }

    fn exec_selected(&mut self) {
        self.last_exec_out = (self.available_days[self.selected_idx.val()].exec)();
    }
}
type MyTerminal = Terminal<CrosstermBackend<io::Stdout>>;

fn setup() -> MyTerminal {
    enable_raw_mode().expect("raw mode enabled");
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).expect("entered alternate screen");

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("terminal with crossterm backend created");
    terminal
}

fn teardown(mut terminal: MyTerminal) {
    disable_raw_mode().expect("disable raw mode");
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .expect("leave alternate screen");
    terminal.show_cursor().expect("terminal show cursor");
}

fn run_app(term: &mut MyTerminal, app: &mut App) {}

fn main() {
    let mut term = setup();

    teardown(term);
}
