use chrono;
use std::sync::mpsc::channel;
use timer;

#[cfg(debug_assertions)]
pub fn start_timer(duration: u32) {
  let tmr = timer::Timer::new();
  let (tx, rx) = channel();

  tmr.schedule_with_delay(chrono::Duration::seconds(duration as i64), move || {
    let _ = tx.send(());
  });
  let _ = rx.recv();
}

#[cfg(not(debug_assertions))]
pub fn start_timer(duration: u32) {
  let tmr = timer::Timer::new();
  let (tx, rx) = channel();

  tmr.schedule_with_delay(chrono::Duration::minutes(duration as i64), move || {
    let _ = tx.send(());
  });
  let _ = rx.recv();
}
