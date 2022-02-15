use std::path::Path;
use async_recursion::async_recursion;
use serde_json::Value;
use tokio::{fs, io};
use tokio::fs::ReadDir;

pub struct Font {
    pub providers: Vec<Provider>
}

pub struct Provider {
    pub file: Option<String>
}

impl Font {
    pub async fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let bytes = fs::read(path.as_ref()).await?;
        let mut providers = Vec::new();
        if let Value::Object(root_object) = serde_json::from_slice(&*bytes)? {
            if let Some(Value::Array(providers_values)) = root_object.get("providers") {
                for provider in providers_values {
                    if provider.is_object() {
                        let file = if let Some(file) = provider.get("file") {
                            if file.is_string() {
                                Some(file.as_str().unwrap().to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                        providers.push(Provider {
                            file
                        })
                    }
                }
            }
        }
        Ok(Font { providers })
    }
}

pub async fn get_fonts<P: AsRef<Path>>(path: P) -> Vec<Font> {
    let mut fonts: Vec<Font> = Vec::new();
    if let Ok(directory) = fs::read_dir(path).await {
        get_fonts_recursion(directory, &mut fonts).await;
    }
    fonts
}

#[async_recursion]
async fn get_fonts_recursion(mut directory: ReadDir, models: &mut Vec<Font>) {
    while let Some(child) = directory.next_entry().await.unwrap() {
        if let Ok(child_meta) = child.metadata().await {
            if child_meta.is_dir() {
                if let Ok(child_dir) = fs::read_dir(child.path()).await {
                    get_fonts_recursion(child_dir, models).await
                }
            } else if let Some(extension) = child.path().extension() {
                if extension.eq_ignore_ascii_case("json") {
                    if let Ok(model) = Font::new(child.path()).await {
                        models.push(model)
                    }
                }
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    #[error("Json error: {0}")]
    JsonError(#[from] serde_json::Error),
}
