use serde_derive::{Deserialize, Serialize};

#[derive(Copy, Clone)]
pub enum SessionKind {
  Pomodoro,
  ShortBreak,
  LongBreak,
}

#[derive(Copy, Clone)]
pub struct Session {
  pub duration: u32,
  pub kind: SessionKind,
}

impl Session {
  pub fn new(session_duration: u32, session_kind: SessionKind) -> Session {
    Session {
      duration: session_duration,
      kind: session_kind,
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
  pub completed_pomodoro: u32,
  pub completed_short_break: u32,
  pub completed_long_break: u32,
  pub date: String,
}
