use crate::system::{storage, string, tools, util};

pub fn show() {
  let stats = storage::show_db().expect(string::ERR_SHOW_STATISTICS);
  tools::print_stat_table(stats);
}

pub fn reset() {
  storage::reset_db().expect(string::ERR_RESET_STATISTICS);
  util::print_terminal(string::MSG_STAT_RESET);
}

pub fn export() {
  let stats = storage::show_db().expect(string::ERR_SHOW_STATISTICS);
  tools::export_json(stats);
  util::print_terminal(string::MSG_STAT_EXPORTED);
}
