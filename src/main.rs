use actix_web::{post, web, App, HttpResponse, HttpServer, error::Error};
use actix_web::error;
use futures::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Todo {
    title: String,
    contents: String,
}

const MAX_SIZE: usize = 262_144;

#[post("/todos")]
async fn create_todo(mut payload: web::Payload) -> Result<HttpResponse, Error> {
  // payload is a stream of Bytes objects
  let mut body = web::BytesMut::new();
  while let Some(chunk) = payload.next().await {
      let chunk = chunk?;
      // limit max size of in-memory payload
      if (body.len() + chunk.len()) > MAX_SIZE {
          return Err(error::ErrorBadRequest("overflow"));
      }
      body.extend_from_slice(&chunk);
  }

  // body is loaded, now we can deserialize serde-json
  let todo = serde_json::from_slice::<Todo>(&body)?;
  Ok(HttpResponse::Ok().json(todo)) // <- send response
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
