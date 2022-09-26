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

pub enum Value {
    Step(Option<i32>, i32),
    Range(i32, i32),
    List(Vec<i32>),
    Single(i32),
    Wildcard,
}

impl Value {
    fn from_str(
        value: &str,
        elem_parser: fn(&str) -> Result<i32, String>,
    ) -> Result<Value, String> {
        if value.contains('/') {
            parse_step(value, elem_parser)
        } else if value.contains('-') {
            parse_range(value, elem_parser)
        } else if value.contains(',') {
            parse_list(value, elem_parser)
        } else if value == "*" {
            Ok(Value::Wildcard)
        } else {
            Ok(Value::Single(elem_parser(value)?))
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            Value::Step(Some(start), step) => format!("{start}/{step}"),
            Value::Step(None, step) => format!("*/{step}"),
            Value::Range(start, stop) => format!("{start}-{stop}"),
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

fn parse_range(input: &str, elem_parser: fn(&str) -> Result<i32, String>) -> Result<Value, String> {
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
    Ok(Value::Range(l[0], l[1]))
}

fn parse_step(input: &str, elem_parser: fn(&str) -> Result<i32, String>) -> Result<Value, String> {
    let mut iter = input.split('/');
    let start = iter
        .next()
        .ok_or(format!("step must have two elements"))
        .map(|s| {
            if s == "*" {
                None
            } else {
                Some(elem_parser(s).ok()?)
            }
        })?;
    let step = iter
        .next()
        .ok_or(format!("step must have two elements"))
        .and_then(elem_parser)?;
    if iter.next().is_some() {
        return Err(format!("step must have two elements"));
    }

    Ok(Value::Step(start, step))
}

fn parse_minute(elem: &str) -> Result<i32, String> {
    elem.parse::<i32>()
        .map_err(|_| format!("{elem} is not in 0-59"))
        .and_then(|i| {
            if i <= 59 && i >= 0 {
                Ok(i)
            } else {
                Err(format!("{i} is not in 0-59"))
            }
        })
}

fn parse_hour(elem: &str) -> Result<i32, String> {
    elem.parse::<i32>()
        .map_err(|_| format!("{elem} is not in 0-23"))
        .and_then(|i| {
            if i <= 23 && i >= 0 {
                Ok(i)
            } else {
                Err(format!("{i} is not in 0-23"))
            }
        })
}

fn parse_day_of_month(elem: &str) -> Result<i32, String> {
    elem.parse::<i32>()
        .map_err(|_| format!("{elem} is not in 1-31"))
        .and_then(|i| {
            if i <= 31 && i >= 1 {
                Ok(i)
            } else {
                Err(format!("{i} is not in 1-31"))
            }
        })
}

fn parse_month(elem: &str) -> Result<i32, String> {
    match MONTH_NAMES.iter().position(|x| x == &elem) {
        Some(i) => Ok((i + 1) as i32),
        None => elem
            .parse::<i32>()
            .map_err(|_| format!("{elem} is not in 1-12 or JAN-DEC"))
            .and_then(|i| {
                if i <= 12 && i >= 1 {
                    Ok(i)
                } else {
                    Err(format!("{i} is not in 1-12 or JAN-DEC"))
                }
            }),
    }
}

fn parse_day_of_week(elem: &str) -> Result<i32, String> {
    match WEEK_DAY_NAMES.iter().position(|x| x == &elem) {
        Some(i) => Ok((i + 1) as i32),
        None => elem
            .parse::<i32>()
            .map_err(|_| format!("{elem} is not in 0-6 or MON-SUN"))
            .and_then(|i| {
                if i <= 6 && i >= 0 {
                    Ok(i)
                } else {
                    Err(format!("{i} is not in 0-6 or MON-SUN"))
                }
            }),
    }
}
