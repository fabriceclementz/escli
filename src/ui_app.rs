use anyhow::Result;
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind,
};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Span, Spans, Text};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState, Tabs};
use ratatui::{Frame, Terminal};
use std::time::{Duration, Instant};

const LAYOUT_NORMAL_MODE: [Constraint; 3] = [
    Constraint::Percentage(5),
    Constraint::Percentage(90),
    Constraint::Percentage(5),
];

const LAYOUT_FILTER_MODE: [Constraint; 4] = [
    Constraint::Percentage(5),
    Constraint::Percentage(5),
    Constraint::Percentage(85),
    Constraint::Percentage(5),
];

const INDICES_TABLE_HEADER: [&str; 7] = [
    "NAME",
    "STATUS",
    "HEALTH",
    "PRI",
    "REP",
    "STORE SIZE",
    "DOCS",
];

const INDICES_TABLE_WIDTH: [Constraint; 7] = [
    Constraint::Percentage(40),
    Constraint::Percentage(10),
    Constraint::Percentage(10),
    Constraint::Percentage(5),
    Constraint::Percentage(10),
    Constraint::Percentage(10),
    Constraint::Percentage(15),
];

/// This struct holds the current state of the app.
pub struct UiApp {
    selected_screen_idx: usize,
    /// Current value of the filter indices input
    indices_filter: String,
    input_mode: InputMode,
    indices: StatefulTable<String>,
}

#[derive(Debug, PartialEq)]
enum InputMode {
    Normal,
    Filtering,
}

