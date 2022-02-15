use crate::model::Model;
use async_recursion::async_recursion;
use std::io;
use std::path::Path;
use tokio::fs;
use tokio::fs::{DirEntry, ReadDir};

pub struct UnusedTextureChecker {
    pub unused_textures: Vec<String>,
    pub errors: Vec<Error>,
}

impl UnusedTextureChecker {
    pub async fn new<P: AsRef<Path>>(root_path: P) -> Self {
        let assets = match fs::read_dir(root_path.as_ref().join("assets")).await {
            Ok(assets) => assets,
            Err(err) => {
                return UnusedTextureChecker {
                    unused_textures: vec![],
                    errors: vec![Error::IoError(err)],
                }
            }
        };
        let mut errors = Vec::new();
        let namespaces = get_namespaces(assets, &mut errors).await;
        let mut unused_textures = Vec::new();

        // Collect all textures
        for namespace in &namespaces {
            match fs::read_dir(namespace.path().join("textures")).await {
                Ok(textures) => {
                    get_textures(
                        textures,
                        namespace.file_name().to_str().unwrap(),
                        vec![],
                        &mut unused_textures,
                        &mut errors,
                    )
                    .await
                }
                Err(_) => continue,
            }
        }

        // Remove textures in models
        for namespace in &namespaces {
            match fs::read_dir(namespace.path().join("models")).await {
                Ok(models) => remove_textures(models, &mut unused_textures, &mut errors).await,
                Err(_) => continue,
            }
        }

        UnusedTextureChecker {
            unused_textures,
            errors,
        }
    }
}

pub enum Error {
    IoError(io::Error),
}

async fn get_namespaces(mut assets: ReadDir, errors: &mut Vec<Error>) -> Vec<DirEntry> {
    let mut namespaces = Vec::new();
    loop {
        match assets.next_entry().await {
            Ok(Some(child)) => {
                if let Ok(child_meta) = child.metadata().await {
                    if child_meta.is_dir() {
                        namespaces.push(child)
                    }
                }
            }
            Ok(None) => break,
            Err(err) => {
                errors.push(Error::IoError(err));
            }
        }
    }
    namespaces
}

#[async_recursion]
async fn get_textures(
    mut directory: ReadDir,
    namespace_name: &str,
    last_path: Vec<String>,
    textures: &mut Vec<String>,
    errors: &mut Vec<Error>,
) {
    loop {
        match directory.next_entry().await {
            Ok(Some(child)) => {
                if let Ok(child_meta) = child.metadata().await {
                    let file_name = child.file_name().to_str().unwrap().to_string();
                    let mut path = Vec::new();
                    path.extend_from_slice(&last_path);
                    path.push(file_name);
                    if child_meta.is_dir() {
                        match fs::read_dir(child.path()).await {
                            Ok(child_dir) => {
                                get_textures(child_dir, namespace_name, path, textures, errors)
                                    .await
                            }
                            Err(err) => {
                                errors.push(Error::IoError(err));
                            }
                        };
                    } else if let Some(extension) = child.path().extension() {
                        if extension.eq_ignore_ascii_case("png") {
                            let join_path = path.join("/");
                            textures.push(format!(
                                "{}:{}",
                                namespace_name,
                                join_path.split_at(join_path.len() - 4).0
                            ));
                        }
                    }
                }
            }
            Ok(None) => {
                break;
            }
            Err(err) => errors.push(Error::IoError(err)),
        }
    }
}

#[async_recursion]
async fn remove_textures(
    mut directory: ReadDir,
    textures: &mut Vec<String>,
    errors: &mut Vec<Error>,
) {
    loop {
        match directory.next_entry().await {
            Ok(Some(child)) => {
                if let Ok(child_meta) = child.metadata().await {
                    if child_meta.is_dir() {
                        match fs::read_dir(child.path()).await {
                            Ok(child_dir) => remove_textures(child_dir, textures, errors).await,
                            Err(err) => {
                                errors.push(Error::IoError(err));
                            }
                        };
                    } else if let Some(extension) = child.path().extension() {
                        if extension.eq_ignore_ascii_case("json") {
                            if let Ok(model) = Model::new(child.path()).await {
                                for (_, texture) in model.textures {
                                    textures.retain(|t| t.as_str() != texture);
                                }
                            }
                        }
                    }
                }
            }
            Ok(None) => {
                break;
            }
            Err(err) => errors.push(Error::IoError(err)),
        }
    }
}
