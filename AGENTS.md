# 🤖 Agent Guidelines (AI Developer Context)

This file provides architectural context and instructions for AI agents working on `cyberfeed`.

## Architecture Overview
* `src/main.rs`: Entry point, loads configuration, initializes SQLite database, performs startup fetch, and launches the TUI runner.
* `src/config.rs`: Handles YAML configuration loading (`config.yaml`), theme enumeration (`ThemeName`), and defaults.
* `src/db.rs`: SQLite persistence layer (`rusqlite`) with non-destructive table creation and column migrations.
* `src/fetcher.rs`: Async RSS/Atom parsing (`rss`, `atom_syndication`) using `reqwest` with custom browser User-Agent headers.
* `src/ui.rs`: Ratatui interface, supporting dual view modes (`List` and `Reader`), tabbed index filtering (`filtered_indices()`), clipboard integration (`arboard`), and theme palette rendering.

## Code Conventions
* Avoid borrowing conflicts between `app.articles` and `app.state` by using **index-based filtering (`filtered_indices() -> Vec<usize>`)** rather than returning element references.
* Ensure all Rust code passes `cargo fmt` and `cargo clippy` without warnings.
