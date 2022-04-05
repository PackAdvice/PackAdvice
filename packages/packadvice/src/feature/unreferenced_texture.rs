use crate::Pack;
use std::collections::HashSet;
use std::path::Path;

pub struct UnreferencedTextureChecker {
    /// Textures not used in models or fonts
    pub textures: Vec<String>,
}

impl UnreferencedTextureChecker {
    pub fn new(pack: &Pack) -> Self {
        let mut textures = HashSet::new();
        for namespace in &pack.namespaces {
            for texture in &namespace.textures {
                textures.insert(format!("{}:{}", namespace.name, texture.path));
            }
        }
        for namespace in &pack.namespaces {
            for font in &namespace.fonts {
                for provider in &font.providers {
                    if let Some(file) = &provider.file {
                        let texture = if file.contains(':') {
                            format!("{}", Path::new(file.as_str()).with_extension("").display())
                        } else {
                            format!(
                                "minecraft:{}",
                                Path::new(file.as_str()).with_extension("").display()
                            )
                        };
                        textures.retain(|t| t.as_str() != texture);
                    }
                }
            }
            for model in &namespace.models {
                for value in model.textures.values() {
                    let texture = if value.contains(':') {
                        value.to_string()
                    } else {
                        format!("minecraft:{}", value)
                    };
                    textures.retain(|t| t.as_str() != texture);
                }
            }
        }
        let mut textures = Vec::from_iter(textures);
        textures.sort();
        UnreferencedTextureChecker { textures }
    }
}