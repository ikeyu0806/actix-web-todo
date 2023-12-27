use actix_web::{post, web, App, HttpResponse, HttpServer};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use rusqlite::{params, Connection, Result};
use std::fmt;

#[derive(Debug)]
struct CustomError {
    message: String,
}

impl actix_web::error::ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().body(self.message.clone())
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<actix_web::error::PayloadError> for CustomError {
    fn from(_: actix_web::error::PayloadError) -> Self {
        CustomError {
            message: "Payload error".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Todo {
    id: Option<i64>,
    title: String,
    contents: String,
}

fn init_db() -> Result<Connection, CustomError> {
    let conn = Connection::open_in_memory().map_err(|_| CustomError {
        message: "Failed to initialize the database.".to_string(),
    })?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            contents TEXT NOT NULL
        )",
        [],
    )
    .map_err(|_| CustomError {
        message: "Failed to create todos table.".to_string(),
    })?;

    Ok(conn)
}

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

    println!("Value of todo.title: {}", todo.title);
    println!("Value of &todo.title: {}", &todo.title);
    println!("Value of todo.contents: {}", todo.contents);
    println!("Value of &todo.contents: {}", &todo.contents);

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
