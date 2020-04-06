use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};

const DB_PATH: &str = "db/stats.db3";

pub fn create_db() -> Result<()> {
  let conn = Connection::open(DB_PATH)?;

  conn.execute(
    "CREATE TABLE IF NOT EXISTS stats (
      id INTEGER PRIMARY KEY,
      session_kind INTEGER,
      date DATE default CURRENT_DATE
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
