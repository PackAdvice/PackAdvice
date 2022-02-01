mod emoji;
mod log;

use getopts::Options;
use std::{env, process};
use packadvice::ExitCode;

macro_rules! packadvice_title {
    () => {
        "PackAdvice"
    };
}

fn main() {
    process::exit(run() as i32);
}

fn run() -> ExitCode {
    let mut options = Options::new();

    options.optflag("v", "version", "Prints version information");

    match options.parse(env::args().skip(1)) {
        Ok(option_matches) => {
            if option_matches.opt_present("v") {
                print_version_information();

                ExitCode::Success
            } else {
                match option_matches.free.first() {
                    Some(directory_path) => advice(directory_path),
                    None => {
                        println!("Usage:");
                        print!("    {} [OPTION]...", env!("CARGO_BIN_NAME"));
                        println!("{}", options.usage(""));

                        ExitCode::Success
                    }
                }
            }
        }
        Err(parse_err) => {
            error!(
                "{}\n\
                Run {} to see command line argument help",
                parse_err,
                env!("CARGO_BIN_NAME")
            );

            ExitCode::InputError
        }
    }
}

fn print_version_information() {
    println!("{} {}", packadvice_title!(), env!("CARGO_PKG_VERSION"));
}

fn advice(directory_path: &str) -> ExitCode {
    ExitCode::Success
}
