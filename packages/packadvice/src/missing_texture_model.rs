use crate::Pack;
use std::collections::HashSet;

pub struct MissingTextureChecker {
    /// Models with #missing in texture
    pub models: Vec<String>,
}

impl MissingTextureChecker {
    pub fn new(pack: &Pack) -> Self {
        let mut _models = HashSet::new();
        for namespace in &pack.namespaces {
            for model in &namespace.models {
                for element in &model.elements {
                    for face in element.faces.values() {
                        if let Some(texture) = &face.texture {
                            if texture == "#missing" {
                                _models.insert(format!("{}:{}", namespace.name, model.pack_path));
                            }
                        }
                    }
                }
            }
        }
        let mut models = Vec::from_iter(_models);
        models.sort();
        MissingTextureChecker { models }
    }
}
