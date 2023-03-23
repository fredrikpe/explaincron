use clap::{App, Arg};

use crate::cron;

const ABOUT: &str = "\nExplain cron schedules in human readable form.
cron syntax:
    *	any value
    ,	value list separator
    -	range of values
    /	step values";

const USAGE: &str = "explaincron [FLAGS] [ARGS]
    explaincron '3-5 1/4 * FEB SUN'";

fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse::<usize>()
        .map_err(|_| format!("'{s}' is not a positive number"))
}

pub fn app() -> App<'static, 'static> {
    return App::new("explaincron")
        .version("0.1")
        .usage(USAGE)
        .author("Fredrik Pe <fredrikpei@gmail.com>")
        .about(ABOUT)
        .arg(
            Arg::with_name("SCHEDULE")
                .help("The cron schedule to explain.")
                .required_unless("random")
                .index(1)
                .validator(|input| cron::Schedule::from_str(&input).map(|_| ())),
        )
        .arg(
            Arg::with_name("random")
                .short("r")
                .long("random")
                .multiple(false)
                .help("Output a random cron schedule"),
        )
        .arg(
            Arg::with_name("num-next-occurrence")
                .short("n")
                .default_value("1")
                .long("next-occurrence")
                .multiple(false)
                .validator(|input| parse_usize(&input).map(|_| ()))
                .help("Show <n> next occurrences"),
        );
}
