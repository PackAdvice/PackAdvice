mod pack_meta;

use crate::pack_meta::PackMeta;
use std::path::PathBuf;
use thiserror::Error;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::Sender;
use tokio::{fs, io};

#[derive(Default)]
pub struct PackAdviser;

impl PackAdviser {
    pub fn new() -> Self {
        Self
    }

    pub fn run(
        &self,
        options: PackOptions,
        status_sender: Sender<PackAdviserStatus>,
    ) -> Result<(), PackAdviserError> {
        let runtime = Runtime::new().unwrap();
        runtime.block_on(async {
            // Check the pack directory exists
            let _ = fs::read_dir(options.path.as_path()).await?;

            // Check pack.mcmeta
            match PackMeta::new(options.path.as_path()).await {
                Ok(pack_meta) => {
                    status_sender
                        .send(PackAdviserStatus {
                            path: options.path.display().to_string(),
                            status_type: PackAdviserStatusType::Notice(format!(
                                "pack_format: {} ({})",
                                pack_meta.pack_format,
                                pack_meta.minecraft_version()
                            )),
                        })
                        .await
                        .ok();
                    pack_meta
                }
                Err(err) => {
                    status_sender
                        .send(PackAdviserStatus {
                            path: options.path.display().to_string(),
                            status_type: PackAdviserStatusType::Error(
                                PackAdviserStatusError::PackMetaError(err),
                            ),
                        })
                        .await
                        .ok();
                    return Ok(());
                }
            };

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
    IoError(#[from] io::Error),
}

pub struct PackAdviserStatus {
    pub path: String,
    pub status_type: PackAdviserStatusType,
}

pub enum PackAdviserStatusType {
    Notice(String),
    Error(PackAdviserStatusError),
}

#[derive(Error, Debug)]
pub enum PackAdviserStatusError {
    #[error("{0}")]
    PackMetaError(pack_meta::Error),
}
