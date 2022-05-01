mod feature;
pub mod pack;
pub mod result;
mod util;

use crate::feature::elements_counter::ModelElementsCounter;
use crate::feature::missing_texture_model::MissingTextureChecker;
use crate::feature::unreferenced_model::UnreferencedModelChecker;
use crate::feature::unreferenced_texture::UnreferencedTextureChecker;
use crate::pack::Pack;
use crate::result::PackResult;
use std::path::PathBuf;
use tokio::runtime::Runtime;
use tokio::{fs, io};

#[derive(Default)]
pub struct PackAdviser;

impl PackAdviser {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self, options: PackOptions) -> Result<PackResult, PackAdviserError> {
        let runtime = Runtime::new().unwrap();
        runtime.block_on(async {
            let _ = fs::read_dir(options.path.as_path()).await?; // Check the pack directory exists
            let pack = Pack::new(options.path.as_path()).await?;
            let unreferenced_texture_checker = UnreferencedTextureChecker::new(&pack);
            let unreferenced_model_checker = UnreferencedModelChecker::new(&pack);
            let missing_texture_checker = MissingTextureChecker::new(&pack);
            let model_elements_counter = ModelElementsCounter::new(&pack);
            Ok(PackResult {
                pack,
                unreferenced_texture_checker,
                unreferenced_model_checker,
                missing_texture_checker,
                model_elements_counter,
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
