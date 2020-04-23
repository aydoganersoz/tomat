mod args;
mod bar;
mod commands;
mod storage;
mod timer;
mod tomat;
mod util;

fn main() {
    util::register_sigint();

    let args = args::parse_args();

    util::create_dir().expect("create directory failed");
    storage::create_db().expect("create database failed");

    match args {
        args::Command::Start(x) => {
            tomat::run_tomat(x);
        }
        args::Command::Export() => {
            commands::stat::export();
        }
        args::Command::Reset() => {
            commands::stat::reset();
        }
        args::Command::Show() => {
            commands::stat::show();
        }
        args::Command::Undefined() => {
            println!("Command::Undefined()");
        }
    }
}
