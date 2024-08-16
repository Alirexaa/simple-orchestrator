use std::io::Cursor;

use rocket::{
    http::{ContentType, Status},
    response::{self, Responder, Response},
    serde,
};
use rocket_okapi::okapi::schemars::{self, schema_for};
use rocket_okapi::{
    okapi::{openapi3::Responses, schemars::JsonSchema},
    response::OpenApiResponderInner,
};
use serde::{Deserialize, Serialize};

pub mod containers;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct IgniterApiResult<T: Serialize + JsonSchema> {
    pub is_success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize + JsonSchema> IgniterApiResult<T> {
    pub fn failure(error: String) -> IgniterApiResult<T> {
        IgniterApiResult {
            data: None::<T>,
            error: Some(error),
            is_success: false,
        }
    }
    pub fn succeed(data: Option<T>) -> IgniterApiResult<T> {
        IgniterApiResult {
            data: data,
            error: None,
            is_success: true,
        }
    }
}

impl<T: Serialize + JsonSchema> std::fmt::Display for IgniterApiResult<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self)
    }
}

impl<'r, T: Serialize + JsonSchema> Responder<'r, 'static> for IgniterApiResult<T> {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> response::Result<'static> {
        let body = serde::json::to_string(&self).unwrap();
        Response::build()
            .status(Status::Ok)
            .header(ContentType::JSON)
            .sized_body(body.len(), Cursor::new(body))
            .ok()
    }
}

impl<T: Serialize + JsonSchema> OpenApiResponderInner for IgniterApiResult<T> {
    fn responses(
        _gen: &mut rocket_okapi::r#gen::OpenApiGenerator,
    ) -> rocket_okapi::Result<rocket_okapi::okapi::openapi3::Responses> {
        use rocket_okapi::util::add_schema_response;
        let mut responses = Responses::default();
        let schema = schema_for!(T).schema;
        add_schema_response(&mut responses, 200, "application/json", schema)?;
        Ok(responses)
    }
}
