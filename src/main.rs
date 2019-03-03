extern crate chrono;
#[macro_use]
extern crate clap;
extern crate dt;
extern crate env_logger;
extern crate log;

use std::io::Write;

use chrono::Local;
use clap::App;
use env_logger::Builder;
use log::LevelFilter;

use dt::{clean, init};

fn main() {
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

    let yaml = load_yaml!("../config/dt.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        let project_name = matches.value_of("project_name").unwrap();
        init(project_name);
    }

    if matches.subcommand_matches("clean").is_some() {
        //TODO Print the error from clean
        if let Err(error) = clean() {
            eprintln!("{}", error)
        }
    }
}
