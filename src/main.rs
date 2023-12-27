use actix_web::{post, web, App, HttpResponse, HttpServer};
use futures::StreamExt;
use rusqlite::{params};

mod domain;
use domain::Todo;

mod infrastructure;
use infrastructure::init_db;

mod util;
use util::CustomError;

#[post("/todos")]
async fn create_todo(mut payload: web::Payload) -> Result<HttpResponse, CustomError> {
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

    let conn = init_db()?;

    conn.execute(
        "INSERT INTO todos (title, contents) VALUES (?1, ?2)",
        params![&todo.title, &todo.contents],
    ).map_err(|err| CustomError {
        message: format!("Failed to insert todo into the database: {}", err),
    })?;

    Ok(HttpResponse::Ok().json(todo))
}

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
