mod emoji;
mod exit_code;
mod log;

use crate::exit_code::ExitCode;
use getopts::Options;
use packadvice::{PackAdviser, PackAdviserStatus, PackAdviserStatusType, PackOptions};
use std::path::PathBuf;
use std::{env, process};
use tokio::runtime;
use tokio::sync::mpsc::channel;

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
                        println!(
                            "{}",
                            options.usage(concat!(
                                "Usage:\n    ",
                                env!("CARGO_BIN_NAME"),
                                " [Option...] [Pack Directory]"
                            ))
                        );

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
    println!("PackAdvice {}", env!("CARGO_PKG_VERSION"));
}

fn advice(directory_path: &str) -> ExitCode {
    let (sender, mut receiver) = channel::<PackAdviserStatus>(64);
    let runtime = runtime::Builder::new_current_thread().build().unwrap();
    let cli_thread = runtime.spawn(async move {
        while let Some(PackAdviserStatus { path, status_type }) = receiver.recv().await {
            match status_type {
                PackAdviserStatusType::Notice(message) => {
                    trace!("[{}] {}", path, message)
                }
                PackAdviserStatusType::Error(err) => {
                    error!("[{}] {}", path, err)
                }
            }
        }
    });
    let options = PackOptions {
        path: PathBuf::from(directory_path),
    };
    let packadviser = runtime.spawn_blocking(|| PackAdviser::new().run(options, sender));
    runtime.block_on(async {
        match packadviser.await.unwrap() {
            Ok(_) => {
                cli_thread.await.ok();
                ExitCode::Success
            }
            Err(error) => {
                error!("{}", error);
                ExitCode::from(error)
            }
        }
    })
}
