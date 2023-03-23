mod clap_app;
mod cron;

use cron::{DayOfMonth, DayOfWeek, Hour, Minute, Month, Schedule};
use time::ext::NumericalDuration;
use time::{OffsetDateTime, UtcOffset};

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

    if matches.is_present("random") {
        println!("{}", schedule.to_string());
    }

    let num_next_occurrence = matches
        .value_of("num-next-occurrence")
        .map(|s| s.parse::<usize>().unwrap())
        .unwrap();

    let odt: OffsetDateTime = std::time::SystemTime::now().into();
    let offset = UtcOffset::current_local_offset().map_err(|_e| format!("date error"))?;
    let mut next = odt.to_offset(offset);

    for _ in 1..=num_next_occurrence {
        next = cron::next_occurrence(next, &schedule)?;

        println!(
            "{} {:0>2}:{:0>2}:{:0>2}",
            next.date(),
            next.hour(),
            next.minute(),
            next.second()
        );

        next = next.saturating_add(1.seconds());
    }

    Ok(())
}
