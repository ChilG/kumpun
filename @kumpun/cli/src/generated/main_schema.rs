use serde::{Deserialize, Serialize};

use crate::generated::shared::address::Address;

#[derive(Debug, Serialize, Deserialize)]
pub struct MainSchema {
    pub address: Address,
}