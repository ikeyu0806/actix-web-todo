use actix_web::HttpResponse;
use std::fmt;

#[derive(Debug)]
pub struct CustomError {
    pub message: String,
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

impl From<anyhow::Error> for CustomError {
  fn from(error: anyhow::Error) -> Self {
      CustomError {
          message: error.to_string(),
      }
  }
}
