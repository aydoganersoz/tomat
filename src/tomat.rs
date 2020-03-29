use crate::args;
use crate::timer;
use chrono::Local;

pub fn run_tomat(durations: args::Durations) {
  let sessions = [
    (durations.pomodoro, "pomodoro"),
    (durations.short_break, "short break"),
    (durations.pomodoro, "pomodoro"),
    (durations.short_break, "short break"),
    (durations.pomodoro, "pomodoro"),
    (durations.long_break, "long break"),
  ];

  loop {
    for session in sessions.iter() {
      let now = Local::now().format("%H:%M:%S");
      println!(
        "{} {} started for {} minutes",
        now,
        (*session).1,
        (*session).0
      );
      timer::start_timer((*session).0);
    }
  }
}
