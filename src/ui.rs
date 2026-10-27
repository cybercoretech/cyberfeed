use crate::config::ThemeName;
use crate::db::{Article, Database};
use anyhow::Result;
use arboard::Clipboard;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Tabs, Wrap},
    Frame, Terminal,
};
use std::{io, time::Duration};

pub struct Theme {
    pub border: Color,
    pub title: Color,
    pub tab_active: Color,
    pub tab_inactive: Color,
    pub source_tag: Color,
    pub highlight_bg: Color,
    pub highlight_fg: Color,
    pub code_fg: Color,
    pub code_bg: Color,
}

impl Theme {
    pub fn from_name(name: &ThemeName) -> Self {
        match name {
            ThemeName::Cyberpunk => Self {
                border: Color::Cyan,
                title: Color::Yellow,
                tab_active: Color::Yellow,
                tab_inactive: Color::Gray,
                source_tag: Color::Green,
                highlight_bg: Color::DarkGray,
                highlight_fg: Color::White,
                code_fg: Color::LightMagenta,
                code_bg: Color::Rgb(20, 20, 35),
            },
            ThemeName::Dracula => Self {
                border: Color::Rgb(189, 147, 249),
                title: Color::Rgb(255, 121, 198),
                tab_active: Color::Rgb(255, 121, 198),
                tab_inactive: Color::Rgb(98, 114, 164),
                source_tag: Color::Rgb(80, 250, 123),
                highlight_bg: Color::Rgb(68, 71, 90),
                highlight_fg: Color::Rgb(248, 248, 242),
                code_fg: Color::Rgb(139, 233, 253),
                code_bg: Color::Rgb(40, 42, 54),
            },
            ThemeName::Nord => Self {
                border: Color::Rgb(136, 192, 208),
                title: Color::Rgb(235, 203, 139),
                tab_active: Color::Rgb(235, 203, 139),
                tab_inactive: Color::Rgb(76, 86, 106),
                source_tag: Color::Rgb(163, 190, 140),
                highlight_bg: Color::Rgb(67, 76, 94),
                highlight_fg: Color::Rgb(236, 239, 244),
                code_fg: Color::Rgb(143, 188, 187),
                code_bg: Color::Rgb(46, 52, 64),
            },
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum ViewMode {
    List,
    Reader,
}

pub struct App {
    pub db: Database,
    pub articles: Vec<Article>,
    pub state: ListState,
    pub view_mode: ViewMode,
    pub should_quit: bool,
    pub status_msg: String,
    pub tabs: Vec<String>,
    pub tab_index: usize,
    pub theme: Theme,
}

impl App {
    pub fn new(db: Database, theme_name: ThemeName) -> Result<Self> {
        let articles = db.get_articles()?;

        let mut tabs = vec!["All".to_string()];
        let mut sources: Vec<String> = articles.iter().map(|a| a.source.clone()).collect();
        sources.sort();
        sources.dedup();
        tabs.extend(sources);

        let mut state = ListState::default();
        if !articles.is_empty() {
            state.select(Some(0));
        }

        Ok(Self {
            db,
            articles,
            state,
            view_mode: ViewMode::List,
            should_quit: false,
            status_msg: "Ready. [Tab] Switch Tab | [Enter] Reader | [o/b] Browser | [q] Quit"
                .into(),
            tabs,
            tab_index: 0,
            theme: Theme::from_name(&theme_name),
        })
    }

    pub fn reload(&mut self) {
        if let Ok(articles) = self.db.get_articles() {
            self.articles = articles;

            let mut tabs = vec!["All".to_string()];
            let mut sources: Vec<String> = self.articles.iter().map(|a| a.source.clone()).collect();
            sources.sort();
            sources.dedup();
            tabs.extend(sources);

            if self.tab_index >= tabs.len() {
                self.tab_index = 0;
            }
            self.tabs = tabs;

            let filtered = self.filtered_indices();
            if filtered.is_empty() {
                self.state.select(None);
            } else if self.state.selected().is_none() {
                self.state.select(Some(0));
            }
        }
    }

    pub fn filtered_indices(&self) -> Vec<usize> {
        if self.tab_index == 0 {
            (0..self.articles.len()).collect()
        } else if let Some(tab_name) = self.tabs.get(self.tab_index) {
            self.articles
                .iter()
                .enumerate()
                .filter(|(_, a)| &a.source == tab_name)
                .map(|(i, _)| i)
                .collect()
        } else {
            (0..self.articles.len()).collect()
        }
    }

    pub fn next_tab(&mut self) {
        if self.tabs.is_empty() {
            return;
        }
        self.tab_index = (self.tab_index + 1) % self.tabs.len();
        let filtered_len = self.filtered_indices().len();
        if filtered_len > 0 {
            self.state.select(Some(0));
        } else {
            self.state.select(None);
        }
        self.status_msg = format!("Switched to tab: {}", self.tabs[self.tab_index]);
    }

    pub fn prev_tab(&mut self) {
        if self.tabs.is_empty() {
            return;
        }
        if self.tab_index == 0 {
            self.tab_index = self.tabs.len() - 1;
        } else {
            self.tab_index -= 1;
        }
        let filtered_len = self.filtered_indices().len();
        if filtered_len > 0 {
            self.state.select(Some(0));
        } else {
            self.state.select(None);
        }
        self.status_msg = format!("Switched to tab: {}", self.tabs[self.tab_index]);
    }

    pub fn next(&mut self) {
        let filtered = self.filtered_indices();
        if filtered.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= filtered.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let filtered = self.filtered_indices();
        if filtered.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    filtered.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn copy_link(&mut self) {
        let filtered = self.filtered_indices();
        if let Some(idx) = self.state.selected() {
            if let Some(&article_idx) = filtered.get(idx) {
                let link = &self.articles[article_idx].link;
                if let Ok(mut cb) = Clipboard::new() {
                    if cb.set_text(link.clone()).is_ok() {
                        self.status_msg = "Link copied to clipboard!".into();
                        return;
                    }
                }
            }
        }
        self.status_msg = "Failed to copy link".into();
    }

    pub fn open_in_browser(&mut self) {
        let filtered = self.filtered_indices();
        if let Some(idx) = self.state.selected() {
            if let Some(&article_idx) = filtered.get(idx) {
                let link = &self.articles[article_idx].link;
                let _ = open::that(link);
                self.status_msg = format!("Opening in browser: {}", link);
            }
        }
    }
}

pub async fn run_tui(mut app: App) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| render_ui(f, &mut app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match app.view_mode {
                        ViewMode::List => {
                            match key.code {
                                KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                                KeyCode::Char('j') | KeyCode::Down => app.next(),
                                KeyCode::Char('k') | KeyCode::Up => app.previous(),
                                KeyCode::Tab => app.next_tab(),
                                KeyCode::BackTab => app.prev_tab(),
                                KeyCode::Right => app.next_tab(),
                                KeyCode::Left => app.prev_tab(),
                                KeyCode::Enter => {
                                    if !app.filtered_indices().is_empty() {
                                        app.view_mode = ViewMode::Reader;
                                        app.status_msg = "Reading in TUI. Press [Esc] to return, [o] for browser.".into();
                                    }
                                }
                                KeyCode::Char('o') | KeyCode::Char('b') => app.open_in_browser(),
                                KeyCode::Char('y') => app.copy_link(),
                                KeyCode::Char('r') => {
                                    app.reload();
                                    app.status_msg = "Reloaded database articles".into();
                                }
                                _ => {}
                            }
                        }
                        ViewMode::Reader => match key.code {
                            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Backspace => {
                                app.view_mode = ViewMode::List;
                                app.status_msg = "Returned to article list.".into();
                            }
                            KeyCode::Char('o') | KeyCode::Char('b') => app.open_in_browser(),
                            KeyCode::Char('y') => app.copy_link(),
                            _ => {}
                        },
                    }
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn format_content_with_highlighting<'a>(content: &'a str, theme: &Theme) -> Vec<Line<'a>> {
    let mut lines = Vec::new();
    let mut in_code_block = false;

    for raw_line in content.lines() {
        let line_str = raw_line.trim_end();

        if line_str.starts_with("```") {
            in_code_block = !in_code_block;
            lines.push(Line::from(Span::styled(
                "--------------------------------------------------------------------------------",
                Style::default().fg(theme.border),
            )));
            continue;
        }

        if in_code_block {
            lines.push(Line::from(Span::styled(
                format!("  {}", line_str),
                Style::default().fg(theme.code_fg).bg(theme.code_bg),
            )));
        } else if line_str.starts_with('#') {
            lines.push(Line::from(Span::styled(
                line_str,
                Style::default()
                    .fg(theme.title)
                    .add_modifier(Modifier::BOLD),
            )));
        } else {
            lines.push(Line::from(Span::raw(line_str)));
        }
    }
    lines
}

fn render_ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(3), // Tabs
            Constraint::Min(10),   // Main content
            Constraint::Length(3), // Footer
        ])
        .split(f.area());

