use chrono::Local;
use clap::ArgMatches;
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::io::Write;

pub fn newbuilder(matches: &ArgMatches) {
    let level = get_level(matches);
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%d/%m/%Y %H:%M:%S %p"),
                record.level(),
                record.args()
            )
        })
        .filter(None, level)
        .target(Target::Stderr)
        .init();
}

fn get_level(matches: &ArgMatches) -> LevelFilter {
    // Checking verbose
    // Ugly, but needed - May end up in a small library later
    let mut level = LevelFilter::Info;
    if matches.is_present("quiet") {
        level = LevelFilter::Warn;
    } else if matches.is_present("verbose") {
        if matches.value_of("verbose").unwrap() == "v1" {
            level = LevelFilter::Debug;
        } else if matches.value_of("verbose").unwrap() == "v" {
            level = LevelFilter::Trace
        }
    }
    level
}
