use clap::{App, AppSettings, Arg};

const VERSION: &str = "0.1.0";

const AUTHOR: &str = "Aydogan Ersoz <aydoganersoz@protonmail.com>";

const APP_TOMAT: &str = "tomat";
const APP_START: &str = "start";
const APP_STAT: &str = "stat";
const APP_RESET: &str = "reset";
const APP_EXPORT: &str = "export";
const APP_SHOW: &str = "show";

const ABOUT_TOMAT: &str = "Minimal pomodoro timer";
const ABOUT_START: &str = "Starts pomodoro timer";
const ABOUT_STAT: &str = "Handles statistics";
const ABOUT_RESET: &str = "Resets statistics";
const ABOUT_EXPORT: &str = "Exports statistics";
const ABOUT_SHOW: &str = "Shows statistics";

const NAME_LONG_BREAK: &str = "long break";
const NAME_SHORT_BREAK: &str = "short break";
const NAME_POMODORO: &str = "pomodoro";
const NAME_JSON: &str = "json";
const NAME_CSV: &str = "csv";

const LONG_LONG_BREAK: &str = "long-break";
const LONG_SHORT_BREAK: &str = "short-break";
const LONG_POMODORO: &str = "pomodoro";
const LONG_JSON: &str = "json";
const LONG_CSV: &str = "csv";

const SHORT_LONG_BREAK: &str = "l";
const SHORT_SHORT_BREAK: &str = "s";
const SHORT_POMODORO: &str = "p";
const SHORT_JSON: &str = "j";
const SHORT_CSV: &str = "c";

const VALUE_NAME_LONG_BREAK: &str = "DURATION";
const VALUE_NAME_SHORT_BREAK: &str = "DURATION";
const VALUE_NAME_POMODORO: &str = "DURATION";
const VALUE_NAME_JSON: &str = "FILENAME";
const VALUE_NAME_EXPORT: &str = "FILENAME";

const DEFAULT_VALUE_LONG_BREAK: &str = "15";
const DEFAULT_VALUE_SHORT_BREAK: &str = "5";
const DEFAULT_VALUE_POMODORO: &str = "25";

const HELP_LONG_BREAK: &str = "Sets long break duration";
const HELP_SHORT_BREAK: &str = "Sets short break duration";
const HELP_POMODORO: &str = "Sets pomodoro duration";
const HELP_JSON: &str = "Exports statistics to a given output file in JSON format";
const HELP_CSV: &str = "Exports statistics to a given output file in CSV format";

const TAKES_VALUE_LONG_BREAK: bool = true;
const TAKES_VALUE_SHORT_BREAK: bool = true;
const TAKES_VALUE_POMODORO: bool = true;
const TAKES_VALUE_JSON: bool = true;
const TAKES_VALUE_CSV: bool = true;

pub struct Duration {
    pub long_break: u32,
    pub short_break: u32,
    pub pomodoro: u32,
}

pub struct StartParam {
    pub duration: Duration,
}

pub enum ExportType {
    JSON,
    CSV,
}

pub struct ExportParam {
    pub export_type: ExportType,
    pub filename: String,
}

pub enum Command {
    Start(StartParam),
    Export(ExportParam),
    Reset(),
    Show(),
    Undefined(),
}

pub fn parse_args() -> Command {
    match get_matches().subcommand() {
        (APP_START, Some(start)) => Command::Start(parse_start(start)),
        (APP_STAT, Some(stat)) => match stat.subcommand() {
            (APP_EXPORT, Some(export)) => Command::Export(parse_export(export)),
            (APP_RESET, Some(_reset)) => Command::Reset(),
            (APP_SHOW, Some(_show)) => Command::Show(),
            (&_, _) => Command::Undefined(), // handled by `ArgRequiredElseHelp`
        },
        (&_, _) => Command::Undefined(), // handled by `ArgRequiredElseHelp`
    }
}

fn parse_export<'a>(export: &'a clap::ArgMatches) -> ExportParam {
    // argument is required so `else` case is obvious
    if export.is_present(NAME_JSON) {
        // JSON
        ExportParam {
            export_type: ExportType::JSON,
            filename: export.value_of(NAME_JSON).unwrap().to_string(),
        }
    } else {
        // CSV
        ExportParam {
            export_type: ExportType::CSV,
            filename: export.value_of(NAME_CSV).unwrap().to_string(),
        }
    }
}

fn parse_start<'a>(start: &clap::ArgMatches) -> StartParam {
    StartParam {
        // arguments have default values so calling `is_present` is not crucial
        duration: Duration {
            long_break: start
                .value_of(NAME_LONG_BREAK)
                .unwrap()
                .parse::<u32>()
                .unwrap(),
            short_break: start
                .value_of(NAME_SHORT_BREAK)
                .unwrap()
                .parse::<u32>()
                .unwrap(),
            pomodoro: start
                .value_of(NAME_POMODORO)
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        },
    }
}

fn get_matches<'a>() -> clap::ArgMatches<'a> {
    App::new(APP_TOMAT)
        .version(VERSION)
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::ArgRequiredElseHelp)
        .author(AUTHOR)
        .about(ABOUT_TOMAT)
        // start subcommand
        .subcommand(
            App::new(APP_START)
                .about(ABOUT_START)
                .setting(AppSettings::DisableVersion)
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
                // pomodoro option
                .arg(
                    Arg::with_name(NAME_POMODORO)
                        .short(SHORT_POMODORO)
                        .long(LONG_POMODORO)
                        .value_name(VALUE_NAME_POMODORO)
                        .help(HELP_POMODORO)
                        .default_value(DEFAULT_VALUE_POMODORO)
                        .takes_value(TAKES_VALUE_POMODORO),
                ),
        )
        // stat subcommand
        .subcommand(
            App::new(APP_STAT)
                .about(ABOUT_STAT)
                .setting(AppSettings::ArgRequiredElseHelp)
                .setting(AppSettings::DisableVersion)
                // reset subcommand
                .subcommand(
                    App::new(APP_RESET)
                        .about(ABOUT_RESET)
                        .setting(AppSettings::DisableVersion),
                )
                // export subcommand
                .subcommand(
                    App::new(APP_EXPORT)
                        .about(ABOUT_EXPORT)
                        .setting(AppSettings::ArgRequiredElseHelp)
                        .setting(AppSettings::DisableVersion)
                        // json option
                        .arg(
                            Arg::with_name(NAME_JSON)
                                .short(SHORT_JSON)
                                .long(LONG_JSON)
                                .value_name(VALUE_NAME_JSON)
                                .help(HELP_JSON)
                                .takes_value(TAKES_VALUE_JSON),
                        )
                        // csv option
                        .arg(
                            Arg::with_name(NAME_CSV)
                                .short(SHORT_CSV)
                                .long(LONG_CSV)
                                .value_name(VALUE_NAME_EXPORT)
                                .help(HELP_CSV)
                                .takes_value(TAKES_VALUE_CSV),
                        ),
                )
                // show subcommand
                .subcommand(
                    App::new(APP_SHOW)
                        .about(ABOUT_SHOW)
                        .setting(AppSettings::DisableVersion),
                ),
        )
        .get_matches()
}
