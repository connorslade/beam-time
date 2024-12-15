use std::net::Ipv4Addr;

use anyhow::Result;
use beam_logic::tile::Tile;
use bincode::Options;
use common::{consts::BINCODE_OPTIONS, map::Map, user::UserId};
use rusqlite::params;

use super::{types::DbUuid, Database};

#[derive(Debug)]
pub struct Results {
    pub user_id: UserId,
    pub ip_address: Ipv4Addr,
    pub timestamp: u64,

    pub level_id: DbUuid,
    pub solution: Map<Tile>,
    pub cost: u32,
    pub latency: u32,
}

impl Database {
    pub fn insert_result(&self, result: Results) -> Result<()> {
        let solution = BINCODE_OPTIONS.serialize(&result.solution)?;

        self.lock().execute(
            include_str!("sql/upsert_results.sql"),
            params![
                result.user_id.type_id(),
                result.user_id.inner() as i64,
                result.ip_address.to_bits(),
                result.timestamp,
                result.level_id,
                solution,
                result.cost,
                result.latency,
            ],
        )?;

        Ok(())
    }
}
