use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Browser {
    pub name: String,
    pub last_sync: u64,
}
