mod log;

use getopts::Options;
use std::{env, process};

macro_rules! packadvice_title {
    () => {
        "PackAdvice"
    };
}

fn main() {
    process::exit(run());
}

fn run() -> i32 {
    let mut options = Options::new();

    options.optflag("v", "version", "Prints version information");

    match options.parse(env::args().skip(1)) {
        Ok(option_matches) => {
            if option_matches.opt_present("v") {
                print_version_information();

                0
            } else {
                match option_matches.free.first() {
                    Some(directory_path) => advice(directory_path),
                    None => {
                        println!("Usage:");
                        print!("    {} [OPTION]...", env!("CARGO_BIN_NAME"));
                        println!("{}", options.usage(""));

                        0
                    }
                }
            }
        }
        Err(parse_err) => {
            error!("{}", parse_err);

            1
        }
    }
}

fn print_version_information() {
    println!("{} {}", packadvice_title!(), env!("CARGO_PKG_VERSION"));
}

fn advice(directory_path: &str) -> i32 {
    0
}
