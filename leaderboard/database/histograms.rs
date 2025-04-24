use anyhow::Result;
use rusqlite::{params, Row};
use uuid::Uuid;

use leaderboard::api::results::Histogram;

use super::{types::DbUuid, Database, DbResult, SimplifyDbResult};

impl Database {
    /// -> (Cost, Latency)
    pub fn get_histogram(&self, level: Uuid) -> Result<(Histogram, Histogram)> {
        fn parse_histogram(row: &Row<'_>, start: usize) -> rusqlite::Result<Histogram> {
            Ok(Histogram {
                bins: [
                    row.get(start + 1)?,
                    row.get(start + 2)?,
                    row.get(start + 3)?,
                    row.get(start + 4)?,
                    row.get(start + 5)?,
                    row.get(start + 6)?,
                    row.get(start + 7)?,
                    row.get(start + 8)?,
                    row.get(start + 9)?,
                    row.get(start + 10)?,
                    row.get(start + 11)?,
                    row.get(start + 12)?,
                ],
                max: row.get(start)?,
            })
        }

        let data = self
            .lock()
            .query_row(
                "SELECT * FROM histograms WHERE level = ?",
                [level.to_string()],
                |row| Ok((parse_histogram(row, 1)?, parse_histogram(row, 14)?)),
            )
            .simplify()?;

        Ok(match data {
            DbResult::Ok(data) => data,
            DbResult::NotFound => (Histogram::default(), Histogram::default()),
        })
    }

    pub fn update_histograms(&self, level: Uuid) -> Result<()> {
        let db = self.lock();

        let mut stmt = db.prepare("SELECT cost, latency FROM results WHERE level = ?")?;
        let results = stmt.query_map(params![DbUuid::from(level)], |row| {
            Ok((row.get::<_, u32>(0)?, row.get::<_, u32>(1)?))
        })?;

        let (cost, latency) = results
            .filter_map(|x| x.ok())
            .unzip::<_, _, Vec<_>, Vec<_>>();

        let (cost, latency) = (Histogram::new(&cost), Histogram::new(&latency));

        // its fine...
        db.execute(
            include_str!("sql/upsert_histograms.sql"),
            params![
                DbUuid::from(level),
                cost.max,
                cost.bins[0],
                cost.bins[1],
                cost.bins[2],
                cost.bins[3],
                cost.bins[4],
                cost.bins[5],
                cost.bins[6],
                cost.bins[7],
                cost.bins[8],
                cost.bins[9],
                cost.bins[10],
                cost.bins[11],
                latency.max,
                latency.bins[0],
                latency.bins[1],
                latency.bins[2],
                latency.bins[3],
                latency.bins[4],
                latency.bins[5],
                latency.bins[6],
                latency.bins[7],
                latency.bins[8],
                latency.bins[9],
                latency.bins[10],
                latency.bins[11],
            ],
        )?;

        Ok(())
    }
}
