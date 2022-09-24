const MONTH_NAMES: &[&str] = &[
    "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
];
const WEEK_DAY_NAMES: &[&str] = &["MON", "TUE", "WED", "THU", "FRI", "SAT", "SUN"];

pub enum CronElem {
    Step(Option<i32>, i32),
    Range(i32, i32),
    List(Vec<i32>),
    Single(i32),
    Wildcard,
}

pub fn list(
    split: std::str::Split<char>,
    elem_parser: fn(&str) -> Result<i32, String>,
) -> Result<CronElem, String> {
    Ok(CronElem::List(
        split.map(elem_parser).collect::<Result<Vec<_>, _>>()?,
    ))
}

pub fn range(
    split: std::str::Split<char>,
    elem_parser: fn(&str) -> Result<i32, String>,
) -> Result<CronElem, String> {
    let l = split.map(elem_parser).collect::<Result<Vec<_>, _>>()?;
    if l.len() != 2 {
        return Err(format!("range can have only two elements"));
    }
    Ok(CronElem::Range(l[0], l[1]))
}

pub fn step(
    mut split: std::str::Split<char>,
    elem_parser: fn(&str) -> Result<i32, String>,
) -> Result<CronElem, String> {
    let start = split
        .next()
        .ok_or(format!("step must have two elements"))
        .map(|s| {
            if s == "*" {
                None
            } else {
                Some(elem_parser(s).ok()?)
            }
        })?;
    let step = split
        .next()
        .ok_or(format!("step must have two elements"))
        .and_then(elem_parser)?;
    if split.next().is_some() {
        return Err(format!("step must have two elements"));
    }

    Ok(CronElem::Step(start, step))
}

pub fn minute(elem: &str) -> Result<i32, String> {
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

pub fn hour(elem: &str) -> Result<i32, String> {
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

pub fn day_of_month(elem: &str) -> Result<i32, String> {
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

pub fn month(elem: &str) -> Result<i32, String> {
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

pub fn day_of_week(elem: &str) -> Result<i32, String> {
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
