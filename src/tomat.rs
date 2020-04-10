use crate::args;
use crate::storage;
use crate::timer;
use crate::util;
use chrono::Local;

#[derive(Copy, Clone)]
enum SessionKind {
    Pomodoro,
    ShortBreak,
    LongBreak,
}

#[derive(Copy, Clone)]
struct Session {
    duration: u32,
    name: &'static str,
    kind: SessionKind,
}

struct Stats {
    completed_pomodoro: u32,
    completed_short_break: u32,
    completed_long_break: u32,
}

impl Stats {
    pub fn increase(&mut self, kind: SessionKind) {
        match kind {
            SessionKind::Pomodoro => {
                self.completed_pomodoro = self.completed_pomodoro + 1;
            }
            SessionKind::ShortBreak => {
                self.completed_short_break = self.completed_short_break + 1;
            }
            SessionKind::LongBreak => {
                self.completed_long_break = self.completed_long_break + 1;
            }
        }
    }
    pub fn get(&mut self, kind: SessionKind) -> u32 {
        match kind {
            SessionKind::Pomodoro => self.completed_pomodoro,
            SessionKind::ShortBreak => self.completed_short_break,
            SessionKind::LongBreak => self.completed_long_break,
        }
    }
}

pub fn run_tomat(durations: args::Durations) {
    let pomodoro = Session {
        duration: durations.pomodoro,
        name: "pomodoro",
        kind: SessionKind::Pomodoro,
    };
    let short_break = Session {
        duration: durations.short_break,
        name: "short break",
        kind: SessionKind::ShortBreak,
    };
    let long_break = Session {
        duration: durations.long_break,
        name: "long break",
        kind: SessionKind::LongBreak,
    };

    let mut stats = Stats {
        completed_pomodoro: 0,
        completed_short_break: 0,
        completed_long_break: 0,
    };

    let mut sessions = [
        pomodoro,
        short_break,
        pomodoro,
        short_break,
        pomodoro,
        long_break,
    ];

    loop {
        for session in sessions.iter_mut() {
            let now = Local::now().format("%H:%M:%S");
            println!(
                "{} {} started for {} minutes (completed {}s: {})",
                now,
                session.name,
                session.duration,
                session.name,
                stats.get(session.kind)
            );
            timer::start_timer(session.duration);
            stats.increase(session.kind);
            let status = storage::insert_db(session.kind as u8);
            match status {
                Ok(()) => (),
                Err(err) => {
                    println!("insert database failed {}", err);
                    util::exit_program();
                }
            };
        }
    }
}
