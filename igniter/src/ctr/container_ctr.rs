use bollard::{
    container::{Config, CreateContainerOptions},
    secret::ContainerCreateResponse,
    Docker,
};

use crate::container::RunContainerCommand;

///* run docker container
pub async fn run_container(
    command: RunContainerCommand,
) -> Result<ContainerCreateResponse, bollard::errors::Error> {
    let docker = Docker::connect_with_socket_defaults().unwrap();

    let options = Some(CreateContainerOptions {
        name: command.name,
        platform: None,
    });

    let config = Config {
        image: Some(command.image),
        cmd: Some(vec![]),
        ..Default::default()
    };

    let result = docker.create_container(options, config).await;
    result
}
