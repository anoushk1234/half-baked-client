use crate::sampler::GetShredResponse;
use crate::tinydancer::{ClientService, TinyDancer};
use async_trait::async_trait;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{any::Any, thread::Thread};
use std::{fmt, thread::JoinHandle};
use thiserror::Error;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::Direction,
    layout::{Constraint, Layout},
    widgets::{
        Axis, BarChart, Block, Borders, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem,
        ListState, Paragraph, Row, Sparkline, Table, TableState, Tabs, Wrap,
    },
    Frame, Terminal,
};

pub struct UiService {
    //pub views: Vec<String>, //placeholder
    pub ui_service_handle: JoinHandle<()>, // pub table: TableState,  // placeholder view
}

pub struct App {
    title: String,
    table: TableState,
    slot_list: StatefulList<(String, usize)>,
}

// pub struct SlotList {
//     title: String,
//     state: ListState,
//     items: Vec<Vec<(usize, String)>>,
// }
pub struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}
impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
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
impl App {}
pub struct UiConfig {}

pub fn draw<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());
}
pub fn draw_slot_list<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::LightBlue);
    let items: Vec<ListItem> = app
        .slot_list
        .items
        .iter()
        .map(|i| {
            let mut lines = vec![Spans::from(i.0.clone())];
            for _ in 0..i.1 {
                lines.push(Spans::from(Span::styled(
                    "slots 1",
                    Style::default().add_modifier(Modifier::ITALIC),
                )));
            }
            ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
        })
        .collect();
    let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("SLOTS"))
            .highlight_style(Style::default().bg(Color::Cyan).add_modifier(Modifier::BOLD))
            .highlight_symbol(">>");
          
}

#[async_trait]
impl ClientService<UiConfig> for UiService {
    type ServiceError = ThreadJoinError;
    fn new(config: UiConfig) -> Self {
        let ui_service_handle = std::thread::spawn(|| loop {
            println!("rendering ui");
            std::thread::sleep(std::time::Duration::from_secs(2));
        });
        Self { ui_service_handle }
    }
    async fn join(self) -> std::result::Result<(), Self::ServiceError> {
        match self.ui_service_handle.join() {
            Ok(_) => Ok(()),
            Err(error) => Err(ThreadJoinError { error }),
        }
    }
}

#[derive(Debug, Error)]
pub struct ThreadJoinError {
    error: Box<dyn Any + Send>,
}

// impl ThreadJoinError {
//     fn new<E: Any + Send>(msg: Box<E>) -> ThreadJoinError {
//         ThreadJoinError { error: msg }
//     }
// }

impl fmt::Display for ThreadJoinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.error)
    }
}

// impl Error for ThreadJoinError {
//     fn description(&self) -> &str {
//         &self.error.into()
//     }
// }
