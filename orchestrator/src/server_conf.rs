use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub address: String,
    pub igniter_port: String,
    pub name: String,
}

impl ServerConfig {
    pub fn from_json_str(json_str: &str) -> ServerConfig {
        let convert_result: Result<ServerConfig, serde_yaml::Error> =
            serde_yaml::from_str(json_str);
        match convert_result {
            Ok(config) => config,
            Err(err) => panic!("Failed to parse YAML: {}", err),
        }
    }

    pub fn from_file(path: &str) -> ServerConfig {
        let mut file = File::open(path).expect("Failed to open config file");
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)
            .expect("Failed to read cofig file");

        ServerConfig::from_json_str(&file_content)
    }
}
