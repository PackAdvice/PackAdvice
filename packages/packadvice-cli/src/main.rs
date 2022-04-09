mod exit_code;
mod log;
mod output;

use crate::exit_code::ExitCode;
use getopts::Options;
use packadvice::{PackAdviser, PackOptions};
use std::path::PathBuf;
use std::{env, process};
use tokio::runtime;
use crate::output::cli_output;

fn main() {
    process::exit(run() as i32);
}

fn run() -> ExitCode {
    let mut options = Options::new();

    options.optflag("v", "version", "Prints version information");
    options.optmulti("o", "output", "File path to export results", "FILE(.md)");

    match options.parse(env::args().skip(1)) {
        Ok(option_matches) => {
            if option_matches.opt_present("v") {
                print_version_information();

                ExitCode::Success
            } else {
                match option_matches.free.first() {
                    Some(directory_path) => {
                        let output_paths = option_matches.opt_strs("o");
                        advice(directory_path, output_paths)
                    }
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

fn advice(directory_path: &str, output_paths: Vec<String>) -> ExitCode {
    let runtime = runtime::Builder::new_current_thread().build().unwrap();
    let options = PackOptions {
        path: PathBuf::from(directory_path),
    };
    let packadviser = runtime.spawn_blocking(|| PackAdviser::new().run(options));
    runtime.block_on(async {
        match packadviser.await.unwrap() {
            Ok(result) => {
                if output_paths.is_empty() {
                    if let Some(err) = cli_output(&result).err() {
                        error!("Output error ({})", err);
                    }
                } else {
                    for output_path in output_paths {
                        let path = PathBuf::from(&output_path);
                        match result.export(&path).await {
                            Ok(_) => success!("Export to {}", path.display()),
                            Err(err) => error!("Export to {} ({})", path.display(), err),
                        }
                    }
                }
                ExitCode::Success
            }
            Err(error) => {
                error!("{}", error);
                ExitCode::from(error)
            }
        }
    })
}
