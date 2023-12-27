use super::util::CustomError;

use rusqlite::{Connection, Result};

pub fn init_db() -> Result<Connection, CustomError> {
  let conn = Connection::open_in_memory().map_err(|_| CustomError {
      message: "Failed to initialize the database.".to_string(),
  })?;

  conn.execute(
      "CREATE TABLE IF NOT EXISTS todos (
          id INTEGER PRIMARY KEY,
          title TEXT NOT NULL,
          contents TEXT NOT NULL
      )",
      [],
  )
  .map_err(|_| CustomError {
      message: "Failed to create todos table.".to_string(),
  })?;

  Ok(conn)
}
