mod args;
mod bar;
mod storage;
mod timer;
mod tomat;
mod util;

fn main() {
    let durations = args::parse_args();

    let status = util::create_dir();
    match status {
        Ok(()) => (),
        Err(err) => {
            println!("create directory failed {}", err);
            util::exit_program();
        }
    }

    let status = storage::create_db();
    match status {
        Ok(()) => (),
        Err(err) => {
            println!("create database failed {}", err);
            util::exit_program();
        }
    };

    tomat::run_tomat(durations);
}
