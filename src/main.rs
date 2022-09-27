mod clap_app;
mod cron;

use cron::{DayOfMonth, DayOfWeek, Hour, Minute, Month, Schedule};

fn main() -> Result<(), String> {
    let matches = clap_app::app().get_matches();

    let schedule = if matches.is_present("random") {
        Schedule {
            minute: Minute {
                value: cron::random_value(0, 59),
            },
            hour: Hour {
                value: cron::random_value(0, 23),
            },
            day_of_month: DayOfMonth {
                value: cron::random_value(1, 31),
            },
            month: Month {
                value: cron::random_value(1, 12),
            },
            day_of_week: DayOfWeek {
                value: cron::random_value(0, 6),
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
    println!("{}", cron::human_readable(&schedule));

    Ok(())
}
