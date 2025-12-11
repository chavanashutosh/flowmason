use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;
use ratatui::widgets::ListState;

struct App {
    flows: Vec<FlowItem>,
    list_state: ListState,
    mode: AppMode,
    api_url: String,
}

#[derive(Clone)]
struct FlowItem {
    id: String,
    name: String,
    active: bool,
}

#[derive(Clone)]
enum AppMode {
    FlowsList,
    FlowDetails(String),
    Help,
}

impl App {
    fn new() -> Self {
        Self {
            flows: Vec::new(),
            list_state: ListState::default(),
            mode: AppMode::FlowsList,
            api_url: "http://localhost:3000".to_string(),
        }
    }

    async fn load_flows(&mut self) -> Result<()> {
        let client = reqwest::Client::new();
        let url = format!("{}/api/v1/flows", self.api_url);
        let response = client.get(&url).send().await?;
        
        if response.status().is_success() {
            let flows: Vec<serde_json::Value> = response.json().await?;
            self.flows = flows
                .into_iter()
                .map(|f| FlowItem {
                    id: f["id"].as_str().unwrap_or("").to_string(),
                    name: f["name"].as_str().unwrap_or("Unknown").to_string(),
                    active: f["active"].as_bool().unwrap_or(false),
                })
                .collect();
        }
        Ok(())
    }

    fn next_flow(&mut self) {
        if !self.flows.is_empty() {
            let i = match self.list_state.selected() {
                Some(i) => {
                    if i >= self.flows.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.list_state.select(Some(i));
        }
    }

    fn previous_flow(&mut self) {
        if !self.flows.is_empty() {
            let i = match self.list_state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.flows.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.list_state.select(Some(i));
        }
    }
    
    fn get_selected_flow(&self) -> Option<&FlowItem> {
        self.list_state.selected().and_then(|i| self.flows.get(i))
    }
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new();
    let rt = tokio::runtime::Runtime::new()?;
    
    // Load flows
    rt.block_on(app.load_flows()).ok();

    // Run app
    let result = run_app(&mut terminal, &mut app, &rt);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    rt: &tokio::runtime::Runtime,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('h') => app.mode = AppMode::Help,
                    KeyCode::Char('r') => {
                        rt.block_on(app.load_flows()).ok();
                        if !app.flows.is_empty() && app.list_state.selected().is_none() {
                            app.list_state.select(Some(0));
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => app.next_flow(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous_flow(),
                    KeyCode::Enter => {
                        if let Some(flow) = app.get_selected_flow() {
                            app.mode = AppMode::FlowDetails(flow.id.clone());
                        }
                    }
                    KeyCode::Esc => app.mode = AppMode::FlowsList,
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());

    // Title
    let title = Paragraph::new("🧱 FlowMason - Visual Automation Platform")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Main content
    match app.mode.clone() {
        AppMode::FlowsList => render_flows_list(f, app, chunks[1]),
        AppMode::FlowDetails(id) => render_flow_details(f, app, &id, chunks[1]),
        AppMode::Help => render_help(f, chunks[1]),
    }

    // Footer
    let footer = Paragraph::new("Press 'q' to quit | 'h' for help | 'r' to refresh | Arrow keys to navigate")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

fn render_flows_list(f: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .flows
        .iter()
        .enumerate()
        .map(|(i, flow)| {
            let status = if flow.active {
                Span::styled("● Active", Style::default().fg(Color::Green))
            } else {
                Span::styled("○ Inactive", Style::default().fg(Color::Gray))
            };
            ListItem::new(Line::from(vec![
                Span::raw(format!("{} ", i + 1)),
                Span::styled(&flow.name, Style::default().fg(Color::White)),
                Span::raw(" - "),
                status,
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Flows (Press Enter to view details)"),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, area, &mut app.list_state);
}

fn render_flow_details(f: &mut Frame, app: &App, flow_id: &str, area: Rect) {
    let flow = app.flows.iter().find(|f| f.id == *flow_id);
    
    let content = if let Some(flow) = flow {
        vec![
            Line::from(vec![Span::styled("Flow Details", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
            Line::from(""),
            Line::from(vec![Span::raw("ID: "), Span::styled(&flow.id, Style::default().fg(Color::Yellow))]),
            Line::from(vec![Span::raw("Name: "), Span::styled(&flow.name, Style::default().fg(Color::White))]),
            Line::from(vec![Span::raw("Status: "), Span::styled(
                if flow.active { "Active" } else { "Inactive" },
                Style::default().fg(if flow.active { Color::Green } else { Color::Gray })
            )]),
            Line::from(""),
            Line::from("Press 'Esc' to go back"),
        ]
    } else {
        vec![Line::from("Flow not found")]
    };

    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title("Flow Details"))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn render_help(f: &mut Frame, area: Rect) {
    let help_text = vec![
        Line::from(vec![Span::styled("FlowMason TUI Help", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
        Line::from(""),
        Line::from("Navigation:"),
        Line::from("  ↑/k    - Move up"),
        Line::from("  ↓/j    - Move down"),
        Line::from("  Enter  - View flow details"),
        Line::from("  Esc    - Go back"),
        Line::from(""),
        Line::from("Actions:"),
        Line::from("  r      - Refresh flows list"),
        Line::from("  h      - Show this help"),
        Line::from("  q      - Quit"),
    ];

    let paragraph = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

