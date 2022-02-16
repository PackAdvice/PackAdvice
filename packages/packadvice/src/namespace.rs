use crate::model::{get_models, Model};
use crate::texture::{get_textures, Texture};
use std::path::PathBuf;
use tokio::fs::ReadDir;

pub struct Namespace {
    pub name: String,
    pub path: PathBuf,
    pub models: Vec<Model>,
    pub textures: Vec<Texture>,
}

impl Namespace {
    pub async fn new(path: PathBuf) -> Self {
        Namespace {
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            path: path.to_path_buf(),
            models: get_models(path.join("models")).await,
            textures: get_textures(path.join("textures")).await,
        }
    }
}

pub async fn get_namespaces(mut assets: ReadDir) -> Vec<Namespace> {
    let mut namespaces = Vec::new();
    while let Some(directory) = assets.next_entry().await.unwrap() {
        if let Ok(child_meta) = directory.metadata().await {
            if child_meta.is_dir() {
                namespaces.push(Namespace::new(directory.path()).await)
            }
        }
    }
    namespaces
}
