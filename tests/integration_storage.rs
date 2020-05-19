extern crate tomat;

use tomat::system::{storage, tools};
use tomat::types;

#[test]
fn test_insert_db() {
  let result = storage::create_db();
  assert_eq!(result, Ok(()));
  let result = storage::insert_db(types::SessionKind::Pomodoro as u8);
  assert_eq!(result, Ok(()));
}

#[test]
fn test_export_json() {
  tools::create_directories().unwrap();
  let mut records = Vec::<types::Stats>::new();
  records.push(types::Stats {
    completed_pomodoro: 1,
    completed_short_break: 1,
    completed_long_break: 1,
    date: String::from("2020-04-24"),
  });
  tools::export_json(records);
}

#[test]
fn test_print_stat_table() {
  tools::create_directories().unwrap();
  let mut records = Vec::<types::Stats>::new();
  records.push(types::Stats {
    completed_pomodoro: 1,
    completed_short_break: 1,
    completed_long_break: 1,
    date: String::from("2020-04-24"),
  });
  tools::print_stat_table(records);
}
