use anyhow::Result;
use rusqlite::{params, Connection};
use std::path::PathBuf;

#[allow(dead_code)]
pub struct Article {
    pub id: i64,
    pub title: String,
    pub source: String,
    pub content: String,
    pub link: String,
    pub read: bool,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    #[cfg(test)]
    fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch(
            "CREATE TABLE articles (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                source TEXT NOT NULL DEFAULT 'Feed',
                content TEXT NOT NULL DEFAULT '',
                link TEXT UNIQUE NOT NULL,
                read INTEGER DEFAULT 0,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );",
        )?;
        Ok(Self { conn })
    }

    pub fn init() -> Result<Self> {
        let mut db_path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("~/.local/share"));
        db_path.push("cyberfeed");
        std::fs::create_dir_all(&db_path).ok();
        db_path.push("cyberfeed.db");

        let conn = Connection::open(db_path)?;

        // Non-destructive table creation
        conn.execute(
            "CREATE TABLE IF NOT EXISTS articles (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                source TEXT NOT NULL DEFAULT 'Feed',
                content TEXT NOT NULL DEFAULT '',
                link TEXT UNIQUE NOT NULL,
                read INTEGER DEFAULT 0,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // Non-destructive column migrations for legacy databases
        let _ = conn.execute(
            "ALTER TABLE articles ADD COLUMN source TEXT NOT NULL DEFAULT 'Feed'",
            [],
        );
        let _ = conn.execute(
            "ALTER TABLE articles ADD COLUMN content TEXT NOT NULL DEFAULT ''",
            [],
        );
        let _ = conn.execute("ALTER TABLE articles ADD COLUMN read INTEGER DEFAULT 0", []);

        Ok(Self { conn })
    }

    pub fn insert_article(
        &self,
        title: &str,
        source: &str,
        content: &str,
        link: &str,
    ) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO articles (title, source, content, link) VALUES (?1, ?2, ?3, ?4)",
            params![title, source, content, link],
        )?;
        Ok(())
    }

    pub fn get_articles(&self) -> Result<Vec<Article>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, source, content, link, read FROM articles ORDER BY id DESC LIMIT 100",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(Article {
                id: row.get(0)?,
                title: row.get(1)?,
                source: row.get(2)?,
                content: row.get(3)?,
                link: row.get(4)?,
                read: row.get::<_, i32>(5)? != 0,
            })
        })?;

        let mut articles = Vec::new();
        for row in rows {
            articles.push(row?);
        }
        Ok(articles)
    }

    #[allow(dead_code)]
    pub fn toggle_read(&self, id: i64, read: bool) -> Result<()> {
        self.conn.execute(
            "UPDATE articles SET read = ?1 WHERE id = ?2",
            params![if read { 1 } else { 0 }, id],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_db() -> Database {
        Database::in_memory().expect("create in-memory database")
    }

    #[test]
    fn inserts_and_retrieves_article() {
        let db = test_db();
        db.insert_article(
            "Test title",
            "Test source",
            "Test content",
            "https://example.com/1",
        )
        .unwrap();

        let articles = db.get_articles().unwrap();
        assert_eq!(articles.len(), 1);
        assert_eq!(articles[0].title, "Test title");
        assert_eq!(articles[0].source, "Test source");
        assert_eq!(articles[0].content, "Test content");
        assert_eq!(articles[0].link, "https://example.com/1");
        assert!(!articles[0].read);
    }

    #[test]
    fn ignores_duplicate_links() {
        let db = test_db();
        db.insert_article("Original", "Source A", "", "https://example.com/same")
            .unwrap();
        db.insert_article("Duplicate", "Source B", "", "https://example.com/same")
            .unwrap();

        let articles = db.get_articles().unwrap();
        assert_eq!(articles.len(), 1);
        assert_eq!(articles[0].title, "Original");
    }

    #[test]
    fn toggles_read_state() {
        let db = test_db();
        db.insert_article("Test", "Source", "", "https://example.com/read")
            .unwrap();

        let id = db.get_articles().unwrap()[0].id;
        db.toggle_read(id, true).unwrap();
        assert!(db.get_articles().unwrap()[0].read);

        db.toggle_read(id, false).unwrap();
        assert!(!db.get_articles().unwrap()[0].read);
    }

    #[test]
    fn returns_at_most_100_articles_newest_first() {
        let db = test_db();
        for i in 0..105 {
            db.insert_article(
                &format!("Article {i}"),
                "Test",
                "",
                &format!("https://example.com/{i}"),
            )
            .unwrap();
        }

        let articles = db.get_articles().unwrap();
        assert_eq!(articles.len(), 100);
        assert_eq!(articles[0].title, "Article 104");
        assert_eq!(articles[99].title, "Article 5");
    }
}
