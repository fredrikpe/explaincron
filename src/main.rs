mod clap_app;
mod parser;

use parser::CronElem::{List, Range, Single, Step, Wildcard};

struct Schedule {
    minute: parser::CronElem,
    hour: parser::CronElem,
    day_of_month: parser::CronElem,
    month: parser::CronElem,
    day_of_week: parser::CronElem,
}

impl Schedule {
    fn from_str(s: &str) -> Result<Schedule, String> {
        let split: Vec<&str> = s.split(" ").collect();
        if split.len() != 5 {
            return Err("malformed schedule: need 5 components".to_string());
        }

        Ok(Schedule {
            minute: input_from(split[0], parser::minute)?,
            hour: input_from(split[1], parser::hour)?,
            day_of_month: input_from(split[2], parser::day_of_month)?,
            month: input_from(split[3], parser::month)?,
            day_of_week: input_from(split[4], parser::day_of_week)?,
        })
    }
}

fn input_from(
    value: &str,
    elem_parser: fn(&str) -> Result<i32, String>,
) -> Result<parser::CronElem, String> {
    if value.contains('/') {
        parser::step(value.split('/'), elem_parser)
    } else if value.contains('-') {
        parser::range(value.split('-'), elem_parser)
    } else if value.contains(',') {
        parser::list(value.split(','), elem_parser)
    } else if value == "*" {
        Ok(Wildcard)
    } else {
        Ok(Single(elem_parser(value)?))
    }
}

fn ordinal(i: i32) -> String {
    if i == 1i32 {
        return "".to_string();
    }

    let s = i.to_string();

    if s.ends_with('1') && !s.ends_with("11") {
        format!("{}st ", i)
    } else if s.ends_with('2') && !s.ends_with("12") {
        format!("{}nd ", i)
    } else if s.ends_with('3') && !s.ends_with("13") {
        format!("{}rd ", i)
    } else {
        format!("{}th ", i)
    }
}

fn day_of_week_string(i: i32) -> String {
    match i {
        0 => "Sunday".to_string(),
        1 => "Monday".to_string(),
        2 => "Tuesday".to_string(),
        3 => "Wednesday".to_string(),
        4 => "Thursday".to_string(),
        5 => "Friday".to_string(),
        6 => "Saturday".to_string(),
        7 => "Sunday".to_string(),
        _ => panic!("should be validated"),
    }
}

fn month_string(i: i32) -> String {
    match i {
        1 => "January".to_string(),
        2 => "February".to_string(),
        3 => "March".to_string(),
        4 => "April".to_string(),
        5 => "May".to_string(),
        6 => "June".to_string(),
        7 => "July".to_string(),
        8 => "August".to_string(),
        9 => "September".to_string(),
        10 => "October".to_string(),
        11 => "November".to_string(),
        12 => "December".to_string(),
        _ => "January".to_string(),
    }
}

