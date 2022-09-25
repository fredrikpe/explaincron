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

impl CronElem {
    pub fn from_str(
        value: &str,
        elem_parser: fn(&str) -> Result<i32, String>,
    ) -> Result<CronElem, String> {
        if value.contains('/') {
            step(value.split('/'), elem_parser)
        } else if value.contains('-') {
            range(value.split('-'), elem_parser)
        } else if value.contains(',') {
            list(value.split(','), elem_parser)
        } else if value == "*" {
            Ok(CronElem::Wildcard)
        } else {
            Ok(CronElem::Single(elem_parser(value)?))
        }
    }
}

impl std::fmt::Display for CronElem {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            CronElem::Step(Some(start), step) => format!("{start}/{step}"),
            CronElem::Step(None, step) => format!("*/{step}"),
            CronElem::Range(start, stop) => format!("{start}-{stop}"),
            CronElem::List(list) => list
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(","),
            CronElem::Single(i) => i.to_string(),
            CronElem::Wildcard => "*".to_string(),
        };
        fmt.write_str(&str)?;

        Ok(())
    }
}

fn list<'a, I>(iter: I, elem_parser: fn(&str) -> Result<i32, String>) -> Result<CronElem, String>
where
    I: Iterator<Item = &'a str>,
{
    Ok(CronElem::List(
        iter.map(elem_parser).collect::<Result<Vec<_>, _>>()?,
    ))
}

fn range<'a, I>(iter: I, elem_parser: fn(&str) -> Result<i32, String>) -> Result<CronElem, String>
where
    I: Iterator<Item = &'a str>,
{
    let l = iter.map(elem_parser).collect::<Result<Vec<_>, _>>()?;
    if l.len() != 2 {
        return Err(format!("range can have only two elements"));
    }
    if l[0] > l[1] {
        return Err(format!("range error {} is bigger than {}", l[0], l[1]));
    }
    Ok(CronElem::Range(l[0], l[1]))
}

fn step<'a, I>(
    mut iter: I,
    elem_parser: fn(&str) -> Result<i32, String>,
) -> Result<CronElem, String>
where
    I: Iterator<Item = &'a str>,
{
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
