use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourcePort {
    pub name: String,
    pub port_number: String,
}
