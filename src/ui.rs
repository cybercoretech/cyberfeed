use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Tabs},
    Frame, Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use crate::config::ThemeName;
use crate::db::{Database, Article};
use pulldown_cmark::{Parser, Tag, TagEnd, Event as MarkdownEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    List,
    Reader,
    ThemeChooser,
}

#[derive(Debug, Clone)]
pub struct ThemeItem {
    pub id: String,
    pub name: String,
    pub family: String,
    pub primary: Color,
    pub bg: Color,
    pub text: Color,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct ThemePalette {
    pub primary: Color,
    pub secondary: Color,
    pub highlight: Color,
    pub background: Color,
    pub text: Color,
}

impl ThemePalette {
    pub fn for_theme(theme: ThemeName) -> Self {
        match theme {
            ThemeName::Cyberpunk => Self {
                primary: Color::Magenta,
                secondary: Color::Cyan,
                highlight: Color::Yellow,
                background: Color::Black,
                text: Color::White,
            },
            ThemeName::Dracula => Self {
                primary: Color::Rgb(189, 147, 249),
                secondary: Color::Rgb(139, 233, 253),
                highlight: Color::Rgb(255, 121, 198),
                background: Color::Rgb(40, 42, 54),
                text: Color::Rgb(248, 248, 242),
            },
            ThemeName::Nord => Self {
                primary: Color::Rgb(129, 161, 193),
                secondary: Color::Rgb(143, 188, 187),
                highlight: Color::Rgb(235, 203, 139),
                background: Color::Rgb(46, 52, 64),
                text: Color::Rgb(216, 222, 233),
            },
        }
    }

    pub fn for_cybercore(t: &ThemeItem) -> Self {
        Self {
            primary: t.primary,
            secondary: t.primary,
            highlight: t.primary,
            background: t.bg,
            text: t.text,
        }
    }

    pub fn block_style(&self) -> Style {
        Style::default().fg(self.primary).bg(self.background)
    }
}

pub struct App {
    pub theme: ThemeName,
    pub database: Database,
    pub articles: Vec<Article>,
    pub selected_tab: usize,
    pub selected_index: usize,
    pub view_mode: ViewMode,
    pub feed_names: Vec<String>,
    pub themes: Vec<ThemeItem>,
    pub selected_theme_index: usize,
    pub theme_list_offset: usize,
    pub active_cybercore_theme: Option<ThemeItem>,
}

impl App {
    pub fn new(database: Database, theme: ThemeName) -> anyhow::Result<Self> {
        let themes = load_cybercore_themes();
        let mut app = Self {
            theme,
            database,
            articles: Vec::new(),
            selected_tab: 0,
            selected_index: 0,
            view_mode: ViewMode::List,
            feed_names: Vec::new(),
            themes,
            selected_theme_index: 0,
            theme_list_offset: 0,
            active_cybercore_theme: None,
        };
        app.reload_articles()?;
        Ok(app)
    }

    pub fn reload_articles(&mut self) -> anyhow::Result<()> {
        self.articles = self.database.get_articles()?;
        let mut feeds = vec!["All".to_string()];
        for article in &self.articles {
            if !feeds.contains(&article.source) {
                feeds.push(article.source.clone());
            }
        }
        self.feed_names = feeds;
        if self.selected_tab >= self.feed_names.len() {
            self.selected_tab = 0;
        }
        self.clamp_selection();
        Ok(())
    }

    pub fn filtered_articles(&self) -> Vec<&Article> {
        if self.selected_tab == 0 {
            self.articles.iter().collect()
        } else {
            let feed_name = &self.feed_names[self.selected_tab];
            self.articles.iter().filter(|a| &a.source == feed_name).collect()
        }
    }

    pub fn clamp_selection(&mut self) {
        let count = self.filtered_articles().len();
        if count == 0 {
            self.selected_index = 0;
        } else if self.selected_index >= count {
            self.selected_index = count - 1;
        }
    }

    pub fn next_tab(&mut self) {
        if !self.feed_names.is_empty() {
            self.selected_tab = (self.selected_tab + 1) % self.feed_names.len();
            self.selected_index = 0;
        }
    }

