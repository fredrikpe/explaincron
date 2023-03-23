use time::ext::NumericalDuration;
use time::OffsetDateTime;

const MONTH_NAMES: &[&str] = &[
    "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
];
const WEEK_DAY_NAMES: &[&str] = &["MON", "TUE", "WED", "THU", "FRI", "SAT", "SUN"];

pub struct Schedule {
    pub minute: Minute,
    pub hour: Hour,
    pub day_of_month: DayOfMonth,
    pub month: Month,
    pub day_of_week: DayOfWeek,
}

impl Schedule {
    pub fn from_str(s: &str) -> Result<Schedule, String> {
        let mut split = s.split(" ");
        let wrong_number_err = "schedule needs 5 components";

        Ok(Schedule {
            minute: Minute::from_str(split.next().ok_or(wrong_number_err)?)?,
            hour: Hour::from_str(split.next().ok_or(wrong_number_err)?)?,
            day_of_month: DayOfMonth::from_str(split.next().ok_or(wrong_number_err)?)?,
            month: Month::from_str(split.next().ok_or(wrong_number_err)?)?,
            day_of_week: DayOfWeek::from_str(split.next().ok_or(wrong_number_err)?)?,
        })
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} {} {} {} {}",
            self.minute.value.to_string(),
            self.hour.value.to_string(),
            self.day_of_month.value.to_string(),
            self.month.value.to_string(),
            self.day_of_week.value.to_string(),
        )
    }
}

pub fn next_occurrence(
    from_time: OffsetDateTime,
    schedule: &Schedule,
) -> Result<OffsetDateTime, String> {
    let mut next = from_time;

    let (month, wrapped) = next_month(next.month() as i32, &schedule.month);
    if month != next.month() {
        next = next.replace_second(0).unwrap();
        next = next.replace_hour(0).unwrap();
        next = next.replace_minute(0).unwrap();
        next = next.replace_day(1).unwrap();
        next = next
            .replace_month(month)
            .map_err(|e| format!("invalid month {}", e))?;
        next = next
            .replace_year(next.year() + if wrapped { 1 } else { 0 })
            .map_err(|e| format!("invalid year {}", e))?;
        return next_occurrence(next, &schedule);
    }

    let current_day = next.day() as i32;

    let (day, wrapped) = match (&schedule.day_of_month.value, &schedule.day_of_week.value) {
        (Value::Wildcard, _) => next_day_of_week(next, &schedule.day_of_week),
        (_, Value::Wildcard) => {
            next_day_of_month(current_day, max_days(next), &schedule.day_of_month)
        }
        _ => next_day_union(next, &schedule.day_of_week, &schedule.day_of_month),
    };

    if day != next.day() as i32 {
        next = next.replace_second(0).unwrap();
        next = next.replace_hour(0).unwrap();
        next = next.replace_minute(0).unwrap();
        next = next.replace_day(day as u8).unwrap();
        if wrapped {
            next = next
                .replace_month(next.month().next())
                .map_err(|e| format!("{}", e))?;
            if next.month() == time::Month::January {
                next = next
                    .replace_year(next.year() + 1)
                    .map_err(|e| format!("{}", e))?;
            }
        }
        return next_occurrence(next, &schedule);
    }

    let (hour, wrapped) = next_hour(next.hour() as i32, &schedule.hour);

    if hour != next.hour() as i32 {
        next = next.replace_second(0).unwrap();
        next = next.replace_minute(0).unwrap();
        next = next
            .replace_hour(hour as u8)
            .map_err(|_e| format!("date error 4"))?;
        if wrapped {
            next = next.saturating_add(1.days());
        }
        return next_occurrence(next, &schedule);
    }

    let wrapped = next.second() != 0;
    next = next
        .replace_second(0)
        .map_err(|_e| format!("date error 1"))?;

    let (minute, wrapped) = next_minute(
        next.minute() as i32 + if wrapped { 1 } else { 0 },
        &schedule.minute,
    );

    if wrapped || minute != next.minute() as i32 {
        next = next.replace_minute(minute as u8).unwrap();
        if wrapped {
            next = next.saturating_add(1.hours());
        }
        return next_occurrence(next, &schedule);
    }

    Ok(next)
}

