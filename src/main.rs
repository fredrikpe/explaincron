mod clap_app;

enum Input {
    Step(i32, i32),
    Range(i32, i32),
    List(Vec<i32>),
    Single(i32),
    Wildcard,
}

struct Schedule {
    minute: Input,
    hour: Input,
    day_of_month: Input,
    month: Input,
    day_of_week: Input,
}

impl Schedule {
    fn from_str(s: &str) -> Result<Schedule, String> {
        let split: Vec<&str> = s.split(" ").collect();
        if split.len() != 5 {
            return Err("malformed schedule: need 5 components".to_string());
        }

        Ok(Schedule {
            minute: input_from(split[0])?,
            hour: input_from(split[1])?,
            day_of_month: input_from(split[2])?,
            month: input_from(split[3])?,
            day_of_week: input_from(split[4])?,
        })
    }
}


fn input_from(value: &str) -> Result<Input, String> {
    if value.contains('/') {
        let mut split = value.split('/');
        let start = split
            .next()
            .and_then(|s| s.parse::<i32>().ok())
            .ok_or("malformed input")?;
        let step = split
            .next()
            .and_then(|s| s.parse::<i32>().ok())
            .ok_or("malformed input")?;
        return Ok(Input::Step(start, step));
    }

    if value.contains('-') {
        let mut split = value.split('-');
        let start = split
            .next()
            .and_then(|s| s.parse::<i32>().ok())
            .ok_or("malformed input")?;
        let stop = split
            .next()
            .and_then(|s| s.parse::<i32>().ok())
            .ok_or("malformed input")?;
        return Ok(Input::Range(start, stop));
    }

    if value.contains(',') {
        let split = value
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        return Ok(Input::List(split));
    }

    if value == "*" {
        return Ok(Input::Wildcard);
    }
    return Ok(Input::Single(value.parse::<i32>().unwrap()));
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
        0 => "Monday".to_string(),
        1 => "Tuesday".to_string(),
        2 => "Wednesday".to_string(),
        3 => "Thursday".to_string(),
        4 => "Friday".to_string(),
        5 => "Saturday".to_string(),
        6 => "Sunday".to_string(),
        _ => "January".to_string(),
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
        Input::Step(start, step) => result.push_str(&format!(
            "At every {}minute from {start} through 59",
            ordinal(step)
        )),
        Input::Range(start, stop) => {
            result.push_str(&format!("At every minute from {start} through {stop}"))
        }
        Input::List(list) => result.push_str(&format!(
            "At minute {}",
            comma_list(&list, |i| i.to_string())
        )),
        Input::Single(single) => result.push_str(&format!("At minute {single}")),
        Input::Wildcard => result.push_str(&format!("At every minute")),
    }
    match schedule.hour {
        Input::Step(start, step) => result.push_str(&format!(
            " past every {}hour from {} through 23",
            ordinal(step),
            start
        )),
        Input::Range(start, stop) => {
            result.push_str(&format!(" past every hour from {start} through {stop}"))
        }
        Input::List(list) => result.push_str(&format!(
            " past hour {}",
            comma_list(&list, |i| i.to_string())
        )),
        Input::Single(single) => result.push_str(&format!(" past hour {single}")),
        Input::Wildcard => (),
    }
    match schedule.day_of_month {
        Input::Step(start, step) => result.push_str(&format!(
            " on every {}day-of-month from {} through 31",
            ordinal(step),
            start
        )),
        Input::Range(start, stop) => result.push_str(&format!(
            " on every day-of-month from {start} through {stop}"
        )),
        Input::List(list) => result.push_str(&format!(
            " on day-of-month {}",
            comma_list(&list, |i| i.to_string())
        )),
        Input::Single(single) => result.push_str(&format!(" on day-of-month {single}")),
        Input::Wildcard => (),
    }
    match schedule.month {
        Input::Step(start, step) => result.push_str(&format!(
            " in every {}month from {} through December",
            ordinal(step),
            month_string(start)
        )),
        Input::Range(start, stop) => result.push_str(&format!(
            " in every month from {} through {}",
            month_string(start),
            month_string(stop)
        )),
        Input::List(list) => {
            result.push_str(&format!(" in {}", comma_list(&list, |i| month_string(i))))
        }
        Input::Single(single) => result.push_str(&format!(" in {}", month_string(single))),
        Input::Wildcard => (),
    }
    match schedule.day_of_week {
        Input::Step(start, step) => result.push_str(&format!(
            " on every {}day-of-week from {} through Sunday",
            ordinal(step),
            day_of_week_string(start)
        )),
        Input::Range(start, stop) => result.push_str(&format!(
            " on every day-of-week from {} through {}",
            day_of_week_string(start),
            day_of_week_string(stop)
        )),
        Input::List(list) => result.push_str(&format!(
            " on {}",
            comma_list(&list, |i| day_of_week_string(i))
        )),
        Input::Single(single) => result.push_str(&format!(" on {}", day_of_week_string(single))),
        Input::Wildcard => (),
    }
    result.push_str(".");
    result
}

fn main() -> Result<(), String> {
    let matches = clap_app::app().get_matches();

    let schedule = Schedule {
        minute: input_from(matches.value_of("MINUTE").unwrap())?,
        hour: input_from(matches.value_of("HOUR").unwrap())?,
        day_of_month: input_from(matches.value_of("DAY (of month)").unwrap())?,
        month: input_from(matches.value_of("MONTH").unwrap())?,
        day_of_week: input_from(matches.value_of("DAY (of week)").unwrap())?,
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
}
