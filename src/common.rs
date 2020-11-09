use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, content, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct JsonPayload<T> {
  result: Option<T>,
  error: Option<String>,
}

impl<T> JsonPayload<T> {
  pub fn from_result(result: T) -> JsonPayload<T> {
    JsonPayload {
      result: Some(result),
      error: None,
    }
  }

  pub fn from_error_message(error_message: String) -> JsonPayload<T> {
    JsonPayload {
      result: None,
      error: Some(error_message),
    }
  }
}

impl<'a, T: Serialize> Responder<'a> for JsonPayload<T> {
  fn respond_to(self, req: &Request) -> response::Result<'a> {
    serde_json::to_string(&self)
      .map(|string| content::Json(string).respond_to(req).unwrap())
      .map_err(|_e| {
        // error_!("JSON failed to serialize: {:?}", e);

        Status::InternalServerError
      })
  }
}
