use super::super::super::domain::entity::Todo;
use super::super::super::util::error::CustomError;
use super::super::super::infrastructure::sqlite::init_db;
use rusqlite::{params};

pub trait TodoRepository {
  fn insert_todo(&self, todo: &Todo) -> Result<(), CustomError>;
  fn select_todo(&self, todo_id: i32) -> Result<Option<Todo>, CustomError>;
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

  fn select_todo(&self, todo_id: i32) -> Result<Option<Todo>, CustomError> {
    let conn = init_db()?;
    let mut stmt = conn.prepare("SELECT id, title, contents FROM todos WHERE id = ?1")?;
    let mut rows = stmt.query(params![todo_id])?;

    if let Some(row) = rows.next()? {
      let todo = Todo {
        id: Some(row.get(0)?),
        title: row.get(1)?,
        contents: row.get(2)?,
      };
      Ok(Some(todo))
    } else {
      Ok(None)
    }
}
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::super::super::super::domain::entity::Todo;

  #[test]
  fn test_todo_repo() {
    let test_todo = Todo {
      id: None,
      title: String::from("Test Title"),
      contents: String::from("Test Contents"),
    };

    let repo = TodoRepositoryImpl;

    let insert_result = repo.insert_todo(&test_todo);
    assert!(insert_result.is_ok(), "Failed to insert todo: {:?}", insert_result.err());

    let selected_todo_result = repo.select_todo(1);
    assert!(selected_todo_result.is_ok(), "Failed to select todo: {:?}", selected_todo_result.err());
  }
}
