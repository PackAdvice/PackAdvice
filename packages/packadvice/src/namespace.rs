use crate::model::{get_models, Model};
use crate::texture::{get_textures, Texture};
use std::path::{Path, PathBuf};
use tokio::fs::ReadDir;
use crate::font::{Font, get_fonts};

pub struct Namespace {
    pub name: String,
    pub path: PathBuf,
    pub fonts: Vec<Font>,
    pub models: Vec<Model>,
    pub textures: Vec<Texture>,
}

impl Namespace {
    pub async fn new<P: AsRef<Path>>(path: P) -> Self {
        Namespace {
            name: path.as_ref().file_name().unwrap().to_str().unwrap().to_string(),
            path: path.as_ref().to_path_buf(),
            fonts: get_fonts(path.as_ref().join("font")).await,
            models: get_models(path.as_ref().join("models")).await,
            textures: get_textures(path.as_ref().join("textures")).await,
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
