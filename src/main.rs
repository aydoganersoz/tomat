mod commands;
mod system;
mod types;
#[macro_use]
extern crate prettytable;

use system::{args, string, tools, util};

fn main() {
    util::register_sigint().expect(string::ERR_REGISTER_SIGINT);
    let args = args::parse_args();
    tools::create_directories().expect(string::ERR_CREATE_DIRECTORY);
    tools::create_db().expect(string::ERR_CREATE_DATABASE);
    match args {
        args::Command::Start(x) => commands::start::run_tomat(x),
        args::Command::Export() => commands::stat::export(),
        args::Command::Reset() => commands::stat::reset(),
        args::Command::Show() => commands::stat::show(),
        // handled by `ArgRequiredElseHelp` in args module
        args::Command::Undefined() => util::print_terminal(string::ERR_UNDEFINED_COMMAND),
    }
}
