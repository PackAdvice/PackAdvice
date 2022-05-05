use crate::pack_path;
use async_recursion::async_recursion;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use tokio::fs::ReadDir;
use tokio::{fs, io};

pub struct Model {
    pub path: String,
    pub parent: Option<String>,
    pub textures: HashMap<String, String>,
    pub elements: Vec<Element>,
    pub overrides: Vec<Override>,
}

#[derive(Clone, Default)]
pub struct Element {
    pub faces: HashMap<String, Face>,
}

#[derive(Clone)]
pub struct Face {
    pub texture: Option<String>,
}

pub struct Override {
    pub predicate: Predicate,
    pub model: Option<String>,
}

pub struct Predicate {
    pub custom_model_data: Option<i64>,
}

impl Model {
    pub async fn new<P: AsRef<Path>>(file_path: P, path: String) -> Result<Self, Error> {
        let bytes = fs::read(file_path.as_ref()).await?;
        let mut parent = None;
        let mut textures = HashMap::new();
        let mut elements = Vec::new();
        let mut overrides = Vec::new();
        if let Value::Object(root_object) = serde_json::from_slice(&*bytes)? {
            parent = root_object
                .get("parent")
                .and_then(Value::as_str)
                .map(|s| pack_path!(s));
            if let Some(Value::Object(textures_values)) = root_object.get("textures") {
                for (key, value) in textures_values {
                    if value.is_string() {
                        textures.insert(key.to_string(), value.as_str().unwrap().to_string());
                    }
                }
            }
            if let Some(Value::Array(elements_value)) = root_object.get("elements") {
                for value in elements_value {
                    if let Some(element_object) = value.as_object() {
                        let mut faces = HashMap::new();
                        if let Some(Value::Object(faces_value)) = element_object.get("faces") {
                            for (key, value) in faces_value {
                                if let Some(face_object) = value.as_object() {
                                    let texture = face_object
                                        .get("texture")
                                        .and_then(Value::as_str)
                                        .map(|s| s.to_string());
                                    faces.insert(key.to_string(), Face { texture });
                                }
                            }
                        }
                        elements.push(Element { faces })
                    }
                }
            }
            if let Some(Value::Array(overrides_value)) = root_object.get("overrides") {
                for value in overrides_value {
                    if let Some(override_value) = value.as_object() {
                        let predicate = if let Some(Value::Object(predicate_value)) =
                            override_value.get("predicate")
                        {
                            Predicate {
                                custom_model_data: predicate_value
                                    .get("custom_model_data")
                                    .and_then(Value::as_i64),
                            }
                        } else {
                            Predicate {
                                custom_model_data: None,
                            }
                        };
                        let model = override_value
                            .get("model")
                            .and_then(Value::as_str)
                            .map(|s| pack_path!(s));
                        overrides.push(Override { predicate, model })
                    }
                }
            }
        }
        Ok(Model {
            path,
            parent,
            textures,
            elements,
            overrides,
        })
    }
}

pub async fn get_models<P: AsRef<Path>>(path: P) -> Vec<Model> {
    let mut models: Vec<Model> = Vec::new();
    if let Ok(directory) = fs::read_dir(path).await {
        get_models_recursion(directory, Vec::new(), &mut models).await;
    }
    models
}

#[async_recursion]
async fn get_models_recursion(
    mut directory: ReadDir,
    last_path: Vec<String>,
    models: &mut Vec<Model>,
) {
    while let Some(child) = directory.next_entry().await.unwrap() {
        if let Ok(child_meta) = child.metadata().await {
            let file_name = child.file_name().to_str().unwrap().to_string();
            let mut path = Vec::new();
            path.extend_from_slice(&last_path);
            path.push(file_name);
            if child_meta.is_dir() {
                if let Ok(child_dir) = fs::read_dir(child.path()).await {
                    get_models_recursion(child_dir, path, models).await
                }
            } else if let Some(extension) = child.path().extension() {
                if extension.eq_ignore_ascii_case("json") {
                    let join_path = path.join("/");
                    if let Ok(model) = Model::new(
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