fn next_minute(current: i32, minute: &Minute) -> (i32, bool) {
    next_value(current, 0, 59, &minute.value)
}

fn next_hour(current: i32, hour: &Hour) -> (i32, bool) {
    next_value(current, 0, 23, &hour.value)
}

fn is_leap_year(year: i32) -> bool {
    if year % 400 == 0 {
        return true;
    }

    if year % 100 == 0 {
        return false;
    }

    if year % 4 == 0 {
        return true;
    }
    false
}

fn max_days(datetime: OffsetDateTime) -> i32 {
    match datetime.month() {
        time::Month::January
        | time::Month::March
        | time::Month::May
        | time::Month::July
        | time::Month::August
        | time::Month::October
        | time::Month::December => 31,
        time::Month::February => {
            if is_leap_year(datetime.year()) {
                29
            } else {
                28
            }
        }
        _ => 31,
    }
}

fn next_day_of_week(datetime: OffsetDateTime, day_of_week: &DayOfWeek) -> (i32, bool) {
    let current_day = datetime.day() as i32;
    let current_weekday = datetime.weekday().number_from_monday() as i32;
    let (next, _) = next_value(current_weekday, 0, 6, &day_of_week.value);

    let num_days = (next - current_weekday).rem_euclid(7);
    if num_days == 0 {
        return (current_day, false);
    }

    next_day_of_month(
        current_day + 1,
        max_days(datetime),
        &DayOfMonth {
            value: Value::Step(Some(current_day), num_days),
        },
    )
}

fn next_day_of_month(current: i32, max_days: i32, day_of_month: &DayOfMonth) -> (i32, bool) {
    next_value(current, 1, max_days, &day_of_month.value)
}

fn next_day_union(
    datetime: OffsetDateTime,
    day_of_week: &DayOfWeek,
    day_of_month: &DayOfMonth,
) -> (i32, bool) {
    let current_day = datetime.day() as i32;
    let (day1, wrapped1) = next_day_of_week(datetime, day_of_week);
    let (day2, wrapped2) = next_day_of_month(current_day, max_days(datetime), day_of_month);

    if (day1 - current_day).rem_euclid(31) < (day2 - current_day).rem_euclid(31) {
        return (day1, wrapped1);
    }

    (day2, wrapped2)
}

fn next_month(current: i32, month: &Month) -> (time::Month, bool) {
    let (x, wrapped) = next_value(current, 1, 12, &month.value);
    let month = match x {
        1 => time::Month::January,
        2 => time::Month::February,
        3 => time::Month::March,
        4 => time::Month::April,
        5 => time::Month::May,
        6 => time::Month::June,
        7 => time::Month::July,
        8 => time::Month::August,
        9 => time::Month::September,
        10 => time::Month::October,
        11 => time::Month::November,
        12 => time::Month::December,
        _ => time::Month::January,
    };

    (month, wrapped)
}

fn next_value(current: i32, min: i32, max: i32, value: &Value) -> (i32, bool) {
    match value {
        Value::Step(start, step) => {
            let next = (start.unwrap_or(min)..=max)
                .step_by(*step as usize)
                .find(|i| current <= *i);
            match next {
                Some(n) => (n, false),
                None => (min, true),
            }
        }
        Value::Range(start, stop, step) => match (*start..=*stop)
            .step_by(step.map_or(1, |s| s as usize))
            .find(|i| current <= *i)
        {
            Some(n) => (n, false),
            None => (*start, true),
        },
        Value::List(ref list) => match list.iter().find(|i| current <= **i) {
            Some(n) => (*n, false),
            None => (list[0], true),
        },
        Value::Single(single) => (*single, current > *single),

        Value::Wildcard => match (min..=max).find(|i| current <= *i) {
            Some(n) => (n, false),
            None => (min, true),
        },
    }
}