    pub fn prev_tab(&mut self) {
        if !self.feed_names.is_empty() {
            if self.selected_tab == 0 {
                self.selected_tab = self.feed_names.len() - 1;
            } else {
                self.selected_tab -= 1;
            }
            self.selected_index = 0;
        }
    }

    pub fn next_item(&mut self) {
        let count = self.filtered_articles().len();
        if count > 0 && self.selected_index < count - 1 {
            self.selected_index += 1;
        }
    }

    pub fn prev_item(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn selected_article(&self) -> Option<&Article> {
        let filtered = self.filtered_articles();
        filtered.get(self.selected_index).copied()
    }
}

fn parse_hex_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    if hex.len() >= 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(100);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(100);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(100);
        Color::Rgb(r, g, b)
    } else {
        Color::Magenta
    }
}

fn load_cybercore_themes() -> Vec<ThemeItem> {
    let path = "/home/raven/Devspace/Cybercore/framework/cybercore/docs/theme-chooser-data.json";
    if let Ok(contents) = std::fs::read_to_string(path) {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&contents) {
            if let Some(arr) = v.get("themes").and_then(|t| t.as_array()) {
                let mut list = Vec::new();
                for item in arr {
                    let id = item.get("id").and_then(|s| s.as_str()).unwrap_or("theme").to_string();
                    let name = item.get("name").and_then(|s| s.as_str()).unwrap_or("Theme").to_string();
                    let family = item.get("family").and_then(|s| s.as_str()).unwrap_or("general").to_string();
                    let colors = item.get("colors");
                    let bg_hex = colors.and_then(|c| c.get("bg")).and_then(|s| s.as_str()).unwrap_or("050710");
                    let cyan_hex = colors.and_then(|c| c.get("cyan")).and_then(|s| s.as_str()).unwrap_or("22e6ff");
                    let acid_hex = colors.and_then(|c| c.get("acid_green")).and_then(|s| s.as_str()).unwrap_or("c6ff45");
                    
                    list.push(ThemeItem {
                        id,
                        name,
                        family,
                        primary: parse_hex_color(cyan_hex),
                        bg: parse_hex_color(bg_hex),
                        text: parse_hex_color(acid_hex),
                    });
                }
                if !list.is_empty() {
                    return list;
                }
            }
        }
    }
    vec![ThemeItem {
        id: "cyberpunk-fallback".to_string(),
        name: "Cyberpunk Fallback".to_string(),
        family: "fallback".to_string(),
        primary: Color::Magenta,
        bg: Color::Black,
        text: Color::White,
    }]
}

fn render_markdown_highlighted(markdown: &str, palette: &ThemePalette) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    let parser = Parser::new(markdown);
    
    let mut current_spans: Vec<Span<'static>> = Vec::new();
    let mut current_style = Style::default().fg(palette.text);
    let mut in_code_block = false;

    for event in parser {
        match event {
            MarkdownEvent::Start(Tag::Heading { .. }) => {
                current_style = Style::default().fg(palette.primary).add_modifier(Modifier::BOLD);
            }
            MarkdownEvent::End(TagEnd::Heading { .. }) => {
                lines.push(Line::from(current_spans));
                current_spans = Vec::new();
                lines.push(Line::from(""));
                current_style = Style::default().fg(palette.text);
            }
            MarkdownEvent::Start(Tag::CodeBlock(_)) => {
                in_code_block = true;
                lines.push(Line::from(current_spans));
                current_spans = Vec::new();
                current_style = Style::default().fg(palette.highlight).bg(palette.background);
            }
            MarkdownEvent::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                lines.push(Line::from(current_spans));
                current_spans = Vec::new();
                lines.push(Line::from(""));
                current_style = Style::default().fg(palette.text);
            }
            MarkdownEvent::Start(Tag::Emphasis) => {
                current_style = current_style.add_modifier(Modifier::ITALIC);
            }
            MarkdownEvent::End(TagEnd::Emphasis) => {
                current_style = current_style.remove_modifier(Modifier::ITALIC);
            }
            MarkdownEvent::Start(Tag::Strong) => {
                current_style = current_style.add_modifier(Modifier::BOLD);
            }
            MarkdownEvent::End(TagEnd::Strong) => {
                current_style = current_style.remove_modifier(Modifier::BOLD);
            }
            MarkdownEvent::Text(text) => {
                let text_str = text.to_string();
                if in_code_block {
                    for line_str in text_str.lines() {
                        lines.push(Line::from(Span::styled(format!("    {}", line_str), current_style)));
                    }
                } else {
                    for line_str in text_str.lines() {
                        current_spans.push(Span::styled(line_str.to_string(), current_style));
                    }
                }
            }
            MarkdownEvent::SoftBreak | MarkdownEvent::HardBreak
                if !current_spans.is_empty() => {
                lines.push(Line::from(current_spans));
                current_spans = Vec::new();
            }
            _ => {}
        }
    }
    if !current_spans.is_empty() {
        lines.push(Line::from(current_spans));
    }
    if lines.is_empty() {
        lines.push(Line::from(Span::styled(markdown.to_string(), Style::default().fg(palette.text))));
    }
    lines
}

