use crate::storage;

pub fn show() {
  let stats = storage::show_db().expect("failed to show statistics");
  println!("-----------------------------------------------------------------");
  println!("| Day\t\t| Pomodoros\t| Short breaks\t| Long breaks   |");
  println!("-----------------------------------------------------------------");
  for row in stats {
    println!(
      "| {}\t| {}\t\t| {}\t\t| {}             |",
      row.date, row.completed_pomodoro, row.completed_short_break, row.completed_long_break
    );
  }
  println!("-----------------------------------------------------------------");
}
