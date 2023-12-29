use actix_web::{post, web, HttpResponse};
use futures::StreamExt;
use rusqlite::{params};

use super::super::super::util::error::CustomError;
use super::super::super::domain::entity::Todo;
use super::super::super::infrastructure::sqlite::init_db;

#[post("/todos")]
pub async fn create_todo(mut payload: web::Payload) -> Result<HttpResponse, CustomError> {
  let mut body = web::BytesMut::new();
  while let Some(chunk) = payload.next().await {
    let chunk = match chunk {
      Ok(chunk) => chunk,
      Err(err) => return Err(err.into()),
    };

    body.extend_from_slice(&chunk);
  }

  let todo = serde_json::from_slice::<Todo>(&body)
    .map_err(|err| CustomError {
       message: format!("{}", err),
    })?;

  let mut conn = init_db()?;

  let transaction = conn.transaction()?;
  transaction.execute(
    "INSERT INTO todos (title, contents) VALUES (?1, ?2)",
    params![&todo.title, &todo.contents],
  ).map_err(|err| CustomError {
    message: format!("Failed to insert todo into the database: {}", err),
  })?;
  transaction.commit()?;

  Ok(HttpResponse::Ok().json(todo))
}
