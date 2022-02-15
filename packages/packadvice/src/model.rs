use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;
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

#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    #[error("Json error: {0}")]
    JsonError(#[from] serde_json::Error),
}