pub fn human_readable(schedule: &Schedule) -> String {
    let mut result = String::new();

    match &schedule.minute.value {
        Value::Step(start, step) => result.push_str(&format!(
            "At every {}minute{}",
            ordinal(*step),
            match start {
                Some(i) => format!(" from {i} through 59"),
                None => "".to_string(),
            }
        )),
        Value::Range(start, stop, step) => result.push_str(&format!(
            "At every {}minute from {start} through {stop}",
            step.map_or("".to_string(), |s| ordinal(s))
        )),
        Value::List(list) => result.push_str(&format!(
            "At minute {}",
            join_oxford(list, |i| i.to_string())
        )),
        Value::Single(single) => result.push_str(&format!("At minute {single}")),
        Value::Wildcard => result.push_str(&format!("At every minute")),
    }

    match &schedule.hour.value {
        Value::Step(start, step) => result.push_str(&format!(
            " past every {}hour{}",
            ordinal(*step),
            match start {
                Some(i) => format!(" from {i} through 23"),
                None => "".to_string(),
            }
        )),
        Value::Range(start, stop, step) => {
            result.push_str(&format!(
                " past every {}hour from {start} through {stop}",
                step.map_or("".to_string(), |s| ordinal(s))
            ));
        }
        Value::List(list) => result.push_str(&format!(
            " past hour {}",
            join_oxford(list, |i| i.to_string())
        )),
        Value::Single(single) => result.push_str(&format!(" past hour {single}")),
        Value::Wildcard => (),
    }

    match &schedule.day_of_month.value {
        Value::Step(start, step) => result.push_str(&format!(
            " on every {}day-of-month{}",
            ordinal(*step),
            match start {
                Some(i) => format!(" from {i} through 31"),
                None => "".to_string(),
            }
        )),
        Value::Range(start, stop, step) => result.push_str(&format!(
            " on every {}day-of-month from {start} through {stop}",
            step.map_or("".to_string(), |s| ordinal(s)),
        )),
        Value::List(ref list) => result.push_str(&format!(
            " on day-of-month {}",
            join_oxford(list, |i| i.to_string())
        )),
        Value::Single(single) => result.push_str(&format!(" on day-of-month {single}")),
        Value::Wildcard => (),
    }

    match &schedule.month.value {
        Value::Step(start, step) => result.push_str(&format!(
            " in every {}month{}",
            ordinal(*step),
            match start {
                Some(i) => format!(" from {} through December", month_string(*i)),
                None => "".to_string(),
            }
        )),
        Value::Range(start, stop, step) => result.push_str(&format!(
            " in every {}month from {} through {}",
            step.map_or("".to_string(), |s| ordinal(s)),
            month_string(*start),
            month_string(*stop)
        )),
        Value::List(list) => {
            result.push_str(&format!(" in {}", join_oxford(list, |i| month_string(i))))
        }
        Value::Single(single) => result.push_str(&format!(" in {}", month_string(*single))),
        Value::Wildcard => (),
    }

    let day_of_week_prefix = match schedule.day_of_month.value {
        Value::Step(None, _) => "if it's ",
        Value::Wildcard => "",
        _ => "and ",
    };
    match &schedule.day_of_week.value {
        Value::Step(start, step) => result.push_str(&format!(
            " {}on every {}day-of-week{}",
            day_of_week_prefix,
            ordinal(*step),
            match start {
                Some(i) => format!(" from {} through Sunday", day_of_week_string(*i)),
                None => "".to_string(),
            }
        )),
        Value::Range(start, stop, step) => result.push_str(&format!(
            " {}on every {}day-of-week from {} through {}",
            day_of_week_prefix,
            step.map_or("".to_string(), |s| ordinal(s)),
            day_of_week_string(*start),
            day_of_week_string(*stop)
        )),
        Value::List(list) => result.push_str(&format!(
            " {}on {}",
            day_of_week_prefix,
            join_oxford(list, |i| day_of_week_string(i))
        )),
        Value::Single(single) => result.push_str(&format!(
            " {}on {}",
            day_of_week_prefix,
            day_of_week_string(*single)
        )),
        Value::Wildcard => (),
    }
    result.push_str(".");
    result
}

