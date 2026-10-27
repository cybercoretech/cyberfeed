# 📰 Cyberfeed

> A high-performance, CLI-first RSS/Atom reader built in Rust (`cyberfeed`), optimized for lightning-fast terminal workflows, tabbed feed organization, multi-theme support, and syntax-highlighted article reading.

---

## ✨ Features

*   **⚡ Lightning Fast & Asynchronous**: Powered by `tokio` and `reqwest` with custom User-Agent headers and synchronous startup pre-fetching.
*   **📂 Tabbed "Feed Family" Navigation**: Automatically groups and separates articles by feed source into distinct tabs (with an "All" aggregated feed view).
*   **🎨 Multi-Theme Engine**: Fully customizable color schemes supporting **Cyberpunk**, **Dracula**, and **Nord** out of the box via `config.yaml`.
*   **📖 In-TUI Reader & Syntax Highlighting**: Clean markdown and code-block parsing with custom syntax highlighting styles.
*   **📋 Clipboard & Browser Integration**: Instantly copy article links via `arboard` (`y`) or launch them in your default system browser (`o` / `b`).
*   **🗄️ Robust SQLite Storage**: Non-destructive schema migrations and reliable local persistence via `rusqlite`.

---

## ⌨️ Keybindings

| Key / Shortcut | Action |
| :--- | :--- |
| `Tab` / `Right` | Switch to next feed family tab |
| `Shift+Tab` / `Left` | Switch to previous feed family tab |
| `j` / `Down Arrow` | Move selection down |
| `k` / `Up Arrow` | Move selection up |
| `Enter` | Open article in in-TUI reader view |
| `t` | Open 72+ Cybercore Theme Chooser popup |
| `o` or `b` | Open article link in external browser |
| `y` | Copy article link to system clipboard (`arboard`) |
| `r` | Reload database and refresh article list |
| `q` / `Esc` | Quit app (or return to list view from reader) |

---

## ⚙️ Configuration

Cyberfeed is configured via `~/.config/cyberfeed/config.yaml`. Example configuration:

```yaml
theme: cyberpunk # Options: cyberpunk, dracula, nord
refresh_interval_secs: 300
feeds:
  - name: "Rust Blog"
    url: "[https://blog.rust-lang.org/feed.xml](https://blog.rust-lang.org/feed.xml)"
  - name: "Hacker News"
    url: "[https://news.ycombinator.com/rss](https://news.ycombinator.com/rss)"
  - name: "Arch Linux News"
    url: "[https://archlinux.org/feeds/news/](https://archlinux.org/feeds/news/)"
  - name: "LWN.net"
    url: "[https://lwn.net/headlines/newrss](https://lwn.net/headlines/newrss)"
```

---

## 📦 Installation & Building

```bash
git clone [https://github.com/cybercoretech/cyberfeed.git](https://github.com/cybercoretech/cyberfeed.git)
cd cyberfeed
cargo build --release
./target/release/cyberfeed
```

---

## 📄 License

Licensed under either of:
*   Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
*   MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
