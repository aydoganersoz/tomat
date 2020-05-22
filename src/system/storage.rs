use crate::system::tools;
use crate::types::Stats;
use rusqlite::{Connection, Result, NO_PARAMS};

pub fn create_db() -> Result<()> {
  let conn = Connection::open(tools::get_db_path())?;
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

/// Inserts a session into database
///
/// Function takes a parameter of type `u8` for the moment. In the future
/// version, it will be of type `SessionKind` to avoid ambiguity.
///
/// It is necessary to have an already created database to call this file.
/// Otherwise it won't return `Ok`.
///
/// ```
/// extern crate tomat;
///
/// use tomat::system::storage;
/// use tomat::types;
///
/// let result = storage::insert_db(types::SessionKind::Pomodoro as u8);
/// assert_eq!(result, Ok(()));
/// ```
pub fn insert_db(session_kind: u8) -> Result<()> {
  let conn = Connection::open(tools::get_db_path())?;
  conn.execute(
    "INSERT INTO stats (session_kind) values (?1)",
    &[session_kind],
  )?;
  Ok(())
}

pub fn show_db() -> Result<Vec<Stats>> {
  let conn = Connection::open(tools::get_db_path())?;
  let mut stmt = conn.prepare(
    "SELECT
      SUM(session_kind = 0) completed_pomodoro,
      SUM(session_kind = 1) completed_short_break,
      SUM(session_kind = 2) completed_long_break,
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
  let conn = Connection::open(tools::get_db_path())?;
  conn.execute("DELETE FROM stats", NO_PARAMS)?;
  Ok(())
}
