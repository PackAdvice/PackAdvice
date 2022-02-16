use async_recursion::async_recursion;
use std::path::Path;
use tokio::fs;
use tokio::fs::ReadDir;

pub struct Texture {
    pub path: String,
}

pub async fn get_textures<P: AsRef<Path>>(path: P) -> Vec<Texture> {
    let mut textures: Vec<Texture> = Vec::new();
    if let Ok(directory) = fs::read_dir(path).await {
        get_textures_recursion(directory, Vec::new(), &mut textures).await;
    }
    textures
}

#[async_recursion]
async fn get_textures_recursion(
    mut directory: ReadDir,
    last_path: Vec<String>,
    textures: &mut Vec<Texture>,
) {
    while let Some(child) = directory.next_entry().await.unwrap() {
        if let Ok(child_meta) = child.metadata().await {
            let file_name = child.file_name().to_str().unwrap().to_string();
            let mut path = Vec::new();
            path.extend_from_slice(&last_path);
            path.push(file_name);
            if child_meta.is_dir() {
                if let Ok(child_dir) = fs::read_dir(child.path()).await {
                    get_textures_recursion(child_dir, path, textures).await
                }
            } else if let Some(extension) = child.path().extension() {
                if extension.eq_ignore_ascii_case("png") {
                    let join_path = path.join("/");
                    textures.push(Texture {
                        path: join_path.split_at(join_path.len() - 4).0.parse().unwrap(),
                    });
                }
            }
        }
    }
}
