use std::net::IpAddr;

use native_db::{native_db,  ToKey};
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use beam_logic::tile::Tile;
use common::{map::Map, user::UserID};

use crate::database::id_key::IdKey;

#[derive(Serialize, Deserialize, PartialEq)]
#[native_model(id = 1, version = 1)]
#[native_db]
pub struct Results {
    #[primary_key]
    id: IdKey,

    user_id: UserID,
    ip_address: IpAddr,
    timestamp: u64,

    level_id: Uuid,
    cost: u32,
    latency: u32,

    solution: Map<Tile>,
}
