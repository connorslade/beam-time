use std::net::IpAddr;

use native_db::{native_db, ToKey};
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};

use beam_logic::tile::Tile;
use common::{map::Map, user::UserId};

use crate::database::id_key::IdKey;

#[derive(Serialize, Deserialize, PartialEq)]
#[native_model(id = 1, version = 1)]
#[native_db]
pub struct Results {
    #[primary_key]
    pub id: IdKey,

    pub user_id: UserId,
    pub ip_address: IpAddr,
    pub timestamp: u64,

    #[secondary_key]
    pub level_id: IdKey,
    pub cost: u32,
    pub latency: u32,

    pub solution: Map<Tile>,
}
