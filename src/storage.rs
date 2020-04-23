use rusqlite::{Connection, Result, NO_PARAMS};
use serde_derive::{Deserialize, Serialize};

const DB_PATH: &str = "db/stats.db3";

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
  pub completed_pomodoro: u32,
  pub completed_short_break: u32,
  pub completed_long_break: u32,
  pub date: String,
}

pub fn create_db() -> Result<()> {
  let conn = Connection::open(DB_PATH)?;

  conn.execute(
    "CREATE TABLE IF NOT EXISTS stats (
      id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
      session_kind INTEGER NOT NULL,
      ts TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    )",
    NO_PARAMS,
  )?;

  Ok(())
}

pub fn insert_db(session_kind: u8) -> Result<()> {
  let conn = Connection::open(DB_PATH)?;

  conn.execute(
    "INSERT INTO stats (session_kind) values (?1)",
    &[session_kind],
  )?;

  Ok(())
}

pub fn show_db() -> Result<Vec<Stats>> {
  let conn = Connection::open(DB_PATH)?;

  let mut stmt = conn.prepare(
    "SELECT
      SUM(session_kind = 0) session_kind_0,
      SUM(session_kind = 1) session_kind_1,
      SUM(session_kind = 2) session_kind_2,
      DATE(ts) ts_day
    FROM stats
    GROUP BY DATE(ts)
    ORDER BY ts_day DESC",
  )?;

  let mut stat = Vec::<Stats>::new();
  let mut rows = stmt.query(NO_PARAMS)?;
  while let Some(row) = rows.next()? {
    stat.push(Stats {
      completed_pomodoro: row.get(0)?,
      completed_short_break: row.get(1)?,
      completed_long_break: row.get(2)?,
      date: row.get(3)?,
    });
  }

  Ok(stat)
}

pub fn reset_db() -> Result<()> {
  let conn = Connection::open(DB_PATH)?;

  conn.execute("DELETE FROM stats", NO_PARAMS)?;

  Ok(())
}
