pub mod lib;
use lib::state::*;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    fmt::Debug,
    io,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::{Duration, Instant},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};

fn spawn_input_thread() -> Receiver<Event<KeyEvent>> {
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout).expect("poll not working") {
                if let CEvent::Key(key) = event::read().expect("Failed to read keystroke") {
                    tx.send(Event::Input(key))
                        .expect("Cannot send key through mpsc");
                }
            }
            if last_tick.elapsed() >= tick_rate && tx.send(Event::Tick).is_ok() {
                last_tick = Instant::now();
            }
        }
    });
    rx
}

fn render_loop<T: tui::backend::Backend>(
    term: &mut Terminal<T>,
    mut state: TuiState,
    rx: Receiver<Event<KeyEvent>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut active_menu_item = MenuItem::Home;
    loop {
        term.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Length(3),
                        Constraint::Min(2),
                    ]
                    .as_ref(),
                )
                .split(size);
            let v_split = Layout::default()
                .direction(Direction::Horizontal)
                .margin(0)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[2]);
            rect.render_widget(render_menu(), chunks[0]);
            rect.render_widget(render_search_bar(&state), chunks[1]);
            rect.render_stateful_widget(render_list(&state), v_split[0], &mut state.list_state);
            rect.render_widget(render_details(&state), v_split[1]);
        })?;
        if let Ok(res) = user_input_handle(term, &mut state, &rx) {
            if res {
                break;
            }
        }
    }
    Ok(())
}

fn render_list<'a>(state: &TuiState) -> List<'a> {
    let blk = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Papers")
        .border_type(BorderType::Plain);
    let items = {
        if let Some(entries) = &state.data {
            entries
                .iter()
                .map(|item| {
                    ListItem::new(Spans::from(vec![Span::styled(
                        item.title.clone(),
                        Style::default(),
                    )]))
                })
                .collect::<Vec<ListItem>>()
        } else {
            vec![ListItem::new("?")]
        }
    };
    List::new(items).block(blk).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    )
}

fn render_details<'a>(state: &TuiState) -> Paragraph<'a> {
    let text = {
        if let Some(n) = state.list_state.selected() {
            if let Some(data) = &state.data {
                data[n].summary.clone()
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    };
    Paragraph::new(text)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Description")
                .border_type(BorderType::Plain),
        )
}

fn render_search_bar<'a>(state: &TuiState) -> Paragraph<'a> {
    Paragraph::new(state.input.clone())
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Search")
                .border_type(BorderType::Plain),
        )
}

fn render_menu<'a>() -> Tabs<'a> {
    let menu = MenuItem::TITLES
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();
    Tabs::new(menu)
}

fn user_input_handle<T: tui::backend::Backend>(
    term: &mut Terminal<T>,
    state: &mut TuiState,
    rx: &Receiver<Event<KeyEvent>>,
) -> Result<bool, Box<dyn std::error::Error>> {
    match rx.recv()? {
        Event::Input(e) => {
            if state.input_state == InputState::NormalMode {
                match e.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        term.show_cursor()?;
                        return Ok(true);
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        if let Some(selected) = state.list_state.selected() {
                            let amount = {
                                if let Some(data) = &state.data {
                                    data.len()
                                } else {
                                    0
                                }
                            };
                            if selected == amount - 1 {
                                state.list_state.select(Some(0));
                            } else {
                                state.list_state.select(Some(selected + 1));
                            }
                        }
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        if let Some(selected) = state.list_state.selected() {
                            let amount = 4;
                            if selected > 0 {
                                state.list_state.select(Some(selected - 1));
                            } else {
                                state.list_state.select(Some(amount - 1));
                            }
                        }
                    }
                    KeyCode::Char('i') | KeyCode::Char('s') => {
                        state.input_state = InputState::InsertMode
                    }
                    KeyCode::Enter => {
                        if let Some(u) = state.list_state.selected() {
                            if let Some(data) = &state.data {
                                let _ = open::that(&data[u].pdf_link);
                            }
                        }
                    }
                    _ => (),
                }
            } else if state.input_state == InputState::InsertMode {
                match e.code {
                    KeyCode::Esc => state.input_state = InputState::NormalMode,
                    KeyCode::Backspace => {
                        let _ = state.input.pop();
                    }
                    KeyCode::Char(c) => state.input.push(c),
                    KeyCode::Enter => {
                        state.search(0, 10);
                    }
                    _ => {}
                }
            }
        }
        Event::Tick => {}
    }
    Ok(false)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("Cannot run window in raw mode");
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    let rx = spawn_input_thread();
    let mut state = TuiState::default();
    render_loop(&mut terminal, state, rx);
    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    );
    Ok(())
}
