use serde::{Deserialize, Serialize};

use super::resource_port::ResourcePort;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutableResource {
    pub name: String,
    pub path: String,
    pub start_command: String,
    pub ports: Vec<ResourcePort>,
    pub args: Vec<String>,
    pub replica: i32,
}
