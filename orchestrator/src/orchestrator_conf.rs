use std::{collections::HashMap, fs::File, io::Read};

use serde::{Deserialize, Serialize};

use crate::{
    resource::{container_resource::ContainerResource, excutable_resource::ExecutableResource},
    server_conf::ServerConfig,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrchestratorConfig {
    pub servers: HashMap<String, ServerConfig>,
    pub executable: HashMap<String, ExecutableResource>,
    pub containers: HashMap<String, ContainerResource>,
    pub mapping: HashMap<String, Vec<ServerResourceMapping>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerResourceMapping {
    pub executable: String,
    pub executable_path: String,
    pub container: String,
}

impl OrchestratorConfig {
    pub fn from_json_str(json_str: &str) -> OrchestratorConfig {
        let convert_result: Result<OrchestratorConfig, serde_yaml::Error> =
            serde_yaml::from_str(json_str);
        match convert_result {
            Ok(config) => config,
            Err(err) => panic!("Failed to parse YAML: {}", err),
        }
    }

    pub fn from_file(path: &str) -> OrchestratorConfig {
        let mut file = File::open(path).expect("Failed to open config file");
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)
            .expect("Failed to read cofig file");

        OrchestratorConfig::from_json_str(&file_content)
    }
}
