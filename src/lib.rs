extern crate chrono;
#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;

use std::io::Write;

use chrono::Local;
use clap::App;
use env_logger::Builder;
use log::LevelFilter;

use commands::{clean, init};

mod commands;

pub fn setup_logging() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
}

pub fn handle_commands() -> Result<(), String> {
    let yaml = load_yaml!("../config/dt.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        let project_name = matches.value_of("project_name").unwrap();
        init(project_name)
    } else if matches.subcommand_matches("clean").is_some() {
        clean()
    } else {
        Ok(())
    }
}
