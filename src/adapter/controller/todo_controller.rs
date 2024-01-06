use std::sync::Arc;
use actix_web::{get, post, web, test, App, HttpResponse, web::{Data, Path}};
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

#[get("/todos/{id}")]
pub async fn get_todo(
    todo_id: Path<i32>,
    todo_repo: Data<Arc<dyn TodoRepository + Send + Sync>>,
) -> Result<HttpResponse, CustomError> {
    let result = todo_repo.select_todo(todo_id.into_inner());
    match result {
        Ok(Some(todo)) => Ok(HttpResponse::Ok().json(todo)),
        Ok(None) => Ok(HttpResponse::NotFound().body("Todo not found")),
        Err(err) => Err(err),
    }
}

struct MockTodoRepository;

#[async_trait::async_trait]
impl TodoRepository for MockTodoRepository {
  fn insert_todo(&self, _todo: &Todo) -> Result<(), CustomError> {
    Ok(())
  }
  fn select_todo(&self, _todo_id: i32) -> Result<Option<Todo>, CustomError> {
    Ok(Some(Todo {
      id: Some(1),
      title: String::from("Test Title"),
      contents: String::from("Test Contents"),
    }))
  }
}

#[actix_rt::test]
async fn test_create_todo() {
  let todo_repo: Data<Arc<dyn TodoRepository + Send + Sync>> = Data::new(Arc::new(MockTodoRepository));

  let app = test::init_service(
    App::new().app_data(todo_repo.clone())
              .service(create_todo)
  ).await;

  let request = test::TestRequest::post()
    .uri("/todos")
    .set_json(&Todo {
      id: None,
      title: String::from("Test Title"),
      contents: String::from("Test Contents"),
    })
    .to_request();

  let response = test::call_service(&app, request).await;

  assert!(response.status().is_success());
}

#[actix_rt::test]
async fn test_get_todo() {
  let todo_repo: Data<Arc<dyn TodoRepository + Send + Sync>> = Data::new(Arc::new(MockTodoRepository));

  let app = test::init_service(
    App::new().app_data(todo_repo.clone())
              .service(get_todo)
  ).await;

  let request = test::TestRequest::get()
    .uri("/todos/1")
    .to_request();

  let response = test::call_service(&app, request).await;
 
  assert!(response.status().is_success());
}
