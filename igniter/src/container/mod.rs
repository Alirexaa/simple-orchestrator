use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct RunContainerCommand {
    pub name: String,
    pub image: String,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct InspectContainerCommand {
    pub name: String,
}

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

/// ContainerState stores container's running state. It's part of ContainerJSONBase and will be returned by the \"inspect\" command.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ContainerState {
    /// String representation of the container state. Can be one of \"created\", \"running\", \"paused\", \"restarting\", \"removing\", \"exited\", or \"dead\".
    pub status: Option<ContainerStateStatusEnum>,

    /// Whether this container is running.  Note that a running container can be _paused_. The `Running` and `Paused` booleans are not mutually exclusive:  When pausing a container (on Linux), the freezer cgroup is used to suspend all processes in the container. Freezing the process requires the process to be running. As a result, paused containers are both `Running` _and_ `Paused`.  Use the `Status` field instead to determine if a container's state is \"running\".
    pub running: Option<bool>,

    /// Whether this container is paused.
    pub paused: Option<bool>,

    /// Whether this container is restarting.
    pub restarting: Option<bool>,

    /// Whether a process within this container has been killed because it ran out of memory since the container was last started.
    pub oom_killed: Option<bool>,

    pub dead: Option<bool>,

    /// The process ID of this container
    pub pid: Option<i64>,

    /// The last exit code of this container
    pub exit_code: Option<i64>,

    pub error: Option<String>,

    /// The time when this container was last started.
    pub started_at: Option<String>,

    /// The time when this container last exited.
    pub finished_at: Option<String>,
    // pub health: Option<Health>,
}

#[allow(non_camel_case_types)]
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, JsonSchema,
)]
pub enum ContainerStateStatusEnum {
    #[serde(rename = "")]
    EMPTY,
    #[serde(rename = "created")]
    CREATED,
    #[serde(rename = "running")]
    RUNNING,
    #[serde(rename = "paused")]
    PAUSED,
    #[serde(rename = "restarting")]
    RESTARTING,
    #[serde(rename = "removing")]
    REMOVING,
    #[serde(rename = "exited")]
    EXITED,
    #[serde(rename = "dead")]
    DEAD,
    #[serde(rename = "unknown")]
    UNKHOWN,
}

impl ::std::fmt::Display for ContainerStateStatusEnum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ContainerStateStatusEnum::EMPTY => write!(f, ""),
            ContainerStateStatusEnum::CREATED => write!(f, "{}", "created"),
            ContainerStateStatusEnum::RUNNING => write!(f, "{}", "running"),
            ContainerStateStatusEnum::PAUSED => write!(f, "{}", "paused"),
            ContainerStateStatusEnum::RESTARTING => write!(f, "{}", "restarting"),
            ContainerStateStatusEnum::REMOVING => write!(f, "{}", "removing"),
            ContainerStateStatusEnum::EXITED => write!(f, "{}", "exited"),
            ContainerStateStatusEnum::DEAD => write!(f, "{}", "dead"),
            _ => write!(f, "{}", "unkhown"),
        }
    }
}

impl ::std::str::FromStr for ContainerStateStatusEnum {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(ContainerStateStatusEnum::EMPTY),
            "created" => Ok(ContainerStateStatusEnum::CREATED),
            "running" => Ok(ContainerStateStatusEnum::RUNNING),
            "paused" => Ok(ContainerStateStatusEnum::PAUSED),
            "restarting" => Ok(ContainerStateStatusEnum::RESTARTING),
            "removing" => Ok(ContainerStateStatusEnum::REMOVING),
            "exited" => Ok(ContainerStateStatusEnum::EXITED),
            "dead" => Ok(ContainerStateStatusEnum::DEAD),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for ContainerStateStatusEnum {
    fn as_ref(&self) -> &str {
        match self {
            ContainerStateStatusEnum::EMPTY => "",
            ContainerStateStatusEnum::CREATED => "created",
            ContainerStateStatusEnum::RUNNING => "running",
            ContainerStateStatusEnum::PAUSED => "paused",
            ContainerStateStatusEnum::RESTARTING => "restarting",
            ContainerStateStatusEnum::REMOVING => "removing",
            ContainerStateStatusEnum::EXITED => "exited",
            ContainerStateStatusEnum::DEAD => "dead",
            _ => "unkhown",
        }
    }
}
