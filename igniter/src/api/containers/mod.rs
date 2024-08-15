use crate::container::{InspectContainerCommand, InspectContainerResult, RunContainerCommand};
use crate::ctr;
use bollard::secret::ContainerInspectResponse;
use rocket::post;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, JsonSchema};

/// run docker container
#[openapi(tag = "containers")]
#[post("/containers/run", data = "<command>")]
pub async fn run_container(command: Json<RunContainerCommand>) -> Result<String, String> {
    let result = ctr::container_ctr::run_container(command.0).await;
    match result {
        Ok(v) => return Ok(v),
        Err(e) => return Err(e.to_string()),
    }
}

/// run docker container
#[openapi(tag = "containers")]
#[post("/containers/inspect", data = "<command>")]
pub async fn inspect_container(
    command: Json<InspectContainerCommand>,
) -> Result<Json<InspectContainerResult>, String> {
    let result = ctr::container_ctr::inspect_container(command.0).await;
    match result {
        Ok(v) => return Ok(Json(v)),
        Err(e) => return Err(e.to_string()),
    }
}
