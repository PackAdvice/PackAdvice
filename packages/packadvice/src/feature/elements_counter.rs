use crate::{pack_path, Pack};
use std::collections::HashSet;

pub struct ModelElementsCounter {
    /// List of models and number of elements
    pub models: Vec<(String, usize)>,
}

impl ModelElementsCounter {
    pub fn new(pack: &Pack) -> Self {
        let mut models = HashSet::new();
        for namespace in &pack.namespaces {
            for model in &namespace.models {
                models.insert((pack_path!(namespace.name, model.path), model.elements.len()));
            }
        }
        let mut models = Vec::from_iter(models);
        models.sort_by(|(_, size_a), (_, size_b)| size_b.cmp(size_a));
        ModelElementsCounter { models }
    }
}
