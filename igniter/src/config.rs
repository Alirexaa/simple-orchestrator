use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub start_command: String,
    pub path: String,
}

impl Config {
    /** Reads YAML config file from a given path and returns the config as struct*/
    pub fn from_file(path: &str) -> Config {
        let mut file = File::open(path).expect("Failed to open config file.");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Failed to read config file.");

        Config::from_str(&content)
    }
    /** convert given content to the config as struct*/
    pub fn from_str(content: &str) -> Config {
        let config_result: Result<Config, serde_yaml::Error> = serde_yaml::from_str(content);
        let config: Config = match config_result {
            Ok(config) => config,
            Err(err) => panic!("Failed to parse YAML: {}", err),
        };

        config
    }
}
