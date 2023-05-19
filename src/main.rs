use std::process;

use minils::Config;

fn main() {
    let config = Config::new();

    if let Err(e) = minils::run(&config) {
        eprint!("Application error: {}\n", e);
        process::exit(1);
    }
}
