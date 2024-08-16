use bollard::{container::StatsOptions, Docker};
use rocket::futures::{pin_mut, StreamExt};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct StatsContainerCommand {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct StatsContainerResult {
    pub name: String,
}

/// run docker container
pub async fn stats_container(
    command: StatsContainerCommand,
) -> Result<StatsContainerResult, String> {
    let docker = Docker::connect_with_socket_defaults().unwrap();
    docker.ping().await.unwrap();

    let options = StatsOptions {
        stream: false,
        one_shot: true,
    };

    let stream = docker.stats(&command.name, Some(options));
    pin_mut!(stream);
    let stats_result = stream.next().await.unwrap();
    match stats_result {
        Err(e) => return Err(e.to_string()),
        //todo: map fields
        Ok(v) => return Ok(StatsContainerResult { name: v.name }),
    }
}
