mod args;
mod bar;
mod storage;
mod timer;
mod tomat;
mod util;

fn main() {
    util::register_sigint();
    let durations = args::parse_args();

    util::create_dir().expect("create directory failed");
    storage::create_db().expect("create database failed");

    tomat::run_tomat(durations);
}
