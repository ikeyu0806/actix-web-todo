use anyhow::{anyhow, Context, Result as AnyhowResult};
use rusqlite::{Connection};

pub fn init_db() -> AnyhowResult<Connection> {
  let conn = Connection::open_in_memory().with_context(|| anyhow!("Failed to initialize the database."))?;

  conn.execute(
    "CREATE TABLE IF NOT EXISTS todos (
      id INTEGER PRIMARY KEY,
      title TEXT NOT NULL,
      contents TEXT NOT NULL
    )",
    [],
  )
  .with_context(|| anyhow!("Failed to create todos table."))?;

  Ok(conn)
}
