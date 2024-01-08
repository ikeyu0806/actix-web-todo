use std::sync::Arc;
use actix_web::{middleware::Logger, App, HttpServer, web::{Data}};

mod adapter;
use adapter::controller::todo_controller::*;
use adapter::gateway::todo_repository::{TodoRepositoryImpl, TodoRepository};
mod domain;
mod infrastructure;
mod custom_error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let todo_repo: Data<Arc<dyn TodoRepository + Send + Sync>> = Data::new(Arc::new(TodoRepositoryImpl));

  HttpServer::new(move || {
    App::new()
      .app_data(todo_repo.clone())
      .service(create_todo)
      .service(get_todo)
      .service(update_todo)
      .service(delete_todo)
      .wrap(Logger::default())
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await
}
