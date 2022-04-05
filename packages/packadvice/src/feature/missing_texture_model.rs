use crate::{minecraft_path, Pack};
use std::collections::HashSet;

pub struct MissingTextureChecker {
    /// Models that contain #missing texture
    pub models: Vec<String>,
}

impl MissingTextureChecker {
    pub fn new(pack: &Pack) -> Self {
        let mut models = HashSet::new();
        for namespace in &pack.namespaces {
            for model in &namespace.models {
                for element in &model.elements {
                    for face in element.faces.values() {
                        if let Some(texture) = &face.texture {
                            if texture == "#missing" {
                                models.insert(minecraft_path!(namespace.name, model.path));
                            }
                        }
                    }
                }
            }
        }
        let mut models = Vec::from_iter(models);
        models.sort();
        MissingTextureChecker { models }
    }
}
