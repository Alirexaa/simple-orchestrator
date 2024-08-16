pub mod inspect;
pub mod run;
pub mod stats;

use std::collections::HashMap;

use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
    UNKNOWN,
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

/// Statistics for the container.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ContainerStats {
    pub read: String,
    pub preread: String,
    pub num_procs: u32,
    pub pids_stats: PidsStats,
    pub network: Option<NetworkStats>,
    pub networks: Option<HashMap<String, NetworkStats>>,
    pub memory_stats: MemoryStats,
    pub blkio_stats: BlkioStats,
    pub cpu_stats: CPUStats,
    pub precpu_stats: CPUStats,
    pub storage_stats: StorageStats,
    // #[serde(default = "empty_string")]
    pub name: String,
    pub id: String,
}

/// Process ID statistics for the container.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct PidsStats {
    pub current: Option<u64>,
    pub limit: Option<u64>,
}

/// General memory statistics for the container.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct MemoryStats {
    pub stats: Option<MemoryStatsStats>,
    pub max_usage: Option<u64>,
    pub usage: Option<u64>,
    pub failcnt: Option<u64>,
    pub limit: Option<u64>,
    pub commit: Option<u64>,
    pub commit_peak: Option<u64>,
    pub commitbytes: Option<u64>,
    pub commitpeakbytes: Option<u64>,
    pub privateworkingset: Option<u64>,
}

/// Granular memory statistics for the container.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
#[serde(untagged)]
pub enum MemoryStatsStats {
    V1(MemoryStatsStatsV1),
    V2(MemoryStatsStatsV2),
}

/// Granular memory statistics for the container, v1 cgroups.
///
/// Exposed in the docker library [here](https://github.com/moby/moby/blob/40d9e2aff130b42ba0f83d5238b9b53184c8ab3b/daemon/daemon_unix.go#L1436).
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
#[serde(deny_unknown_fields)]
pub struct MemoryStatsStatsV1 {
    pub cache: u64,
    pub dirty: u64,
    pub mapped_file: u64,
    pub total_inactive_file: u64,
    pub pgpgout: u64,
    pub rss: u64,
    pub total_mapped_file: u64,
    pub writeback: u64,
    pub unevictable: u64,
    pub pgpgin: u64,
    pub total_unevictable: u64,
    pub pgmajfault: u64,
    pub total_rss: u64,
    pub total_rss_huge: u64,
    pub total_writeback: u64,
    pub total_inactive_anon: u64,
    pub rss_huge: u64,
    pub hierarchical_memory_limit: u64,
    pub total_pgfault: u64,
    pub total_active_file: u64,
    pub active_anon: u64,
    pub total_active_anon: u64,
    pub total_pgpgout: u64,
    pub total_cache: u64,
    pub total_dirty: u64,
    pub inactive_anon: u64,
    pub active_file: u64,
    pub pgfault: u64,
    pub inactive_file: u64,
    pub total_pgmajfault: u64,
    pub total_pgpgin: u64,
    pub hierarchical_memsw_limit: Option<u64>, // only on OSX
    pub shmem: Option<u64>,                    // only on linux kernel > 4.15.0-1106
    pub total_shmem: Option<u64>,              // only on linux kernel > 4.15.0-1106
}

/// Granular memory statistics for the container, v2 cgroups.
///
/// Exposed in the docker library [here](https://github.com/moby/moby/blob/40d9e2aff130b42ba0f83d5238b9b53184c8ab3b/daemon/daemon_unix.go#L1542).
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
#[serde(deny_unknown_fields)]
pub struct MemoryStatsStatsV2 {
    pub anon: u64,
    pub file: u64,
    pub kernel_stack: u64,
    pub slab: u64,
    pub sock: u64,
    pub shmem: u64,
    pub file_mapped: u64,
    pub file_dirty: u64,
    pub file_writeback: u64,
    pub anon_thp: u64,
    pub inactive_anon: u64,
    pub active_anon: u64,
    pub inactive_file: u64,
    pub active_file: u64,
    pub unevictable: u64,
    pub slab_reclaimable: u64,
    pub slab_unreclaimable: u64,
    pub pgfault: u64,
    pub pgmajfault: u64,
    pub workingset_refault: u64,
    pub workingset_activate: u64,
    pub workingset_nodereclaim: u64,
    pub pgrefill: u64,
    pub pgscan: u64,
    pub pgsteal: u64,
    pub pgactivate: u64,
    pub pgdeactivate: u64,
    pub pglazyfree: u64,
    pub pglazyfreed: u64,
    pub thp_fault_alloc: u64,
    pub thp_collapse_alloc: u64,
}

/// I/O statistics for the container.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct BlkioStats {
    pub io_service_bytes_recursive: Option<Vec<BlkioStatsEntry>>,
    pub io_serviced_recursive: Option<Vec<BlkioStatsEntry>>,
    pub io_queue_recursive: Option<Vec<BlkioStatsEntry>>,
    pub io_service_time_recursive: Option<Vec<BlkioStatsEntry>>,
    pub io_wait_time_recursive: Option<Vec<BlkioStatsEntry>>,
    pub io_merged_recursive: Option<Vec<BlkioStatsEntry>>,
    pub io_time_recursive: Option<Vec<BlkioStatsEntry>>,
    pub sectors_recursive: Option<Vec<BlkioStatsEntry>>,
}

/// Network statistics for the container.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct NetworkStats {
    pub rx_dropped: u64,
    pub rx_bytes: u64,
    pub rx_errors: u64,
    pub tx_packets: u64,
    pub tx_dropped: u64,
    pub rx_packets: u64,
    pub tx_errors: u64,
    pub tx_bytes: u64,
}

/// CPU usage statistics for the container.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CPUUsage {
    pub percpu_usage: Option<Vec<u64>>,
    pub usage_in_usermode: u64,
    pub total_usage: u64,
    pub usage_in_kernelmode: u64,
}

/// CPU throttling statistics.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ThrottlingData {
    pub periods: u64,
    pub throttled_periods: u64,
    pub throttled_time: u64,
}

/// General CPU statistics for the container.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CPUStats {
    pub cpu_usage: CPUUsage,
    pub system_cpu_usage: Option<u64>,
    pub online_cpus: Option<u64>,
    pub throttling_data: ThrottlingData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct BlkioStatsEntry {
    pub major: u64,
    pub minor: u64,
    pub op: String,
    pub value: u64,
}

/// File I/O statistics for the container.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct StorageStats {
    pub read_count_normalized: Option<u64>,
    pub read_size_bytes: Option<u64>,
    pub write_count_normalized: Option<u64>,
    pub write_size_bytes: Option<u64>,
}
