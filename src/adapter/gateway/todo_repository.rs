use super::super::super::domain::entity::Todo;
use super::super::super::util::error::CustomError;
use super::super::super::infrastructure::sqlite::init_db;
use rusqlite::{params};

pub fn insert_todo(todo: &Todo) -> Result<(), CustomError> {
  let mut conn = init_db()?;

  let transaction = conn.transaction()?;
  transaction
    .execute(
      "INSERT INTO todos (title, contents) VALUES (?1, ?2)",
      params![&todo.title, &todo.contents],
    )
    .map_err(|err| CustomError {
      message: format!("Failed to insert todo into the database: {}", err),
    })?;
  transaction.commit()?;

  Ok(())
}
