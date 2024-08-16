use bollard::{container::InspectContainerOptions, Docker};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{ContainerState, ContainerStateStatusEnum};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct InspectContainerResult {
    /// The ID of the container
    pub id: Option<String>,

    /// The time the container was created
    pub created: Option<String>,

    /// The path to the command being run
    pub path: Option<String>,

    /// The arguments to the command being run
    pub args: Option<Vec<String>>,

    pub state: Option<ContainerState>,

    /// The container's image ID
    pub image: Option<String>,

    pub resolv_conf_path: Option<String>,

    pub hostname_path: Option<String>,

    pub hosts_path: Option<String>,

    pub log_path: Option<String>,

    pub name: Option<String>,

    pub restart_count: Option<i64>,

    pub driver: Option<String>,

    pub platform: Option<String>,

    pub mount_label: Option<String>,

    pub process_label: Option<String>,

    pub app_armor_profile: Option<String>,

    /// IDs of exec instances that are running in the container.
    pub exec_ids: Option<Vec<String>>,

    // pub host_config: Option<HostConfig>,

    // pub graph_driver: Option<GraphDriverData>,
    /// The size of files that have been created or changed by this container.
    pub size_rw: Option<i64>,

    /// The total size of all the files in this container.
    pub size_root_fs: Option<i64>,
    // pub mounts: Option<Vec<MountPoint>>,

    // pub config: Option<ContainerConfig>,

    // pub network_settings: Option<NetworkSettings>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct InspectContainerCommand {
    pub name: String,
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
                            },
                        },
                    }),
                },
            })
        }
    }
}
