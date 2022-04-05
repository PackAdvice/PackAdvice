mod feature;
mod pack;
mod result;

use crate::pack::{Pack, pack_meta};
use crate::result::PackResult;
use std::path::PathBuf;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::Sender;
use tokio::{fs, io};
use crate::feature::missing_texture_model::MissingTextureChecker;
use crate::feature::unreferenced_texture::UnreferencedTextureChecker;

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
    ) -> Result<PackResult, PackAdviserError> {
        let runtime = Runtime::new().unwrap();
        runtime.block_on(async {
            // Check the pack directory exists
            let _ = fs::read_dir(options.path.as_path()).await?;

            let pack = Pack::new(options.path.as_path()).await?;

            // Check pack.mcmeta
            status_sender
                .send(PackAdviserStatus {
                    path: options.path.display().to_string(),
                    status_type: PackAdviserStatusType::Notice(format!(
                        "pack_format: {} ({})",
                        pack.pack_meta.pack_format,
                        pack.pack_meta.minecraft_version()
                    )),
                })
                .await
                .ok();

            // Check unused textures
            let unreferenced_texture_checker = UnreferencedTextureChecker::new(&pack);
            for texture in &unreferenced_texture_checker.textures {
                status_sender
                    .send(PackAdviserStatus {
                        path: texture.to_string(),
                        status_type: PackAdviserStatusType::Warn(
                            "Unused texture in model".to_string(),
                        ),
                    })
                    .await
                    .ok();
            }

            // Check models with #missing in texture
            let missing_texture_checker = MissingTextureChecker::new(&pack);
            for missing_texture_model in &missing_texture_checker.models {
                status_sender
                    .send(PackAdviserStatus {
                        path: missing_texture_model.to_string(),
                        status_type: PackAdviserStatusType::Warn(
                            "Textures contain #missing".to_string(),
                        ),
                    })
                    .await
                    .ok();
            }

            Ok(PackResult {
                pack,
                unreferenced_texture_checker,
                missing_texture_checker,
            })
        })
    }
}

pub struct PackOptions {
    /// Pack directory path
    pub path: PathBuf,
}

#[derive(thiserror::Error, Debug)]
pub enum PackAdviserError {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    #[error("Pack error: {0}")]
    PackError(#[from] pack::Error),
}

pub struct PackAdviserStatus {
    pub path: String,
    pub status_type: PackAdviserStatusType,
}

pub enum PackAdviserStatusType {
    Notice(String),
    Warn(String),
    Error(PackAdviserStatusError),
}

#[derive(thiserror::Error, Debug)]
pub enum PackAdviserStatusError {
    #[error("{0}")]
    PackMetaError(pack_meta::Error),
}