pub struct Minute {
    pub value: Value,
}

impl Minute {
    pub fn from_str(value: &str) -> Result<Minute, String> {
        Ok(Minute {
            value: Value::from_str(value, parse_minute)?,
        })
    }
}

pub struct Hour {
    pub value: Value,
}

impl Hour {
    pub fn from_str(value: &str) -> Result<Hour, String> {
        Ok(Hour {
            value: Value::from_str(value, parse_hour)?,
        })
    }
}

pub struct DayOfMonth {
    pub value: Value,
}

impl DayOfMonth {
    pub fn from_str(value: &str) -> Result<DayOfMonth, String> {
        Ok(DayOfMonth {
            value: Value::from_str(value, parse_day_of_month)?,
        })
    }
}

pub struct Month {
    pub value: Value,
}

impl Month {
    pub fn from_str(value: &str) -> Result<Month, String> {
        Ok(Month {
            value: Value::from_str(value, parse_month)?,
        })
    }
}

pub struct DayOfWeek {
    pub value: Value,
}

impl DayOfWeek {
    pub fn from_str(value: &str) -> Result<DayOfWeek, String> {
        Ok(DayOfWeek {
            value: Value::from_str(value, parse_day_of_week)?,
        })
    }
}

#[derive(Debug)]
pub enum Value {
    Step(Option<i32>, i32),
    Range(i32, i32, Option<i32>),
    List(Vec<i32>),
    Single(i32),
    Wildcard,
}

impl Value {
    fn from_str(
        value: &str,
        elem_parser: fn(&str) -> Result<i32, String>,
    ) -> Result<Value, String> {
        if value.contains('-') {
            parse_stepped_range(value, elem_parser)
        } else if value.contains('/') {
            parse_step(value, elem_parser)
        } else if value.contains(',') {
            parse_list(value, elem_parser)
        } else if value == "*" {
            Ok(Value::Wildcard)
        } else {
            Ok(Value::Single(elem_parser(value)?))
        }
    }
}

pub fn random_value(min: i32, max: i32) -> Value {
    match fastrand::i32(0..=10) {
        0 => match fastrand::i32(2..4) {
            3 => Value::List((0..3).map(|_| fastrand::i32(min..=max)).collect()),
            4 => Value::List((0..4).map(|_| fastrand::i32(min..=max)).collect()),
            _ => Value::List((0..2).map(|_| fastrand::i32(min..=max)).collect()),
        },
        1 => {
            let start = fastrand::i32(min..max);
            if fastrand::i32(0..=4) == 4 {
                Value::Range(
                    start,
                    fastrand::i32(start..=max),
                    Some(fastrand::i32(2..=4)),
                )
            } else {
                Value::Range(start, fastrand::i32(start..=max), None)
            }
        }
        2 => Value::Single(fastrand::i32(min..=max)),
        3 => Value::Step(Some(fastrand::i32(min..=max)), fastrand::i32(min..=max)),
        _ => Value::Wildcard,
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            Value::Step(Some(start), step) => format!("{start}/{step}"),
            Value::Step(None, step) => format!("*/{step}"),
            Value::Range(start, stop, step) => format!(
                "{start}-{stop}{}",
                match step {
                    Some(s) => "/".to_string() + s.to_string().as_ref(),
                    None => "".to_string(),
                }
            ),
            Value::List(list) => list
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(","),
            Value::Single(i) => i.to_string(),
            Value::Wildcard => "*".to_string(),
        };
        fmt.write_str(&str)?;

        Ok(())
    }
}

