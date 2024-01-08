use std::sync::Arc;
use actix_web::{get, post, delete, web, test, App, HttpResponse, web::{Data, Path}};
use futures::StreamExt;
use crate::custom_error::CustomError;
use crate::domain::entity::Todo;
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

#[post("/todos/{id}")]
pub async fn update_todo(
  todo_id: Path<i32>,
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

  let updated_todo = Todo {
    id: Some(todo_id.into_inner().into()),
    title: todo.title,
    contents: todo.contents,
  };

  match todo_repo.update_todo(&updated_todo) {
    Ok(_) => Ok(HttpResponse::Ok().json(updated_todo)),
    Err(err) => Err(err),
  }
}

#[delete("/todos/{id}")]
pub async fn delete_todo(
  todo_id: Path<i32>,
  todo_repo: Data<Arc<dyn TodoRepository + Send + Sync>>,
) -> Result<HttpResponse, CustomError> {
  match todo_repo.delete_todo(todo_id.into_inner().into()) {
    Ok(_) => Ok(HttpResponse::Ok().body("Todo deleted")),
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
  fn update_todo(&self, _todo: &Todo) -> Result<(), CustomError> {
    Ok(())
  }
  fn delete_todo(&self, _todo_id: i32) -> Result<(), CustomError> {
    Ok(())
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

#[actix_rt::test]
async fn test_update_todo() {
  let todo_repo: Data<Arc<dyn TodoRepository + Send + Sync>> = Data::new(Arc::new(MockTodoRepository));

  let app = test::init_service(
    App::new()
      .app_data(todo_repo.clone())
      .service(update_todo),
  )
  .await;

  let request = test::TestRequest::post()
    .uri("/todos/1")
    .set_json(&Todo {
      id: Some(1),
      title: String::from("Updated Title"),
      contents: String::from("Updated Contents"),
    })
    .to_request();

  let response = test::call_service(&app, request).await;

  assert!(response.status().is_success());
}

#[actix_rt::test]
async fn test_delete_todo() {
  let todo_repo: Data<Arc<dyn TodoRepository + Send + Sync>> = Data::new(Arc::new(MockTodoRepository));

  let app = test::init_service(
    App::new()
      .app_data(todo_repo.clone())
      .service(delete_todo),
  )
  .await;

  let request = test::TestRequest::delete()
    .uri("/todos/1")
    .to_request();

  let response = test::call_service(&app, request).await;

  assert!(response.status().is_success());
}
