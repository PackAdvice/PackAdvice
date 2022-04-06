use crate::minecraft_path;
use async_recursion::async_recursion;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use tokio::fs::ReadDir;
use tokio::{fs, io};

pub struct BlockState {
    pub path: String,
    pub variants: HashMap<String, Variant>,
}

pub struct Variant {
    pub model: Option<String>,
}

impl BlockState {
    pub async fn new<P: AsRef<Path>>(file_path: P, path: String) -> Result<Self, Error> {
        let bytes = fs::read(file_path.as_ref()).await?;
        let mut variants = HashMap::new();
        if let Value::Object(root_object) = serde_json::from_slice(&*bytes)? {
            if let Some(Value::Object(textures_values)) = root_object.get("variants") {
                for (key, value) in textures_values {
                    if let Some(variants_value) = value.as_object() {
                        let model = variants_value
                            .get("model")
                            .and_then(Value::as_str)
                            .map(|s| minecraft_path!(s));
                        variants.insert(key.as_str().to_string(), Variant { model });
                    }
                }
            }
        }
        Ok(BlockState { path, variants })
    }
}

pub async fn get_blockstates<P: AsRef<Path>>(path: P) -> Vec<BlockState> {
    let mut models: Vec<BlockState> = Vec::new();
    if let Ok(directory) = fs::read_dir(path).await {
        get_blockstates_recursion(directory, Vec::new(), &mut models).await;
    }
    models
}

#[async_recursion]
async fn get_blockstates_recursion(
    mut directory: ReadDir,
    last_path: Vec<String>,
    models: &mut Vec<BlockState>,
) {
    while let Some(child) = directory.next_entry().await.unwrap() {
        if let Ok(child_meta) = child.metadata().await {
            let file_name = child.file_name().to_str().unwrap().to_string();
            let mut path = Vec::new();
            path.extend_from_slice(&last_path);
            path.push(file_name);
            if child_meta.is_dir() {
                if let Ok(child_dir) = fs::read_dir(child.path()).await {
                    get_blockstates_recursion(child_dir, path, models).await
                }
            } else if let Some(extension) = child.path().extension() {
                if extension.eq_ignore_ascii_case("json") {
                    let join_path = path.join("/");
                    if let Ok(model) = BlockState::new(
                        child.path(),
                        join_path.split_at(join_path.len() - 5).0.parse().unwrap(),
                    )
                    .await
                    {
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
