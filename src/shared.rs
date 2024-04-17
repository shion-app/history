use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Browser {
    pub name: String,
    pub last_sync: u64,
}
