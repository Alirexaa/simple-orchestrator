use bollard::{
    container::{Config, CreateContainerOptions, StartContainerOptions},
    Docker,
};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct RunContainerResult {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct RunContainerCommand {
    pub name: String,
    pub image: String,
}

/// run docker container
pub async fn run_container(command: RunContainerCommand) -> Result<RunContainerResult, String> {
    let docker = Docker::connect_with_socket_defaults().unwrap();
    docker.ping().await.unwrap();
    let options = Some(CreateContainerOptions {
        name: &command.name,
        platform: None,
    });

    let config = Config {
        image: Some(command.image),
        cmd: Some(vec![]),
        ..Default::default()
    };

    let create_result = docker.create_container(options, config).await;
    match create_result {
        Err(e) => return Err(e.to_string()),
        Ok(v) => {
            let start_result = docker
                .start_container(&command.name, None::<StartContainerOptions<String>>)
                .await;
            match start_result {
                Err(e) => Err(e.to_string()),
                Ok(()) => Ok(RunContainerResult { id: v.id }),
            }
        }
    }
}
