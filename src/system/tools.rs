use crate::{
  system::storage,
  system::string,
  system::util,
  types::{SessionKind, Stats},
};
use chrono;
use indicatif::ProgressBar;
use prettytable::{format, Table};
use serde_json;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use timer::Timer;

const EXPORT_FILENAME_EXTENSION: &str = "json";
const EXPORT_FILENAME_SEPARATOR: &str = "_";
const EXPORT_FILENAME_PREFIX: &str = "stats";

const RESOURCE_DIRECTORY: &str = "rsc";
const OUTPUT_DIRECTORY: &str = "out";
const DATABASE_DIRECTORY: &str = "db";

const BIP_FILENAME: &str = "A-Tone-His_Self-1266414414";
const BIP_FILENAME_EXTENSION: &str = "wav";

const DATABASE_FILENAME: &str = "stats";
const DATABASE_FILENAME_EXTENSION: &str = "db3";

pub fn create_db() -> rusqlite::Result<()> {
  storage::create_db()?;
  Ok(())
}

pub fn insert_db(session_kind: SessionKind) -> rusqlite::Result<()> {
  storage::insert_db(session_kind as u8)?;
  Ok(())
}

pub fn new_session_confirmation(session_kind: SessionKind) {
  let confirmation_text = match session_kind {
    SessionKind::Pomodoro => string::DLG_NEW_POMODORO_SESSION_CONFIRMATION,
    SessionKind::ShortBreak => string::DLG_NEW_SHORT_BREAK_SESSION_CONFIRMATION,
    SessionKind::LongBreak => string::DLG_NEW_LONG_BREAK_SESSION_CONFIRMATION,
  };
  let result = util::yes_no_confirmation(confirmation_text);
  match result {
    Ok(res) => {
      if res == false {
        util::exit_program();
      }
    }
    Err(_e) => (),
  }
}

pub fn get_db_path() -> PathBuf {
  util::construct_path(
    vec![DATABASE_DIRECTORY, DATABASE_FILENAME],
    DATABASE_FILENAME_EXTENSION,
  )
}

pub fn create_directories() -> std::io::Result<()> {
  util::create_directory(DATABASE_DIRECTORY)?;
  util::create_directory(OUTPUT_DIRECTORY)?;
  Ok(())
}

pub fn play_bip() {
  let path = util::construct_path(
    vec![RESOURCE_DIRECTORY, BIP_FILENAME],
    BIP_FILENAME_EXTENSION,
  );
  util::play_sound(path);
}

pub fn print_stat_table(records: Vec<Stats>) {
  if records.len() > 0 {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row![
      string::TERM_DAY,
      string::TERM_POMODORO,
      string::TERM_SHORT_BREAK,
      string::TERM_LONG_BREAK
    ]);
    for record in records {
      table.add_row(row![
        record.date,
        record.completed_pomodoro,
        record.completed_short_break,
        record.completed_long_break
      ]);
    }
    table.printstd();
  } else {
    util::print_terminal(string::MSG_NO_STAT_FOUND);
  }
}

pub fn export_json(records: Vec<Stats>) {
  let timestamp = util::get_timestamp().to_string();
  let filename = util::construct_filename(vec![
    EXPORT_FILENAME_PREFIX,
    EXPORT_FILENAME_SEPARATOR,
    &timestamp,
  ]);
  let path = util::construct_path(vec![OUTPUT_DIRECTORY, &filename], EXPORT_FILENAME_EXTENSION);
  let file = util::create_file(path).expect(string::ERR_CREATE_FILE);
  serde_json::to_writer_pretty(&file, &records).expect(string::ERR_WRITE_FILE);
}

pub fn wait_timer(minutes: u32) {
  let bar = ProgressBar::new(minutes as u64);
  for _ in 0..minutes {
    bar.inc(1);
    let tmr = Timer::new();
    let (tx, rx) = channel();
    if cfg!(debug_assertions) {
      // for easy debugging (seconds instead of minutes)
      tmr.schedule_with_delay(chrono::Duration::seconds(1), move || {
        let _ = tx.send(());
      });
    } else {
      tmr.schedule_with_delay(chrono::Duration::minutes(1), move || {
        let _ = tx.send(());
      });
    }
    let _ = rx.recv();
  }
}
