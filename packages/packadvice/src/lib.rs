use std::path::PathBuf;
use thiserror::Error;
use tokio::{fs, io};
use tokio::sync::mpsc::Sender;
use tokio::runtime::Runtime;

pub struct PackAdviser;

impl PackAdviser {
    pub fn new() -> Self {
        Self
    }

    pub fn run(self, options: PackOptions, status_sender: &Sender<PackAdviserStatus>) -> Result<(), PackAdviserError> {
        let runtime = Runtime::new().unwrap();
        runtime.block_on(async {
            match fs::read_dir(options.path).await {
                Ok(_) => {}
                Err(err) => {
                    return Err(PackAdviserError::IoError(err))
                }
            }

            Ok(())
        })
    }
}

pub struct PackOptions {
    /// Pack directory path
    pub path: PathBuf,
}

#[derive(Error, Debug)]
pub enum PackAdviserError {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error)
}

pub enum PackAdviserStatus {
}
