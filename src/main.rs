mod clap_app;
mod cron;

use cron::Value::{List, Range, Single, Step, Wildcard};
use cron::{DayOfMonth, DayOfWeek, Hour, Minute, Month, Schedule};

fn random_cron_elem(min: i32, max: i32) -> cron::Value {
    match fastrand::i32(0..=10) {
        0 => match fastrand::i32(2..4) {
            3 => List((0..3).map(|_| fastrand::i32(min..=max)).collect()),
            4 => List((0..4).map(|_| fastrand::i32(min..=max)).collect()),
            _ => List((0..2).map(|_| fastrand::i32(min..=max)).collect()),
        },
        1 => {
            let start = fastrand::i32(min..max);
            Range(start, fastrand::i32(start..=max))
        }
        2 => Single(fastrand::i32(min..=max)),
        3 => Step(Some(fastrand::i32(min..=max)), fastrand::i32(min..=max)),
        _ => Wildcard,
    }
}

fn ordinal(i: i32) -> String {
    if i == 1i32 {
        return "".to_string();
    }

    let mut s = i.to_string();
    if s.ends_with('1') && !s.ends_with("11") {
        s.push_str("st ")
    } else if s.ends_with('2') && !s.ends_with("12") {
        s.push_str("nd ")
    } else if s.ends_with('3') && !s.ends_with("13") {
        s.push_str("rd ")
    } else {
        s.push_str("th ")
    }
    s
}

fn day_of_week_string(i: i32) -> String {
    match i {
        1 => "Monday".to_string(),
        2 => "Tuesday".to_string(),
        3 => "Wednesday".to_string(),
        4 => "Thursday".to_string(),
        5 => "Friday".to_string(),
        6 => "Saturday".to_string(),
        _ => "Sunday".to_string(),
    }
}

fn month_string(i: i32) -> String {
    match i {
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

fn join_oxford(vec: Vec<i32>, to_string: fn(i32) -> String) -> String {
    match vec.as_slice().split_last() {
        None => String::new(),
        Some((last, [])) => format!("{}", to_string(*last)),
        Some((last, [i])) => format!("{} and {}", to_string(*i), to_string(*last)),
        Some((last, first)) => format!(
            "{}, and {}",
            first.iter().fold(String::new(), |mut a, b| {
                if a.len() > 0 {
                    a.push_str(", ");
                }
                a.push_str(&to_string(*b));
                a
            }),
            to_string(*last)
        ),
    }
}

fn human_readable_schedule(schedule: Schedule) -> String {
    let mut result = "".to_string();
    match schedule.minute.value {
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
            join_oxford(list, |i| i.to_string())
        )),
        Single(single) => result.push_str(&format!("At minute {single}")),
        Wildcard => result.push_str(&format!("At every minute")),
    }
    match schedule.hour.value {
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
            join_oxford(list, |i| i.to_string())
        )),
        Single(single) => result.push_str(&format!(" past hour {single}")),
        Wildcard => (),
    }
    match schedule.day_of_month.value {
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
            join_oxford(list.to_vec(), |i| i.to_string())
        )),
        Single(single) => result.push_str(&format!(" on day-of-month {single}")),
        Wildcard => (),
    }
    match schedule.month.value {
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
        List(list) => result.push_str(&format!(
            " in {}",
            join_oxford(list, |i| month_string(i))
        )),
        Single(single) => result.push_str(&format!(" in {}", month_string(single))),
        Wildcard => (),
    }
    let day_of_week_prefix = match schedule.day_of_month.value {
        Step(None, _) => "if it's ",
        Wildcard => "",
        _ => "and ",
    };
    match schedule.day_of_week.value {
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
            join_oxford(list, |i| day_of_week_string(i))
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

    let schedule = if matches.is_present("random") {
        Schedule {
            minute: Minute {
                value: random_cron_elem(0, 59),
            },
            hour: Hour {
                value: random_cron_elem(0, 23),
            },
            day_of_month: DayOfMonth {
                value: random_cron_elem(1, 31),
            },
            month: Month {
                value: random_cron_elem(1, 12),
            },
            day_of_week: DayOfWeek {
                value: random_cron_elem(0, 6),
            },
        }
    } else {
        let first_arg = matches.value_of("MINUTE (or complete schedule)").unwrap();
        if first_arg.contains(char::is_whitespace) {
            Schedule::from_str(first_arg).unwrap()
        } else {
            Schedule {
                minute: Minute::from_str(first_arg)?,
                hour: Hour::from_str(matches.value_of("HOUR").unwrap())?,
                day_of_month: DayOfMonth::from_str(matches.value_of("DAY (of month)").unwrap())?,
                month: Month::from_str(matches.value_of("MONTH").unwrap())?,
                day_of_week: DayOfWeek::from_str(matches.value_of("DAY (of week)").unwrap())?,
            }
        }
    };

    if matches.is_present("random") {
        println!("{}", schedule.to_string());
    }
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

    #[test]
    fn join_oxford_test() {
        assert_eq!(join_oxford(Vec::<i32>::new(), |i| i.to_string()), "");
        assert_eq!(join_oxford(vec![1], |i| i.to_string()), "1");
        assert_eq!(join_oxford(vec![1, 2], |i| i.to_string()), "1 and 2");
        assert_eq!(join_oxford(vec![1, 2, 3], |i| i.to_string()), "1, 2, and 3");
    }
}
