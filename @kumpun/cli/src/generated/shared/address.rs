use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub city: Option<String>,
    pub zip: Option<String>,
}