fn parse_list(input: &str, elem_parser: fn(&str) -> Result<i32, String>) -> Result<Value, String> {
    Ok(Value::List(
        input
            .split(",")
            .map(elem_parser)
            .collect::<Result<Vec<_>, _>>()?,
    ))
}

fn parse_stepped_range(
    input: &str,
    elem_parser: fn(&str) -> Result<i32, String>,
) -> Result<Value, String> {
    let split = input.split('/').collect::<Vec<&str>>();
    if split.len() > 2 {
        return Err(format!("only one '/' is allowed"));
    }
    let step = if split.len() == 2 {
        Some(elem_parser(split[1])?)
    } else {
        None
    };
    parse_range(split[0], step, elem_parser)
}

fn parse_range(
    input: &str,
    step: Option<i32>,
    elem_parser: fn(&str) -> Result<i32, String>,
) -> Result<Value, String> {
    let l = input
        .split("-")
        .map(elem_parser)
        .collect::<Result<Vec<_>, _>>()?;
    if l.len() != 2 {
        return Err(format!("range can have only two elements"));
    }
    if l[0] > l[1] {
        return Err(format!("range error {} is bigger than {}", l[0], l[1]));
    }
    Ok(Value::Range(l[0], l[1], step))
}

fn parse_step(input: &str, elem_parser: fn(&str) -> Result<i32, String>) -> Result<Value, String> {
    let mut iter = input.split('/');
    let start = iter
        .next()
        .ok_or(format!("step must have two elements"))
        .and_then(|s| {
            if s == "*" {
                Ok(None)
            } else {
                elem_parser(s).map(|i| Some(i))
            }
        })?;
    let step = iter
        .next()
        .ok_or(format!("step can only have two elements"))
        .and_then(elem_parser)?;
    if iter.next().is_some() {
        return Err(format!("step can only have two elements"));
    }

    Ok(Value::Step(start, step))
}

fn parse_minute(elem: &str) -> Result<i32, String> {
    elem.parse::<i32>()
        .map_err(|_| format!("'{elem}' is not a valid minute (0-59)"))
        .and_then(|i| {
            if i <= 59 && i >= 0 {
                Ok(i)
            } else {
                Err(format!("'{i}' is not a valid minute (0-59)"))
            }
        })
}

fn parse_hour(elem: &str) -> Result<i32, String> {
    elem.parse::<i32>()
        .map_err(|_| format!("'{elem}' is not a valid hour (0-23)"))
        .and_then(|i| {
            if i <= 23 && i >= 0 {
                Ok(i)
            } else {
                Err(format!("'{i}' is not a valid hour (0-23)"))
            }
        })
}

fn parse_day_of_month(elem: &str) -> Result<i32, String> {
    elem.parse::<i32>()
        .map_err(|_| format!("'{elem}' is a valid day-of-month (1-31)"))
        .and_then(|i| {
            if i <= 31 && i >= 1 {
                Ok(i)
            } else {
                Err(format!("'{i}' is not a valid day-of-month (1-31)"))
            }
        })
}

fn parse_month(elem: &str) -> Result<i32, String> {
    match MONTH_NAMES.iter().position(|x| x == &elem.to_uppercase()) {
        Some(i) => Ok((i + 1) as i32),
        None => elem
            .parse::<i32>()
            .map_err(|_| format!("'{elem}' is not a valid month (1-12 or JAN-DEC)"))
            .and_then(|i| {
                if i <= 12 && i >= 1 {
                    Ok(i)
                } else {
                    Err(format!("'{i}' is not a valid month (1-12 or JAN-DEC)"))
                }
            }),
    }
}

