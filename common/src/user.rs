use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum UserId {
    Hardware(u64),
    Steam(u64),
}

impl UserId {
    pub fn type_id(&self) -> u8 {
        match self {
            UserId::Hardware(_) => 0,
            UserId::Steam(_) => 1,
        }
    }
    pub fn inner(&self) -> u64 {
        match self {
            UserId::Hardware(inner) | UserId::Steam(inner) => *inner,
        }
    }
}
