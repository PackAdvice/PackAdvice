use crate::result::PackResultExportError::{NoFileType, UnsupportedFileType};
use crate::{MissingTextureChecker, PackMeta, UnusedTextureChecker};
use std::ffi::OsStr;
use std::path::Path;
use tokio::fs::File;
use tokio::io;
use tokio::io::AsyncWriteExt;

pub struct PackResult {
    pub pack_meta: PackMeta,
    pub unused_texture_checker: UnusedTextureChecker,
    pub missing_texture_checker: MissingTextureChecker,
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
                        self.pack_meta.pack_format,
                        self.pack_meta.minecraft_version()
                    )
                    .as_ref(),
                )
                .await?;
                if !self.unused_texture_checker.unused_textures.is_empty() {
                    file.write(
                        b"# Unused textures\n\
                        <details>\n\
                        <summary>List</summary>\n\n",
                    )
                    .await?;
                    for texture in &self.unused_texture_checker.unused_textures {
                        file.write(format!(" - `{}`\n", texture).as_ref()).await?;
                    }
                    file.write(b"</details>\n\n").await?;
                }
                if !self.missing_texture_checker.models.is_empty() {
                    file.write(
                        b"# Models with #missing texture\n\
                        <details>\n\
                        <summary>List</summary>\n\n",
                    )
                    .await?;
                    for model in &self.missing_texture_checker.models {
                        file.write(format!(" - `{}`\n", model).as_ref()).await?;
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
