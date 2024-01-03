use std::sync::Arc;
use actix_web::{post, web, HttpResponse, web::{Data}};
use futures::StreamExt;
use super::super::super::util::error::CustomError;
use super::super::super::domain::entity::Todo;
use super::super::gateway::todo_repository::TodoRepository;

#[post("/todos")]
pub async fn create_todo(
  mut payload: web::Payload,
  todo_repo: Data<Arc<dyn TodoRepository + Send + Sync>>,
) -> Result<HttpResponse, CustomError> {
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

  match todo_repo.insert_todo(&todo) {
    Ok(_) => Ok(HttpResponse::Ok().json(todo)),
    Err(err) => Err(err),
  }
}
