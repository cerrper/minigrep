use cerrper_grep;
use std::{env, process};

fn main() {
    let config = cerrper_grep::Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = cerrper_grep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
