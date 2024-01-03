use super::super::super::domain::entity::Todo;
use super::super::super::util::error::CustomError;
use super::super::super::infrastructure::sqlite::init_db;
use rusqlite::{params};

pub trait TodoRepository {
  fn insert_todo(&self, todo: &Todo) -> Result<(), CustomError>;
}

pub struct TodoRepositoryImpl;

impl TodoRepository for TodoRepositoryImpl {
  fn insert_todo(&self, todo: &Todo) -> Result<(), CustomError> {
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
}

