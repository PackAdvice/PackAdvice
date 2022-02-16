use crate::Pack;
use std::collections::HashSet;

pub struct UnusedTextureChecker {
    pub unused_textures: HashSet<String>,
}

impl UnusedTextureChecker {
    pub fn new(pack: &Pack) -> Self {
        let mut unused_textures = HashSet::new();
        for namespace in &pack.namespaces {
            for texture in &namespace.textures {
                unused_textures.insert(format!("{}:{}", namespace.name, texture.path));
            }
        }
        for namespace in &pack.namespaces {
            for model in &namespace.models {
                for texture in model.textures.values() {
                    unused_textures
                        .retain(|t| t.as_str() != format!("{}:{}", namespace.name, texture));
                }
            }
        }
        UnusedTextureChecker { unused_textures }
    }
}
