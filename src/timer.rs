use crate::bar;
use chrono;
use std::sync::mpsc::channel;
use timer;

pub fn start_timer(duration: u32) {
  let bar = bar::Bar::new(duration);

  for _ in 0..duration {
    bar.increase();

    if cfg!(debug_assertions) {
      set_timer(false); // debug
    } else {
      set_timer(true); // release
    }
  }
}

fn set_timer(ndbg: bool) {
  let tmr = timer::Timer::new();
  let (tx, rx) = channel();

  match ndbg {
    false => {
      tmr.schedule_with_delay(chrono::Duration::seconds(1), move || {
        let _ = tx.send(());
      });
    }

    true => {
      tmr.schedule_with_delay(chrono::Duration::minutes(1), move || {
        let _ = tx.send(());
      });
    }
  }

  let _ = rx.recv();
}
