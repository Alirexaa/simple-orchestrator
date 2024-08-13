use serde::{Deserialize, Serialize};

use super::resource_port::ResourcePort;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerResource {
    pub name: String,
    pub image: String,
    pub ports: Vec<ResourcePort>,
    pub args: Vec<String>,
    pub volumes: Vec<String>,
    pub replica: i32,
}
