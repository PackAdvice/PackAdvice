use crate::Pack;
use std::collections::HashSet;

pub struct UnreferencedModelChecker {
    /// Models not used in parent or overrides
    pub models: Vec<String>,
}

impl UnreferencedModelChecker {
    pub fn new(pack: &Pack) -> Self {
        let mut models = HashSet::new();
        for namespace in &pack.namespaces {
            for model in &namespace.models {
                models.insert(format!("{}:{}", namespace.name, model.pack_path));
            }
        }
        for namespace in &pack.namespaces {
            for model in &namespace.models {
                if let Some(parent) = &model.parent {
                    models.remove(parent);
                }
                for override_ in &model.overrides {
                    if let Some(override_model) = &override_.model {
                        models.remove(override_model);
                    }
                }
            }
        }
        let mut models = Vec::from_iter(models);
        models.sort();
        UnreferencedModelChecker { models }
    }
}
