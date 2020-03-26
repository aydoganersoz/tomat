use crate::args;
use crate::timer;

pub fn run_tomat(durations: args::Durations) {
  println!("tomat is running {}", durations.pomodoro);

  timer::start_timer(durations.pomodoro);
  println!("timer run terminated");
}
