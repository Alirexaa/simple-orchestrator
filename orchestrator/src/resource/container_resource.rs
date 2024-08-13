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

impl ContainerResource {
    pub fn as_json_str(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
