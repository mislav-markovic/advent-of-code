use crossterm::event;
use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::text::Text;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::List;
use ratatui::widgets::ListItem;
use ratatui::widgets::Paragraph;
use ratatui::Frame;
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
        if self.val >= rhs {
            self.val -= rhs;
        } else {
            self.val = self.wrap_trashold - (rhs - self.val);
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

fn fake_day2() -> String {
    "Executed fake day2".to_string()
}

fn fake_day3() -> String {
    "Executed fake day3".to_string()
}

impl App {
    fn new() -> Self {
        let available_days = vec![
            Day::new("Day 01".to_string(), Box::new(fake_day1)),
            Day::new("Day 02".to_string(), Box::new(fake_day2)),
            Day::new("Day 03".to_string(), Box::new(fake_day3)),
        ];
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

fn run_app(term: &mut MyTerminal, app: &mut App) {
    loop {
        term.draw(|f| ui(f, &app)).expect("terminal to draw frame");
        if event_loop(app) {
            break;
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // list of days
            Constraint::Min(1),    // exec output
        ])
        .split(f.size());

    let title_rect = chunks[0];
    let content_rect = chunks[1];

    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(30), // list of days
            Constraint::Min(1),     // exec output
        ])
        .split(content_rect);
    let list_rect = content_chunks[0];
    let last_exec_rect = content_chunks[1];

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "AoC-23 Day Execution Picker",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    f.render_widget(title, title_rect);

    let list_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let mut list_items = Vec::<ListItem>::new();

    for (idx, day) in app.available_days.iter().enumerate() {
        let day_style = if idx == app.selected_idx.val() {
            Style::default().fg(Color::Yellow).bg(Color::Magenta)
        } else {
            Style::default().fg(Color::Yellow)
        };

        let day_span = Span::styled(day.name.clone(), day_style);
        list_items.push(ListItem::new(Line::from(day_span)));
    }

    let list = List::new(list_items).block(list_block);

    f.render_widget(list, list_rect);

    let last_exec_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let last_exec_content = Paragraph::new(Text::styled(
        app.last_exec_out.clone(),
        Style::default().fg(Color::Green),
    ))
    .block(last_exec_block);

    f.render_widget(last_exec_content, last_exec_rect);
}

fn event_loop(app: &mut App) -> bool {
    loop {
        if let Event::Key(key) = event::read().expect("to read term event") {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }

            match key.code {
                KeyCode::Char('q') => {
                    return true;
                }
                KeyCode::Down => app.move_selection_down(),
                KeyCode::Up => app.move_selection_up(),
                KeyCode::Enter => app.exec_selected(),
                _ => {}
            }
            break;
        }
    }

    false
}

fn main() {
    let mut term = setup();

    let mut app = App::new();
    run_app(&mut term, &mut app);

    teardown(term);
}
