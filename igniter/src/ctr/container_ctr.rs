use bollard::{
    container::{Config, CreateContainerOptions, InspectContainerOptions, StartContainerOptions},
    Docker,
};

use crate::container::*;

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

/// inspect docker container
pub async fn inspect_container(
    command: InspectContainerCommand,
) -> Result<InspectContainerResult, String> {
    let docker = Docker::connect_with_socket_defaults().unwrap();
    docker.ping().await.unwrap();
    let options = Some(InspectContainerOptions { size: true });

    let inspect_result = docker
        .inspect_container(command.name.as_str(), options)
        .await;
    match inspect_result {
        Err(e) => return Err(e.to_string()),
        Ok(v) => {
            return Ok(InspectContainerResult {
                id: v.id,
                created: v.created,
                driver: v.driver,
                app_armor_profile: v.app_armor_profile,
                args: v.args,
                exec_ids: v.exec_ids,
                hostname_path: v.hostname_path,
                image: v.image,
                hosts_path: v.hosts_path,
                log_path: v.log_path,
                mount_label: v.mount_label,
                name: v.name,
                path: v.path,
                platform: v.platform,
                process_label: v.process_label,
                resolv_conf_path: v.resolv_conf_path,
                restart_count: v.restart_count,
                size_root_fs: v.size_root_fs,
                size_rw: v.size_rw,
                state: match v.state {
                    None => None,
                    Some(s) => Some(ContainerState {
                        dead: s.dead,
                        error: s.error,
                        exit_code: s.exit_code,
                        finished_at: s.finished_at,
                        oom_killed: s.oom_killed,
                        paused: s.paused,
                        pid: s.pid,
                        restarting: s.restarting,
                        running: s.running,
                        started_at: s.started_at,
                        status: match s.status {
                            None => None,
                            Some(st) => match st {
                                bollard::secret::ContainerStateStatusEnum::CREATED => {
                                    Some(ContainerStateStatusEnum::CREATED)
                                }
                                bollard::secret::ContainerStateStatusEnum::DEAD => {
                                    Some(ContainerStateStatusEnum::DEAD)
                                }
                                bollard::secret::ContainerStateStatusEnum::EMPTY => {
                                    Some(ContainerStateStatusEnum::EMPTY)
                                }
                                bollard::secret::ContainerStateStatusEnum::EXITED => {
                                    Some(ContainerStateStatusEnum::EXITED)
                                }
                                bollard::secret::ContainerStateStatusEnum::PAUSED => {
                                    Some(ContainerStateStatusEnum::PAUSED)
                                }
                                bollard::secret::ContainerStateStatusEnum::REMOVING => {
                                    Some(ContainerStateStatusEnum::REMOVING)
                                }
                                bollard::secret::ContainerStateStatusEnum::RESTARTING => {
                                    Some(ContainerStateStatusEnum::RESTARTING)
                                }
                                bollard::secret::ContainerStateStatusEnum::RUNNING => {
                                    Some(ContainerStateStatusEnum::RUNNING)
                                }
                                _ => Some(ContainerStateStatusEnum::UNKHOWN),
                            },
                        },
                    }),
                },
            })
        }
    }
}
