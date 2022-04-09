use crate::feature::elements_counter::ModelElementsCounter;
use crate::feature::unreferenced_model::UnreferencedModelChecker;
use crate::result::PackResultExportError::{NoFileType, UnsupportedFileType};
use crate::{MissingTextureChecker, Pack, UnreferencedTextureChecker};
use std::ffi::OsStr;
use std::path::Path;
use tokio::fs::File;
use tokio::io;
use tokio::io::AsyncWriteExt;

pub struct PackResult {
    pub pack: Pack,
    pub unreferenced_texture_checker: UnreferencedTextureChecker,
    pub unreferenced_model_checker: UnreferencedModelChecker,
    pub missing_texture_checker: MissingTextureChecker,
    pub model_elements_counter: ModelElementsCounter,
}

impl PackResult {
    pub async fn export<P: AsRef<Path>>(&self, path: P) -> Result<(), PackResultExportError> {
        match path.as_ref().extension().and_then(OsStr::to_str) {
            Some("md") => {
                let mut file = File::create(path).await?;
                file.write(
                    format!(
                        "# Pack meta\n\
                        | Pack format | Minecraft version |\n\
                        |---|---|\n\
                        | {} | {} |\n\n",
                        self.pack.pack_meta.pack_format,
                        self.pack.pack_meta.minecraft_version()
                    )
                    .as_ref(),
                )
                .await?;
                if !self.unreferenced_texture_checker.textures.is_empty() {
                    file.write(
                        b"# Unused textures\n\
                        The following files are not used in any model or font. \
                        Please consider these as they may be removed. (However, it does not yet support vanilla textures.)\n\
                        <details>\n\
                        <summary>List</summary>\n\n",
                    )
                    .await?;
                    for texture in &self.unreferenced_texture_checker.textures {
                        file.write(format!(" - `{}`\n", texture).as_ref()).await?;
                    }
                    file.write(b"</details>\n\n").await?;
                }
                if !self.unreferenced_model_checker.models.is_empty() {
                    file.write(
                        b"# Unreferenced models\n\
                        The following files are not used in any model(parent, overrides) or blockstate. \
                        Please consider these as they may be removed. (However, it does not yet support vanilla models.)\n\
                        <details>\n\
                        <summary>List</summary>\n\n",
                    )
                    .await?;
                    for texture in &self.unreferenced_model_checker.models {
                        file.write(format!(" - `{}`\n", texture).as_ref()).await?;
                    }
                    file.write(b"</details>\n\n").await?;
                }
                if !self.missing_texture_checker.models.is_empty() {
                    file.write(
                        b"# Models that contain #missing texture\n\
                        The following files have cubes that have not been assigned textures.\n\
                        <details>\n\
                        <summary>List</summary>\n\n",
                    )
                    .await?;
                    for model in &self.missing_texture_checker.models {
                        file.write(format!(" - `{}`\n", model).as_ref()).await?;
                    }
                    file.write(b"</details>\n\n").await?;
                }
                if !self.model_elements_counter.models.is_empty() {
                    file.write(
                        format!(
                            "# List of model and element amount\n\
                            Too many elements will affect rendering.\n\
                            <details>\n\
                            <summary>List</summary>\n\n\
                            | Model({}) | Elements({}) |\n\
                            |---|---|\n",
                            self.model_elements_counter.models.len(),
                            self.model_elements_counter.total
                        )
                        .as_ref(),
                    )
                    .await?;
                    for (model, size) in &self.model_elements_counter.models {
                        file.write(format!("| `{}` | {} |\n", model, size).as_ref())
                            .await?;
                    }
                    file.write(b"</details>\n\n").await?;
                }
                Ok(())
            }
            Some(extension) => Err(UnsupportedFileType(extension.parse().unwrap())),
            None => Err(NoFileType),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PackResultExportError {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    #[error("Unsupported file type: {0}")]
    UnsupportedFileType(String),

    #[error("No file type")]
    NoFileType,
}
