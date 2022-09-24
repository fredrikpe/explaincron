use clap::{App, Arg};

use crate::parser;

fn input_validator(
    input: String,
    elem_parser: fn(&str) -> Result<i32, String>,
) -> Result<(), String> {
    return if input.contains(',') {
        parser::list(input.split(','), elem_parser).map(|_| ())
    } else if input.contains('-') {
        parser::range(input.split('-'), elem_parser).map(|_| ())
    } else if input.contains('/') {
        parser::step(input.split('/'), elem_parser).map(|_| ())
    } else if input == "*" {
        Ok(())
    } else {
        elem_parser(&input).map(|_| ())
    };
}

fn schedule_validator(schedule_input: String) -> Result<(), String> {
    let parts = schedule_input.split_whitespace().collect::<Vec<&str>>();
    if parts.len() != 5 {
        return Err(format!("schedule does not contain 5 parts"));
    }
    input_validator(parts[0].to_string(), parser::minute)?;
    input_validator(parts[1].to_string(), parser::hour)?;
    input_validator(parts[2].to_string(), parser::day_of_month)?;
    input_validator(parts[3].to_string(), parser::month)?;
    input_validator(parts[4].to_string(), parser::day_of_week)?;

    Ok(())
}

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
                        schedule_validator(input)
                    } else {
                        input_validator(input, parser::minute)
                    }
                }),
        )
        .arg(
            Arg::with_name("HOUR")
                .help("Allowed values 0-23")
                .default_value("*")
                .required(false)
                .index(2)
                .validator(|input| input_validator(input, parser::hour)),
        )
        .arg(
            Arg::with_name("DAY (of month)")
                .help("Allowed values 1-31")
                .default_value("*")
                .required(false)
                .index(3)
                .validator(|input| input_validator(input, parser::day_of_month)),
        )
        .arg(
            Arg::with_name("MONTH")
                .help("Allowed values 1-12 or JAN-DEC")
                .default_value("*")
                .required(false)
                .index(4)
                .validator(|input| input_validator(input, parser::month)),
        )
        .arg(
            Arg::with_name("DAY (of week)")
                .help("Allowed values 0-6 or SUN-SAT")
                .default_value("*")
                .required(false)
                .index(5)
                .validator(|input| input_validator(input, parser::day_of_week)),
        )
        .arg(
            Arg::with_name("random")
                .short("r")
                .long("random")
                .multiple(false)
                .help("Output a random cron schedule"),
        );
}
