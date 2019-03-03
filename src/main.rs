extern crate dt;

use dt::{handle_commands, setup_logging};

fn main() {
    setup_logging();
    if let Err(msg) = handle_commands() {
        eprintln!("Error: {}", msg)
    }
}
