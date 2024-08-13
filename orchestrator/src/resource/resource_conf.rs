use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

use super::resource_type::ResourceType;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceConfig {
    pub name: String,
    pub resouce_type: ResourceType,
    pub path: String,
    pub image: String,
    pub ports: Vec<i32>,
    pub args: Vec<String>,
    pub volumes: Vec<String>,
    pub replica: i32,
}

impl ResourceConfig {
    pub fn from_json_str(json_str: &str) -> ResourceConfig {
        let convert_result: Result<ResourceConfig, serde_yaml::Error> =
            serde_yaml::from_str(json_str);
        match convert_result {
            Ok(config) => config,
            Err(err) => panic!("Failed to parse YAML: {}", err),
        }
    }

    pub fn from_file(path: &str) -> ResourceConfig {
        let mut file = File::open(path).expect("Failed to open config file");
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)
            .expect("Failed to read cofig file");

        ResourceConfig::from_json_str(&file_content)
    }
}
