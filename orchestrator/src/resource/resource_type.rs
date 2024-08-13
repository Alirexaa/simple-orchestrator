use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceType {
    Executable,
    Container,
}

impl ResourceType {
    pub fn from_string(resource_type: &String) -> ResourceType {
        match resource_type.as_str() {
            "executable" => ResourceType::Executable,
            "container" => ResourceType::Container,
            _ => todo!(),
        }
    }
}
