use clap::{App, Arg};

fn minute_validator(s: String) -> Result<(), String> {
    Ok(())
    //match s.parse::<i32>() {
    //Ok(_i) => Ok(()),
    //Err(_i) => Err(s),
    //}
}

const APP_NAME: &str = "asdf";

pub fn app() -> App<'static, 'static> {
    return App::new(APP_NAME)
        .version("0.1")
        .usage("explaincron [FLAGS] [ARGS]\n    explaincron 3-5 1/4 * * *")
        .author("Fredrik Pe <fredrikpei@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("MINUTE")
                .help("Allowed values 0-59")
                .default_value("*")
                .required(false)
                .index(1)
                .validator(minute_validator),
        )
        .arg(
            Arg::with_name("HOUR")
                .help("Allowed values 0-23")
                .default_value("*")
                .required(false)
                .index(2)
                .validator(|_s| Ok(())),
        )
        .arg(
            Arg::with_name("DAY (of month)")
                .help("Allowed values 1-31")
                .default_value("*")
                .required(false)
                .index(3)
                .validator(|_s| Ok(())),
        )
        .arg(
            Arg::with_name("MONTH")
                .help("Allowed values 1-12 or JAN-DEC")
                .default_value("*")
                .required(false)
                .index(4)
                .validator(|_s| Ok(())),
        )
        .arg(
            Arg::with_name("DAY (of week)")
                .help("Allowed values 0-6 or SUN-SAT")
                .default_value("*")
                .required(false)
                .index(5)
                .validator(|_s| Ok(())),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(false)
                .help("Sets the level of verbosity"),
        )
}

