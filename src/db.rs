use rusqlite::{Connection, Result, params};

#[derive(Debug)]
pub struct Score {
    conn: Connection,
}

impl Score {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("r2048.db")?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS data (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                score INTEGER NOT NULL
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    pub fn update_score(&self, new_score: u32) -> Result<()> {
        let current_high: u32 = self
            .conn
            .query_row("SELECT score FROM data WHERE id = 1", [], |row| row.get(0))
            .unwrap_or(0);

        if new_score > current_high {
            self.conn.execute(
                "INSERT INTO data (id, score) VALUES (1, ?1)
                 ON CONFLICT(id) DO UPDATE SET score = excluded.score",
                params![new_score],
            )?;
        }

        Ok(())
    }

    pub fn get_score(&self) -> Result<u32> {
        let score = self
            .conn
            .query_row("SELECT score FROM data WHERE id = 1", [], |row| row.get(0))
            .unwrap_or(0);
        Ok(score)
    }
}