fn parse_day_of_week(elem: &str) -> Result<i32, String> {
    match WEEK_DAY_NAMES
        .iter()
        .position(|x| x == &elem.to_uppercase())
    {
        Some(i) => Ok((i + 1) as i32),
        None => elem
            .parse::<i32>()
            .map_err(|_| format!("'{elem}' is not a valid day-of-week (0-6 or MON-SUN)"))
            .and_then(|i| {
                if i <= 6 && i >= 0 {
                    Ok(i)
                } else {
                    Err(format!("'{i}' is not a valid day-of-week (0-6 or MON-SUN)"))
                }
            }),
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

fn join_oxford(vec: &Vec<i32>, to_string: fn(i32) -> String) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    #[test]
    fn only_wildcards() {
        assert_eq!(
            human_readable(&Schedule::from_str("* * * * *").unwrap()),
            "At every minute."
        );
    }

    #[test]
    fn minute_1() {
        assert_eq!(
            human_readable(&Schedule::from_str("1 * * * *").unwrap()),
            "At minute 1."
        );
    }

    #[test]
    fn minute_2() {
        assert_eq!(
            human_readable(&Schedule::from_str("2 * * * *").unwrap()),
            "At minute 2."
        );
    }

    #[test]
    fn minute_step() {
        assert_eq!(
            human_readable(&Schedule::from_str("2/3 * * * *").unwrap()),
            "At every 3rd minute from 2 through 59."
        );
    }

    #[test]
    fn minute_step_hour_23() {
        assert_eq!(
            human_readable(&Schedule::from_str("2/3 23 * * *").unwrap()),
            "At every 3rd minute from 2 through 59 past hour 23."
        );
    }

    #[test]
    fn minute_range() {
        assert_eq!(
            human_readable(&Schedule::from_str("24-39 * * * *").unwrap()),
            "At every minute from 24 through 39."
        );
    }

    #[test]
    fn minute_list() {
        assert_eq!(
            human_readable(&Schedule::from_str("24,39,42,13 * * * *").unwrap()),
            "At minute 24, 39, 42, and 13."
        );
    }

    #[test]
    fn day_of_week_step() {
        assert_eq!(
            human_readable(&Schedule::from_str("* * * * 4/5").unwrap()),
            "At every minute on every 5th day-of-week from Thursday through Sunday."
        );
    }

    #[test]
    fn day_of_week_str() {
        assert_eq!(
            human_readable(&Schedule::from_str("* * * * WED").unwrap()),
            "At every minute on Wednesday."
        );
    }

    #[test]
    fn month_str() {
        assert_eq!(
            human_readable(&Schedule::from_str("* * * MAY *").unwrap()),
            "At every minute in May."
        );
    }

    #[test]
    fn cron_bug_test() {
        // https://crontab.guru/cron-bug.html
        assert_eq!(
            human_readable(&Schedule::from_str("* * 3 * 1").unwrap()),
            "At every minute on day-of-month 3 and on Monday."
        );

        assert_eq!(
            human_readable(&Schedule::from_str("* * */2 * 1").unwrap()),
            "At every minute on every 2nd day-of-month if it's on Monday."
        );

        assert_eq!(
            human_readable(&Schedule::from_str("* * 1-3 * 1").unwrap()),
            "At every minute on every day-of-month from 1 through 3 and on Monday."
        );
    }

    #[test]
    fn join_oxford_test() {
        assert_eq!(join_oxford(&Vec::<i32>::new(), |i| i.to_string()), "");
        assert_eq!(join_oxford(&vec![1], |i| i.to_string()), "1");
        assert_eq!(join_oxford(&vec![1, 2], |i| i.to_string()), "1 and 2");
        assert_eq!(
            join_oxford(&vec![1, 2, 3], |i| i.to_string()),
            "1, 2, and 3"
        );
    }

    #[test]
    fn ignore_case() {
        assert_eq!(
            human_readable(&Schedule::from_str("* * * * Sun").unwrap()),
            "At every minute on Sunday."
        );
        assert_eq!(
            human_readable(&Schedule::from_str("* * * * mon").unwrap()),
            "At every minute on Monday."
        );
        assert_eq!(
            human_readable(&Schedule::from_str("* * * mar *").unwrap()),
            "At every minute in March."
        );
    }

    #[test]
    fn next_occ() {
        let datetime = datetime!(2022-01-01 13:00:55 +0:00:00);
        let schedule = Schedule::from_str("* * * * *").unwrap();

        assert_eq!(
            datetime!(2022-01-01 13:01:00 +0:00:00),
            next_occurrence(datetime, &schedule).unwrap()
        );
    }

    #[test]
    fn next_occ_wrap_hour() {
        let datetime = datetime!(2022-01-01 13:59:55 +0:00:00);
        let schedule = Schedule::from_str("* * * * *").unwrap();

        assert_eq!(
            datetime!(2022-01-01 14:00:00 +0:00:00),
            next_occurrence(datetime, &schedule).unwrap()
        );
    }

    #[test]
    fn next_occ_all_wrap() {
        let datetime = datetime!(2022-12-31 23:59:55 +0:00:00);
        let schedule = Schedule::from_str("* * * * *").unwrap();

        assert_eq!(
            datetime!(2023-01-01 00:00:00 +0:00:00),
            next_occurrence(datetime, &schedule).unwrap()
        );
    }

    #[test]
    fn next_occ_all_wrap2() {
        let datetime = datetime!(2022-12-31 23:59:55 +0:00:00);
        let schedule = Schedule::from_str("* * * * MON").unwrap();

        assert_eq!(
            datetime!(2023-01-02 00:00:00 +0:00:00),
            next_occurrence(datetime, &schedule).unwrap()
        );
    }

    #[test]
    fn next_occ_union() {
        let datetime = datetime!(2023-03-21 00:00:55 +0:00:00); // Tuesday
        let schedule = Schedule::from_str("34 * 1 * MON,FRI").unwrap();

        assert_eq!(
            datetime!(2023-03-24 00:34:00 +0:00:00),
            next_occurrence(datetime, &schedule).unwrap()
        );
    }

    #[test]
    fn next_occ_union2() {
        let datetime = datetime!(2023-03-21 00:55:55 +0:00:00); // Tuesday
        let schedule = Schedule::from_str("34 * 1 * MON,FRI").unwrap();

        assert_eq!(
            datetime!(2023-03-24 00:34:00 +0:00:00),
            next_occurrence(datetime, &schedule).unwrap()
        );
    }

    #[test]
    fn next_occ_from_crontab_guru_1() {
        let datetime = datetime!(2023-03-22 12:12:55 +0:00:00); // Tuesday
        let schedule = Schedule::from_str("15 14 1 * *").unwrap();

        assert_eq!(
            datetime!(2023-04-01 14:15:00 +0:00:00),
            next_occurrence(datetime, &schedule).unwrap()
        );
    }

    #[test]
    fn next_occ_from_crontab_guru_2() {
        let datetime = datetime!(2023-03-22 12:12:55 +0:00:00); // Tuesday
        let schedule = Schedule::from_str("0 22 * * 1-5").unwrap();

        assert_eq!(
            datetime!(2023-03-22 22:00:00 +0:00:00),
            next_occurrence(datetime, &schedule).unwrap()
        );
    }

    #[test]
    fn next_occ_from_crontab_guru_3() {
        let datetime = datetime!(2023-03-22 12:12:55 +0:00:00); // Tuesday
                                                                // TODO: Implement stepped ranges.
        let schedule = Schedule::from_str("23 0-20/2 * * *").unwrap();

        assert_eq!(
            datetime!(2023-03-22 12:23:00 +0:00:00),
            next_occurrence(datetime, &schedule).unwrap()
        );
    }
}
