use async_recursion::async_recursion;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;
use tokio::fs::ReadDir;
use tokio::{fs, io};

pub struct Model {
    pub textures: HashMap<String, String>,
}

impl Model {
    pub async fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let bytes = fs::read(path.as_ref()).await?;
        let mut textures = HashMap::new();
        if let Value::Object(root_object) = serde_json::from_slice(&*bytes)? {
            if let Some(Value::Object(textures_values)) = root_object.get("textures") {
                for (key, value) in textures_values {
                    if value.is_string() {
                        textures.insert(key.to_string(), value.as_str().unwrap().to_string());
                    }
                }
            }
        }
        Ok(Model { textures })
    }
}

pub async fn get_models<P: AsRef<Path>>(path: P) -> Vec<Model> {
    let mut models: Vec<Model> = Vec::new();
    if let Ok(directory) = fs::read_dir(path).await {
        get_models_recursion(directory, &mut models).await;
    }
    models
}

#[async_recursion]
async fn get_models_recursion(mut directory: ReadDir, models: &mut Vec<Model>) {
    while let Some(child) = directory.next_entry().await.unwrap() {
        if let Ok(child_meta) = child.metadata().await {
            if child_meta.is_dir() {
                if let Ok(child_dir) = fs::read_dir(child.path()).await {
                    get_models_recursion(child_dir, models).await
                }
            } else if let Some(extension) = child.path().extension() {
                if extension.eq_ignore_ascii_case("json") {
                    if let Ok(model) = Model::new(child.path()).await {
                        models.push(model)
                    }
                }
            }
        }
    }
}

#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    #[error("Json error: {0}")]
    JsonError(#[from] serde_json::Error),
}
