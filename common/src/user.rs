use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum UserID {
    Hardware(u64),
    Steam(u64),
}
