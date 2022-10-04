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
        let first_arg = matches.value_of("SCHEDULE").unwrap();
        Schedule::from_str(first_arg).unwrap()
    };

    if matches.is_present("random") {
        println!("{}", schedule.to_string());
    }
    println!("{}", cron::human_readable(&schedule));

    Ok(())
}
