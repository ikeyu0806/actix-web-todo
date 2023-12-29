use actix_web::{App, HttpServer};

mod adapter;
use adapter::controller::todo_controller::create_todo;
mod domain;
mod infrastructure;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
        .service(create_todo)
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await
}
