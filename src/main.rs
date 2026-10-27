mod config;
mod db;
mod fetcher;
mod ui;

use anyhow::Result;
use db::Database;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = config::load_config()?;
    let database = Database::init()?;

    // Fetch feeds synchronously on startup so articles are present before TUI loads
    fetcher::fetch_all_feeds(&cfg, &database).await;

    let app = ui::App::new(database, cfg.theme)?;
    ui::run_tui(app).await?;

    Ok(())
}
