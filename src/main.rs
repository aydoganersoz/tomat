mod args;
mod timer;

fn main() {
    let args = args::parse_args();

    println!("{} {} {}", args.long_break, args.short_break, args.pomodoro);

    timer::start_timer(args.pomodoro);
    println!("program terminates");
}