pub fn draw_ui(f: &mut Frame, app: &App) {
    let palette = if let Some(ref ct) = app.active_cybercore_theme {
        ThemePalette::for_cybercore(ct)
    } else {
        ThemePalette::for_theme(app.theme)
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(2),
        ])
        .split(f.area());

    // Tabs
    let titles: Vec<Line> = app.feed_names.iter().map(|n| Line::from(n.as_str())).collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(" Feeds ").style(palette.block_style()))
        .select(app.selected_tab)
        .style(Style::default().fg(palette.text))
        .highlight_style(Style::default().fg(palette.highlight).add_modifier(Modifier::BOLD));
    f.render_widget(tabs, chunks[0]);

    // Content view
    match app.view_mode {
        ViewMode::List => {
            let filtered = app.filtered_articles();
            let items: Vec<ListItem> = filtered
                .iter()
                .enumerate()
                .map(|(i, article)| {
                    let prefix = if i == app.selected_index { "> " } else { "  " };
                    let content = format!("{}{}[{}] {}", prefix, if article.read { "" } else { "● " }, article.source, article.title);
                    let style = if i == app.selected_index {
                        Style::default().fg(palette.highlight).add_modifier(Modifier::BOLD)
                    } else if article.read {
                        Style::default().fg(Color::DarkGray)
                    } else {
                        Style::default().fg(palette.text)
                    };
                    ListItem::new(content).style(style)
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(" Articles ").style(palette.block_style()));
            f.render_widget(list, chunks[1]);
        }
        ViewMode::Reader | ViewMode::ThemeChooser => {
            let content_lines = if let Some(article) = app.selected_article() {
                let raw_md = format!("# {}\n\n**Source:** {}\n**Link:** {}\n\n---\n\n{}", article.title, article.source, article.link, article.content);
                render_markdown_highlighted(&raw_md, &palette)
            } else {
                vec![Line::from("No article selected.")]
            };

            let paragraph = Paragraph::new(content_lines)
                .style(Style::default().fg(palette.text).bg(palette.background))
                .block(Block::default().borders(Borders::ALL).title(" Reader (Syntax Highlighted) ").style(palette.block_style()))
                .wrap(ratatui::widgets::Wrap { trim: false });
            f.render_widget(paragraph, chunks[1]);
        }
    }

    // Status bar
    let status_text = match app.view_mode {
        ViewMode::List | ViewMode::ThemeChooser => " [Tab] Feed | [j/k] Nav | [Enter] Read | [t] Cybercore Themes (72+) | [o] Browser | [y] Copy | [r] Refresh | [q] Quit ",
        ViewMode::Reader => " [Esc/q] Back | [t] Cybercore Themes (72+) | [o] Browser | [y] Copy ",
    };
    let status = Paragraph::new(status_text)
        .style(Style::default().fg(palette.highlight).bg(palette.background));
    f.render_widget(status, chunks[2]);

    // Popup Theme Chooser
    if app.view_mode == ViewMode::ThemeChooser {
        let area = centered_rect(70, 75, f.area());
        f.render_widget(Clear, area);

        let height = area.height.saturating_sub(2) as usize;
        let start = app.theme_list_offset;
        let end = std::cmp::min(start + height, app.themes.len());

        let items: Vec<ListItem> = app.themes[start..end]
            .iter()
            .enumerate()
            .map(|(idx, t)| {
                let actual_idx = start + idx;
                let prefix = if actual_idx == app.selected_theme_index { "> " } else { "  " };
                let content = format!("{}{}: {} [{}]", prefix, t.id, t.name, t.family);
                let style = if actual_idx == app.selected_theme_index {
                    Style::default().fg(palette.highlight).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(palette.text)
                };
                ListItem::new(content).style(style)
            })
            .collect();

        let title = format!(" Select Cybercore Theme ({}/{}): [j/k] Nav, [Enter] Apply, [Esc] Close ", app.selected_theme_index + 1, app.themes.len());
        let theme_list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(title).style(palette.block_style()));
        f.render_widget(theme_list, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub async fn run_tui(mut app: App) -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = (|| -> anyhow::Result<()> {
        loop {
            terminal.draw(|f| draw_ui(f, &app))?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match app.view_mode {
                        ViewMode::List => match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => break,
                            KeyCode::Tab | KeyCode::Right => app.next_tab(),
                            KeyCode::BackTab | KeyCode::Left => app.prev_tab(),
                            KeyCode::Char('j') | KeyCode::Down => app.next_item(),
                            KeyCode::Char('k') | KeyCode::Up => app.prev_item(),
                            KeyCode::Enter => {
                                if let Some(article) = app.selected_article() {
                                    let _ = app.database.toggle_read(article.id, true);
                                    let _ = app.reload_articles();
                                    app.view_mode = ViewMode::Reader;
                                }
                            }
                            KeyCode::Char('r') => {
                                let _ = app.reload_articles();
                            }
                            KeyCode::Char('t') => {
                                app.view_mode = ViewMode::ThemeChooser;
                            }
                            KeyCode::Char('o') | KeyCode::Char('b') => {
                                if let Some(article) = app.selected_article() {
                                    let _ = open::that(&article.link);
                                }
                            }
                            KeyCode::Char('y') => {
                                if let Some(article) = app.selected_article() {
                                    if let Ok(mut clipboard) = arboard::Clipboard::new() {
                                        let _ = clipboard.set_text(article.link.clone());
                                    }
                                }
                            }
                            _ => {}
                        },
                        ViewMode::Reader => match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => {
                                app.view_mode = ViewMode::List;
                            }
                            KeyCode::Char('t') => {
                                app.view_mode = ViewMode::ThemeChooser;
                            }
                            KeyCode::Char('o') | KeyCode::Char('b') => {
                                if let Some(article) = app.selected_article() {
                                    let _ = open::that(&article.link);
                                }
                            }
                            KeyCode::Char('y') => {
                                if let Some(article) = app.selected_article() {
                                    if let Ok(mut clipboard) = arboard::Clipboard::new() {
                                        let _ = clipboard.set_text(article.link.clone());
                                    }
                                }
                            }
                            _ => {}
                        },
                        ViewMode::ThemeChooser => match key.code {
                            KeyCode::Esc => {
                                app.view_mode = ViewMode::List;
                            }
                            KeyCode::Char('j') | KeyCode::Down => {
                                if !app.themes.is_empty() && app.selected_theme_index < app.themes.len() - 1 {
                                    app.selected_theme_index += 1;
                                    let visible_height = 20;
                                    if app.selected_theme_index >= app.theme_list_offset + visible_height {
                                        app.theme_list_offset = app.selected_theme_index - visible_height + 1;
                                    }
                                }
                            }
                            KeyCode::Char('k') | KeyCode::Up => {
                                if app.selected_theme_index > 0 {
                                    app.selected_theme_index -= 1;
                                    if app.selected_theme_index < app.theme_list_offset {
                                        app.theme_list_offset = app.selected_theme_index;
                                    }
                                }
                            }
                            KeyCode::Enter => {
                                if let Some(t) = app.themes.get(app.selected_theme_index).cloned() {
                                    app.active_cybercore_theme = Some(t);
                                }
                                app.view_mode = ViewMode::List;
                            }
                            _ => {}
                        },
                    }
                }
            }
        }
        Ok(())
    })();

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res?;
    Ok(())
}
