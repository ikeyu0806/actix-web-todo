use std::path::Path;
use anyhow::{anyhow, Context, Result as AnyhowResult};
use rusqlite::{Connection};

pub fn init_db() -> AnyhowResult<Connection> {
  let db_path = Path::new("todo.db");
  let conn = Connection::open(db_path).with_context(|| anyhow!("Failed to initialize the database."))?;

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