fn comma_list(list: &Vec<i32>, f: fn(i32) -> String) -> String {
    match list.len() {
        0 => "".to_string(),
        1 => format!("{}", f(list[0])),
        2 => format!("{} and {}", f(list[0]), f(list[1])),
        _ => {
            let (last, elements) = list.split_last().unwrap();
            format!(
                "{}, and {last}",
                elements
                    .iter()
                    .map(|i| f(*i))
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        }
    }
}

fn human_readable_schedule(schedule: Schedule) -> String {
    let mut result = "".to_string();
    match schedule.minute {
        Step(start, step) => result.push_str(&format!(
            "At every {}minute{}",
            ordinal(step),
            match start {
                Some(i) => format!(" from {i} through 59"),
                None => "".to_string(),
            }
        )),
        Range(start, stop) => {
            result.push_str(&format!("At every minute from {start} through {stop}"))
        }
        List(list) => result.push_str(&format!(
            "At minute {}",
            comma_list(&list, |i| i.to_string())
        )),
        Single(single) => result.push_str(&format!("At minute {single}")),
        Wildcard => result.push_str(&format!("At every minute")),
    }
    match schedule.hour {
        Step(start, step) => result.push_str(&format!(
            " past every {}hour{}",
            ordinal(step),
            match start {
                Some(i) => format!(" from {i} through 23"),
                None => "".to_string(),
            }
        )),
        Range(start, stop) => {
            result.push_str(&format!(" past every hour from {start} through {stop}"))
        }
        List(list) => result.push_str(&format!(
            " past hour {}",
            comma_list(&list, |i| i.to_string())
        )),
        Single(single) => result.push_str(&format!(" past hour {single}")),
        Wildcard => (),
    }
    match schedule.day_of_month {
        Step(start, step) => result.push_str(&format!(
            " on every {}day-of-month{}",
            ordinal(step),
            match start {
                Some(i) => format!(" from {i} through 31"),
                None => "".to_string(),
            }
        )),
        Range(start, stop) => result.push_str(&format!(
            " on every day-of-month from {start} through {stop}"
        )),
        List(ref list) => result.push_str(&format!(
            " on day-of-month {}",
            comma_list(&list, |i| i.to_string())
        )),
        Single(single) => result.push_str(&format!(" on day-of-month {single}")),
        Wildcard => (),
    }
    match schedule.month {
        Step(start, step) => result.push_str(&format!(
            " in every {}month{}",
            ordinal(step),
            match start {
                Some(i) => format!(" from {} through December", month_string(i)),
                None => "".to_string(),
            }
        )),
        Range(start, stop) => result.push_str(&format!(
            " in every month from {} through {}",
            month_string(start),
            month_string(stop)
        )),
        List(list) => result.push_str(&format!(" in {}", comma_list(&list, |i| month_string(i)))),
        Single(single) => result.push_str(&format!(" in {}", month_string(single))),
        Wildcard => (),
    }
    let day_of_week_prefix = match schedule.day_of_month {
        Step(None, _) => "if it's ",
        Wildcard => "",
        _ => "and ",
    };
    match schedule.day_of_week {
        Step(start, step) => result.push_str(&format!(
            " {}on every {}day-of-week{}",
            day_of_week_prefix,
            ordinal(step),
            match start {
                Some(i) => format!(" from {} through Sunday", day_of_week_string(i)),
                None => "".to_string(),
            }
        )),
        Range(start, stop) => result.push_str(&format!(
            " {}on every day-of-week from {} through {}",
            day_of_week_prefix,
            day_of_week_string(start),
            day_of_week_string(stop)
        )),
        List(list) => result.push_str(&format!(
            " {}on {}",
            day_of_week_prefix,
            comma_list(&list, |i| day_of_week_string(i))
        )),
        Single(single) => result.push_str(&format!(
            " {}on {}",
            day_of_week_prefix,
            day_of_week_string(single)
        )),
        Wildcard => (),
    }
    result.push_str(".");
    result
}

fn main() -> Result<(), String> {
    let matches = clap_app::app().get_matches();

    let first_arg = matches.value_of("MINUTE (or complete schedule)").unwrap();

    let schedule = if first_arg.contains(char::is_whitespace) {
        Schedule::from_str(first_arg).unwrap()
    } else {
        Schedule {
            minute: input_from(first_arg, parser::minute)?,
            hour: input_from(matches.value_of("HOUR").unwrap(), parser::hour)?,
            day_of_month: input_from(
                matches.value_of("DAY (of month)").unwrap(),
                parser::day_of_month,
            )?,
            month: input_from(matches.value_of("MONTH").unwrap(), parser::month)?,
            day_of_week: input_from(
                matches.value_of("DAY (of week)").unwrap(),
                parser::day_of_week,
            )?,
        }
    };

    println!("{}", human_readable_schedule(schedule));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_wildcards() {
        assert_eq!(
            human_readable_schedule(Schedule::from_str("* * * * *").unwrap()),
            "At every minute."
        );
    }

    #[test]
    fn minute_1() {
        assert_eq!(
            human_readable_schedule(Schedule::from_str("1 * * * *").unwrap()),
            "At minute 1."
        );
    }

    #[test]
    fn minute_2() {
        assert_eq!(
            human_readable_schedule(Schedule::from_str("2 * * * *").unwrap()),
            "At minute 2."
        );
    }

    #[test]
    fn minute_step() {
        assert_eq!(
            human_readable_schedule(Schedule::from_str("2/3 * * * *").unwrap()),
            "At every 3rd minute from 2 through 59."
        );
    }

    #[test]
    fn minute_step_hour_23() {
        assert_eq!(
            human_readable_schedule(Schedule::from_str("2/3 23 * * *").unwrap()),
            "At every 3rd minute from 2 through 59 past hour 23."
        );
    }

    #[test]
    fn minute_range() {
        assert_eq!(
            human_readable_schedule(Schedule::from_str("24-39 * * * *").unwrap()),
            "At every minute from 24 through 39."
        );
    }

    #[test]
    fn minute_list() {
        assert_eq!(
            human_readable_schedule(Schedule::from_str("24,39,42,13 * * * *").unwrap()),
            "At minute 24, 39, 42, and 13."
        );
    }

    #[test]
    fn day_of_week_step() {
        assert_eq!(
            human_readable_schedule(Schedule::from_str("* * * * 4/5").unwrap()),
            "At every minute on every 5th day-of-week from Thursday through Sunday."
        );
    }

    #[test]
    fn day_of_week_str() {
        assert_eq!(
            human_readable_schedule(Schedule::from_str("* * * * WED").unwrap()),
            "At every minute on Wednesday."
        );
    }

    #[test]
    fn month_str() {
        assert_eq!(
            human_readable_schedule(Schedule::from_str("* * * MAY *").unwrap()),
            "At every minute in May."
        );
    }

    #[test]
    fn cron_bug_test() {
        // https://crontab.guru/cron-bug.html
        assert_eq!(
            human_readable_schedule(Schedule::from_str("* * 3 * 1").unwrap()),
            "At every minute on day-of-month 3 and on Monday."
        );

        assert_eq!(
            human_readable_schedule(Schedule::from_str("* * */2 * 1").unwrap()),
            "At every minute on every 2nd day-of-month if it's on Monday."
        );

        assert_eq!(
            human_readable_schedule(Schedule::from_str("* * 1-3 * 1").unwrap()),
            "At every minute on every day-of-month from 1 through 3 and on Monday."
        );
    }
}