impl UiApp {
    pub fn new() -> Self {
        let indices: Vec<String> = (0..=10).map(|i| format!("Indice {i}")).collect();
        Self {
            indices: StatefulTable::with_items(indices),
            indices_filter: String::new(),
            input_mode: InputMode::Normal,
            selected_screen_idx: 0,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();

        loop {
            terminal.draw(|f| {
                draw_ui(f, self);
            })?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match self.input_mode {
                        InputMode::Normal => {
                            if key.kind == KeyEventKind::Press {
                                match key.code {
                                    KeyCode::Char('q') => break,
                                    KeyCode::Char('f') => self.input_mode = InputMode::Filtering,
                                    KeyCode::Left => self.indices.unselect(),
                                    KeyCode::Down => self.indices.next(),
                                    KeyCode::Up => self.indices.previous(),
                                    _ => {}
                                }
                            }
                        }
                        InputMode::Filtering => {
                            if key.kind == KeyEventKind::Press {
                                match key.code {
                                    KeyCode::Esc => self.input_mode = InputMode::Normal,
                                    KeyCode::Char(c) => {
                                        self.indices_filter.push(c);
                                    }
                                    KeyCode::Backspace => {
                                        self.indices_filter.pop();
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;

        Ok(())
    }
}

impl Default for UiApp {
    fn default() -> Self {
        Self::new()
    }
}

struct StatefulTable<T> {
    state: TableState,
    items: Vec<T>,
}

impl<T> StatefulTable<T> {
    fn with_items(items: Vec<T>) -> StatefulTable<T> {
        StatefulTable {
            state: TableState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

fn draw_ui<B: Backend>(f: &mut Frame<B>, app: &mut UiApp) {
    let constraints = if app.input_mode == InputMode::Normal {
        LAYOUT_NORMAL_MODE.as_slice()
    } else {
        LAYOUT_FILTER_MODE.as_slice()
    };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints.as_ref())
        // .margin(2)
        .split(f.size());

    draw_header(f, app, chunks[0]);
    match app.input_mode {
        InputMode::Normal => {
            draw_indices_table(f, app, chunks[1]);
            draw_help(f, app, chunks[2]);
        }
        InputMode::Filtering => {
            draw_input_filter(f, app, chunks[1]);
            draw_indices_table(f, app, chunks[2]);
            draw_help(f, app, chunks[3]);
        }
    };
}

fn draw_header<B: Backend>(f: &mut Frame<B>, app: &mut UiApp, rect: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
        .split(rect);

    draw_tabs(f, app, chunks[0]);
    draw_infos(f, app, chunks[1]);
}

fn draw_infos<B: Backend>(f: &mut Frame<B>, app: &mut UiApp, rect: Rect) {
    let spans = vec![
        Spans::from(vec![
            Span::styled("Cluster: ", Style::default().fg(Color::LightBlue)),
            Span::styled("local", Style::default().add_modifier(Modifier::BOLD)),
        ]),
        Spans::from(vec![
            Span::styled("Host:    ", Style::default().fg(Color::LightBlue)),
            Span::styled("localhost", Style::default().add_modifier(Modifier::BOLD)),
        ]),
    ];

    let infos = Paragraph::new(spans)
        .style(Style::default().fg(Color::Gray))
        .alignment(ratatui::layout::Alignment::Left);
    f.render_widget(infos, rect);
}

/// Render the tabs, one per screen
fn draw_tabs<B: Backend>(f: &mut Frame<B>, app: &mut UiApp, rect: Rect) {
    let screens = vec!["Indices", "Aliases", "Nodes"];

    let titles = screens
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow)),
                Span::styled(rest, Style::default().fg(Color::Gray)),
            ])
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default())
        .select(app.selected_screen_idx)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black)
                .fg(Color::Cyan),
        );
    f.render_widget(tabs, rect);
}

fn draw_help<B: Backend>(f: &mut Frame<B>, app: &UiApp, rect: Rect) {
    let mut help_text = match app.input_mode {
        InputMode::Normal => Text::from(Spans::from(vec![
            Span::raw("Press "),
            Span::styled("<q>", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit "),
            Span::styled("<f>", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to filter the list."),
        ])),
        InputMode::Filtering => Text::from(Spans::from(vec![
            Span::raw("Press "),
            Span::styled("<esc>", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit filter mode"),
        ])),
    };

    help_text.patch_style(Style::default().add_modifier(Modifier::RAPID_BLINK));
    let help_message = Paragraph::new(help_text);
    f.render_widget(help_message, rect);
}

fn draw_input_filter<B: Backend>(f: &mut Frame<B>, app: &UiApp, rect: Rect) {
    let text = Text::from(Spans::from(vec![
        Span::raw("> "),
        Span::styled(
            app.indices_filter.as_str(),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ]));

    let input = Paragraph::new(text)
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(input, rect);
}

fn draw_indices_table<B: Backend>(f: &mut Frame<B>, app: &mut UiApp, rect: Rect) {
    let header = draw_indices_table_header();
    let selected_style = Style::default()
        .bg(Color::Cyan)
        .fg(Color::Black)
        .add_modifier(Modifier::BOLD);

    let rows = app
        .indices
        .items
        .iter()
        .filter(|index| index.to_lowercase().contains(app.indices_filter.as_str()))
        .map(|index| draw_indices_table_row(app, index));

    let indices_count = rows.clone().count();

    let filter = if app.indices_filter.is_empty() {
        "".to_string()
    } else {
        format!("/ match against {} <f>", app.indices_filter.as_str())
    };

    let table_title = Spans::from(vec![
        Span::styled("Indices ", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(filter),
        Span::styled("[", Style::default().add_modifier(Modifier::BOLD)),
        Span::styled(
            format!("{indices_count}"),
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("]", Style::default().add_modifier(Modifier::BOLD)),
    ]);

    let table = Table::new(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .style(Style::default().fg(Color::Cyan).bg(Color::Black))
                .title(table_title)
                .title_alignment(ratatui::layout::Alignment::Center),
        )
        .highlight_style(selected_style)
        .widths(&INDICES_TABLE_WIDTH);
    f.render_stateful_widget(table, rect, &mut app.indices.state)
}

/// Render an index table row
fn draw_indices_table_row(app: &UiApp, index_name: &str) -> Row<'static> {
    let cells: Vec<String> = vec![
        index_name.to_owned(),
        "".to_owned(),
        "".to_owned(),
        "".to_owned(),
        "".to_owned(),
        "".to_owned(),
        "".to_owned(),
    ];

    Row::new(cells)
        .height(1)
        .bottom_margin(0)
        .style(Style::default().fg(Color::Cyan))
}

/// Render the indices table header
fn draw_indices_table_header() -> Row<'static> {
    let header_cells = INDICES_TABLE_HEADER.iter().map(|h| Cell::from(*h));
    Row::new(header_cells)
        .height(1)
        .bottom_margin(1)
        .style(Style::default().fg(Color::Gray))
}
