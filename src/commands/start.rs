use crate::system::{args, string, tools};
use crate::types::*;

pub fn run_tomat(start_param: args::StartParam) {
    let pomodoro = Session::new(start_param.duration.pomodoro, SessionKind::Pomodoro);
    let short_break = Session::new(start_param.duration.short_break, SessionKind::ShortBreak);
    let long_break = Session::new(start_param.duration.long_break, SessionKind::LongBreak);
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
            tools::new_session_confirmation(session.kind);
            tools::wait_timer(session.duration);
            tools::insert_db(session.kind).expect(string::ERR_INSERT_DATABASE);
            tools::play_bip();
        }
    }
}
