use std::{env::args, fs};

use anyhow::{Context, Result};
use rusqlite::Connection;

use crate::{config::Config, database::Db};

pub struct App {
    pub config: Config,
    pub db: Db,
}

impl App {
    pub fn new() -> Result<Self> {
        let config_path = args().nth(1).unwrap_or_else(|| "config.toml".to_string());
        let raw_config = fs::read_to_string(config_path).context("While reading config")?;
        let config = toml::from_str::<Config>(&raw_config).context("While parsing config")?;

        fs::create_dir_all(config.server.database_path.parent().unwrap())?;
        let connection = Connection::open(&config.server.database_path)
            .context("While opening connection to Database")?;
        let db = Db::new(connection);
        db.init().context("While initializing Database")?;

        Ok(Self { config, db })
    }
}
