use clap::{App, Arg};

use crate::cron;

const ABOUT: &str = "\nExplain cron schedules in human readable form.
cron syntax:
    *	any value
    ,	value list separator
    -	range of values
    /	step values";

const USAGE: &str = "explaincron [FLAGS] [ARGS]
    explaincron 3-5 1/4 \\* \\* \\*
    explaincron '* * * FEB SUN'";

pub fn app() -> App<'static, 'static> {
    return App::new("explaincron")
        .version("0.1")
        .usage(USAGE)
        .author("Fredrik Pe <fredrikpei@gmail.com>")
        .about(ABOUT)
        .arg(
            Arg::with_name("MINUTE (or complete schedule)")
                .help("Allowed values 0-59. Or a complete schedule.")
                .required_unless("random")
                .index(1)
                .validator(|input| {
                    if input.contains(char::is_whitespace) {
                        cron::Schedule::from_str(&input).map(|_| ())
                    } else {
                        cron::Minute::from_str(&input).map(|_| ())
                    }
                }),
        )
        .arg(
            Arg::with_name("HOUR")
                .help("Allowed values 0-23")
                .default_value("*")
                .required(false)
                .index(2)
                .validator(|input| cron::Hour::from_str(&input).map(|_| ())),
        )
        .arg(
            Arg::with_name("DAY (of month)")
                .help("Allowed values 1-31")
                .default_value("*")
                .required(false)
                .index(3)
                .validator(|input| cron::DayOfMonth::from_str(&input).map(|_| ())),
        )
        .arg(
            Arg::with_name("MONTH")
                .help("Allowed values 1-12 or JAN-DEC")
                .default_value("*")
                .required(false)
                .index(4)
                .validator(|input| cron::Month::from_str(&input).map(|_| ())),
        )
        .arg(
            Arg::with_name("DAY (of week)")
                .help("Allowed values 0-6 or SUN-SAT")
                .default_value("*")
                .required(false)
                .index(5)
                .validator(|input| cron::DayOfWeek::from_str(&input).map(|_| ())),
        )
        .arg(
            Arg::with_name("random")
                .short("r")
                .long("random")
                .multiple(false)
                .help("Output a random cron schedule"),
        );
}
