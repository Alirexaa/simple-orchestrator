use rocket::post;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

use crate::container::{
    self,
    inspect::{InspectContainerCommand, InspectContainerResult},
    run::{RunContainerCommand, RunContainerResult},
};

use super::IgniterApiResult;

/// run docker container
#[openapi(tag = "containers")]
#[post("/containers/run", data = "<command>")]
pub async fn run_container(
    command: Json<RunContainerCommand>,
) -> IgniterApiResult<RunContainerResult> {
    let result = container::run::run_container(command.0).await;
    match result {
        Ok(v) => return IgniterApiResult::succeed(Some(v)),
        Err(e) => return IgniterApiResult::failure(e),
    }
}

/// run docker container
#[openapi(tag = "containers")]
#[post("/containers/inspect", data = "<command>")]
pub async fn inspect_container(
    command: Json<InspectContainerCommand>,
) -> IgniterApiResult<InspectContainerResult> {
    let result = container::inspect::inspect_container(command.0).await;
    match result {
        Ok(v) => return IgniterApiResult::succeed(Some(v)),
        Err(e) => return IgniterApiResult::failure(e),
    }
}
