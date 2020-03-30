mod args;
mod bar;
mod timer;
mod tomat;

fn main() {
    let durations = args::parse_args();

    tomat::run_tomat(durations);
}
