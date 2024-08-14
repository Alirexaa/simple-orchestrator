use crate::container::RunContainerCommand;
use crate::ctr;
use rocket::post;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

#[openapi(tag = "containers")]
#[post("/containers/run", data = "<command>")]
pub async fn run_container(command: Json<RunContainerCommand>) -> String {
    let _ = ctr::container_ctr::run_container(command.0).await;
    String::from("value")
}
