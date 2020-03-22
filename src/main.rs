mod args;

fn main() {
    let args = args::parse_args();

    println!("{} {} {}", args.long_break, args.short_break, args.pomodoro);
}
