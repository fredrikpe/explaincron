use clap::{App, Arg};

const MONTH_NAMES: &[&str] = &["JAN", "FEB", "MAR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC"];
const WEEK_DAY_NAMES: &[&str] = &["MON", "TUE", "WED", "THU", "FRI", "SAT", "SUN"];

fn minute_validator(elem: &str) -> Result<(), String> {
    elem.parse::<i32>()
        .map_err(|_| format!("{elem} is not in 0-59"))
        .and_then(|i| {
            if i <= 59 && i >= 0 {
                Ok(())
            } else {
                Err(format!("{i} is not in 0-59"))
            }
        })
}

fn hour_validator(elem: &str) -> Result<(), String> {
    elem.parse::<i32>()
        .map_err(|_| format!("{elem} is not in 0-23"))
        .and_then(|i| {
            if i <= 23 && i >= 0 {
                Ok(())
            } else {
                Err(format!("{i} is not in 0-23"))
            }
        })
}

fn day_of_month_validator(elem: &str) -> Result<(), String> {
    elem.parse::<i32>()
        .map_err(|_| format!("{elem} is not in 1-31"))
        .and_then(|i| {
            if i <= 31 && i >= 1 {
                Ok(())
            } else {
                Err(format!("{i} is not in 1-31"))
            }
        })
}

fn month_validator(elem: &str) -> Result<(), String> {
    if MONTH_NAMES.contains(&elem) {
        return Ok(())
    } else {
        elem.parse::<i32>()
            .map_err(|_| format!("{elem} is not in 1-12 or JAN-DEC"))
            .and_then(|i| {
                if i <= 12 && i >= 1 {
                    Ok(())
                } else {
                    Err(format!("{i} is not in 1-12 or JAN-DEC"))
                }
            })
    }
}

fn day_of_week_validator(elem: &str) -> Result<(), String> {
    if WEEK_DAY_NAMES.contains(&elem) {
        return Ok(())
    } else {
        elem.parse::<i32>()
            .map_err(|_| format!("{elem} is not in 0-6 or MON-SUN"))
            .and_then(|i| {
                if i <= 6 && i >= 0 {
                    Ok(())
                } else {
                    Err(format!("{i} is not in 0-6 or MON-SUN"))
                }
            })
    }
}

fn list_validator(
    split: std::str::Split<char>,
    elem_parser: fn(&str) -> Result<(), String>,
) -> Result<(), String> {
    split
        .map(elem_parser)
        .collect::<Result<Vec<_>, _>>()
        .map(|_| ())
}

fn range_validator(
    split: std::str::Split<char>,
    elem_parser: fn(&str) -> Result<(), String>,
) -> Result<(), String> {
    let l = split.map(elem_parser).collect::<Result<Vec<_>, _>>()?;
    if l.len() != 2 {
        return Err(format!("range can have only two elements"));
    }
    Ok(())
}

fn step_validator(
    split: std::str::Split<char>,
    elem_parser: fn(&str) -> Result<(), String>,
) -> Result<(), String> {
    let l = split.map(elem_parser).collect::<Result<Vec<_>, _>>()?;
    if l.len() != 2 {
        return Err(format!("step can have only two elements"));
    }
    Ok(())
}

fn input_validator(input: String, elem_validator: fn(&str) -> Result<(), String>) -> Result<(), String> {
    return if input.contains(',') {
        list_validator(input.split(','), elem_validator)
    } else if input.contains('-') {
        range_validator(input.split('-'), elem_validator)
    } else if input.contains('/') {
        step_validator(input.split('/'), elem_validator)
    } else if input == "*" {
        Ok(())
    } else {
        elem_validator(&input)
    };
}

const APP_NAME: &str = "asdf";

pub fn app() -> App<'static, 'static> {
    return App::new(APP_NAME)
        .version("0.1")
        .usage("explaincron [FLAGS] [ARGS]\n    explaincron 3-5 1/4 * * *")
        .author("Fredrik Pe <fredrikpei@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("MINUTE")
                .help("Allowed values 0-59")
                .default_value("*")
                .required(false)
                .index(1)
                .validator(|input| input_validator(input, minute_validator)),
        )
        .arg(
            Arg::with_name("HOUR")
                .help("Allowed values 0-23")
                .default_value("*")
                .required(false)
                .index(2)
                .validator(|input| input_validator(input, hour_validator)),
        )
        .arg(
            Arg::with_name("DAY (of month)")
                .help("Allowed values 1-31")
                .default_value("*")
                .required(false)
                .index(3)
                .validator(|input| input_validator(input, day_of_month_validator)),
        )
        .arg(
            Arg::with_name("MONTH")
                .help("Allowed values 1-12 or JAN-DEC")
                .default_value("*")
                .required(false)
                .index(4)
                .validator(|input| input_validator(input, month_validator)),
        )
        .arg(
            Arg::with_name("DAY (of week)")
                .help("Allowed values 0-6 or SUN-SAT")
                .default_value("*")
                .required(false)
                .index(5)
                .validator(|input| input_validator(input, day_of_week_validator)),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(false)
                .help("Sets the level of verbosity"),
        );
}
