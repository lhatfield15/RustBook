use std::env;
use std::process;

use minigrep;

fn main() {
    let config = minigrep::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("App err: {e}");
        process::exit(1);
    }
}
