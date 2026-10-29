mod config;
mod db;
mod fetcher;
mod ui;

use anyhow::Result;
use db::Database;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::args().nth(1).as_deref() == Some("daemon") {
        return run_daemon().await;
    }

    let cfg = config::load_config()?;
    let database = Database::init()?;

    let app = ui::App::new(database, cfg.theme)?;
    ui::run_tui(app).await?;

    Ok(())
}

async fn run_daemon() -> Result<()> {
    let cfg = config::load_config()?;
    let database = Database::init()?;
    let interval = Duration::from_secs(cfg.refresh_interval_secs.max(30));

    eprintln!(
        "Cyberfeed daemon started; refresh interval: {} seconds",
        interval.as_secs()
    );

    loop {
        eprintln!("Refreshing {} configured feeds", cfg.feeds.len());
        fetcher::fetch_all_feeds(&cfg, &database).await;
        eprintln!(
            "Feed refresh complete; next refresh in {} seconds",
            interval.as_secs()
        );
        tokio::time::sleep(interval).await;
    }
}
