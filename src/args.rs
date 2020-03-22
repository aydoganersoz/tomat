use clap::{App, Arg};

const NAME_LONG_BREAK: &str = "long break";
const NAME_SHORT_BREAK: &str = "short break";
const NAME_POMODORO: &str = "pomodoro";

const LONG_LONG_BREAK: &str = "long-break";
const LONG_SHORT_BREAK: &str = "short-break";
const LONG_POMODORO: &str = "pomodoro";

const SHORT_LONG_BREAK: &str = "l";
const SHORT_SHORT_BREAK: &str = "s";
const SHORT_POMODORO: &str = "p";

const VALUE_NAME_LONG_BREAK: &str = "DURATION";
const VALUE_NAME_SHORT_BREAK: &str = "DURATION";
const VALUE_NAME_POMODORO: &str = "DURATION";

const DEFAULT_VALUE_LONG_BREAK: &str = "15";
const DEFAULT_VALUE_SHORT_BREAK: &str = "5";
const DEFAULT_VALUE_POMODORO: &str = "25";

const HELP_LONG_BREAK: &str = "Sets long break duration";
const HELP_SHORT_BREAK: &str = "Sets short break duration";
const HELP_POMODORO: &str = "Sets pomodoro duration";

const TAKES_VALUE_LONG_BREAK: bool = true;
const TAKES_VALUE_SHORT_BREAK: bool = true;
const TAKES_VALUE_POMODORO: bool = true;

pub struct Durations {
  pub long_break: u32,
  pub short_break: u32,
  pub pomodoro: u32,
}

pub fn parse_args() -> Durations {
  let matches = App::new("tomat")
    .version("0.1.0")
    .author("Aydogan Ersoz <aydoganersoz@protonmail.com>")
    .about("Minimal pomodoro timer")
    // long break option
    .arg(
      Arg::with_name(NAME_LONG_BREAK)
        .short(SHORT_LONG_BREAK)
        .long(LONG_LONG_BREAK)
        .value_name(VALUE_NAME_LONG_BREAK)
        .help(HELP_LONG_BREAK)
        .default_value(DEFAULT_VALUE_LONG_BREAK)
        .takes_value(TAKES_VALUE_LONG_BREAK),
    )
    // short break option
    .arg(
      Arg::with_name(NAME_SHORT_BREAK)
        .short(SHORT_SHORT_BREAK)
        .long(LONG_SHORT_BREAK)
        .value_name(VALUE_NAME_SHORT_BREAK)
        .help(HELP_SHORT_BREAK)
        .default_value(DEFAULT_VALUE_SHORT_BREAK)
        .takes_value(TAKES_VALUE_SHORT_BREAK),
    )
    // pomodoro duration option flag
    .arg(
      Arg::with_name(NAME_POMODORO)
        .short(SHORT_POMODORO)
        .long(LONG_POMODORO)
        .value_name(VALUE_NAME_POMODORO)
        .help(HELP_POMODORO)
        .default_value(DEFAULT_VALUE_POMODORO)
        .takes_value(TAKES_VALUE_POMODORO),
    )
    .get_matches();

  let durations = Durations {
    long_break: matches
      .value_of(NAME_LONG_BREAK)
      .unwrap()
      .parse::<u32>()
      .unwrap(),
    short_break: matches
      .value_of(NAME_SHORT_BREAK)
      .unwrap()
      .parse::<u32>()
      .unwrap(),
    pomodoro: matches
      .value_of(NAME_POMODORO)
      .unwrap()
      .parse::<u32>()
      .unwrap(),
  };

  durations
}