    let theme = &app.theme;

    // Title Block
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(theme.border));
    let title = Paragraph::new(Span::styled(
        " 📰 CYBERFEED RSS READER (TUI) ",
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(theme.title),
    ))
    .block(title_block);
    f.render_widget(title, chunks[0]);

    // Tabs Widget
    let tab_titles: Vec<Line> = app
        .tabs
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let style = if i == app.tab_index {
                Style::default()
                    .fg(theme.tab_active)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.tab_inactive)
            };
            Line::from(Span::styled(format!(" {} ", t), style))
        })
        .collect();

    let tabs = Tabs::new(tab_titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Feed Families ")
                .border_style(Style::default().fg(theme.border)),
        )
        .highlight_style(
            Style::default()
                .fg(theme.tab_active)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(tabs, chunks[1]);

    match app.view_mode {
        ViewMode::List => {
            let main_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
                .split(chunks[2]);

            let filtered = app.filtered_indices();

            let list_items: Vec<ListItem> = filtered
                .iter()
                .map(|&article_idx| {
                    let item = &app.articles[article_idx];
                    let content = vec![Line::from(vec![
                        Span::styled(
                            format!("[{}] ", item.source),
                            Style::default().fg(theme.source_tag),
                        ),
                        Span::raw(&item.title),
                    ])];
                    ListItem::new(content)
                })
                .collect();

            let list = List::new(list_items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!(" Articles ({}) ", filtered.len()))
                        .border_style(Style::default().fg(theme.border)),
                )
                .highlight_style(
                    Style::default()
                        .bg(theme.highlight_bg)
                        .fg(theme.highlight_fg)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("> ");

            f.render_stateful_widget(list, main_chunks[0], &mut app.state);

            let preview_lines = match app.state.selected() {
                Some(i) => {
                    if let Some(&article_idx) = filtered.get(i) {
                        let item = &app.articles[article_idx];
                        let mut lines = vec![
                            Line::from(vec![
                                Span::styled(
                                    "Title: ",
                                    Style::default().add_modifier(Modifier::BOLD),
                                ),
                                Span::raw(&item.title),
                            ]),
                            Line::from(vec![
                                Span::styled(
                                    "Source: ",
                                    Style::default().add_modifier(Modifier::BOLD),
                                ),
                                Span::styled(&item.source, Style::default().fg(theme.source_tag)),
                            ]),
                            Line::from(vec![
                                Span::styled(
                                    "Link: ",
                                    Style::default().add_modifier(Modifier::BOLD),
                                ),
                                Span::raw(&item.link),
                            ]),
                            Line::from(""),
                        ];
                        lines.extend(format_content_with_highlighting(&item.content, theme));
                        lines
                    } else {
                        vec![Line::from("No article selected.")]
                    }
                }
                None => vec![Line::from(
                    "No articles available in this tab. Press [r] to reload.",
                )],
            };

            let preview = Paragraph::new(preview_lines)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Quick Preview ")
                        .border_style(Style::default().fg(theme.border)),
                )
                .wrap(Wrap { trim: true });
            f.render_widget(preview, main_chunks[1]);
        }
        ViewMode::Reader => {
            let filtered = app.filtered_indices();
            let reader_lines = match app.state.selected() {
                Some(i) => {
                    if let Some(&article_idx) = filtered.get(i) {
                        let item = &app.articles[article_idx];
                        let mut lines = vec![
                            Line::from(Span::styled(format!("TITLE: {}", item.title), Style::default().fg(theme.title).add_modifier(Modifier::BOLD))),
                            Line::from(Span::styled(format!("SOURCE: {}", item.source), Style::default().fg(theme.source_tag))),
                            Line::from(Span::raw(format!("LINK: {}", item.link))),
                            Line::from(Span::styled("--------------------------------------------------------------------------------", Style::default().fg(theme.border))),
                            Line::from(""),
                        ];
                        lines.extend(format_content_with_highlighting(&item.content, theme));
                        lines
                    } else {
                        vec![Line::from("No article selected.")]
                    }
                }
                None => vec![Line::from("No article selected.")],
            };

            let reader_pane = Paragraph::new(reader_lines)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" 📖 In-TUI Reader View (Press Esc to exit) ")
                        .border_style(Style::default().fg(theme.border)),
                )
                .wrap(Wrap { trim: true });
            f.render_widget(reader_pane, chunks[2]);
        }
    }

    // Status / Keybindings Footer
    let footer_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border));
    let footer_text = match app.view_mode {
        ViewMode::List => {
            format!(
                " [Tab/Arrow] Switch Tab | [j/k] Nav | [Enter] Reader | [o/b] Browser | [q] Quit | Status: {}",
                app.status_msg
            )
        }
        ViewMode::Reader => {
            format!(
                " [Esc] Back to List | [o/b] Open in Browser | [y] Copy Link | Status: {}",
                app.status_msg
            )
        }
    };
    let footer = Paragraph::new(Span::styled(footer_text, Style::default().fg(theme.title)))
        .block(footer_block);
    f.render_widget(footer, chunks[3]);
}
