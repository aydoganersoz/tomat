use crate::storage;
use chrono::Utc;
use serde_json;
use std::fs::File;

pub fn show() {
  let stats = storage::show_db().expect("failed to show statistics");
  println!("-----------------------------------------------------------------");
  println!("| Day\t\t| Pomodoros\t| Short breaks\t| Long breaks   |");
  println!("-----------------------------------------------------------------");
  if stats.len() > 0 {
    for row in stats {
      println!(
        "| {}\t| {}\t\t| {}\t\t| {}             |",
        row.date, row.completed_pomodoro, row.completed_short_break, row.completed_long_break
      );
    }
    println!("-----------------------------------------------------------------");
  }
}

pub fn reset() {
  storage::reset_db().expect("failed to reset statistics");
}

pub fn export() {
  let stats = storage::show_db().expect("failed to show statistics");
  let timestamp = Utc::now().timestamp();
  let mut filename = String::from("out/stats_");
  filename.push_str(&timestamp.to_string());
  filename.push_str(".json");

  let file = File::create(filename).expect("failed to create file");
  serde_json::to_writer_pretty(&file, &stats).expect("failed to write to file");
}
