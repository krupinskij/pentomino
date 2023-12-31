use pentomino::config::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    Config::validate(&config).unwrap_or_else(|err| {
        eprintln!("Problem validating configuration: {err}");
        process::exit(1);
    });

    pentomino::run(config);
}
