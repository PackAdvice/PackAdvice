#[cfg(test)]
mod test {
    use crate::pack::model::Model;
    use crate::pack::namespace::Namespace;
    use crate::{ModelElementsCounter, Pack};

    #[test]
    fn empty_pack() {
        let pack = Default::default();
        let counter = ModelElementsCounter {
            total: 0,
            models: vec![],
        };
        assert_eq!(counter, ModelElementsCounter::new(&pack))
    }

    #[test]
    fn works() {
        let pack = Pack {
            pack_meta: Default::default(),
            namespaces: vec![
                namespace(
                    "namespace_a",
                    vec![model("model_a", 2), model("model_b", 3)],
                ),
                namespace(
                    "namespace_b",
                    vec![model("model_c", 0), model("model_d", 5)],
                ),
            ],
        };
        let counter = ModelElementsCounter {
            total: 10,
            models: vec![
                ("namespace_b:model_d".to_string(), 5),
                ("namespace_a:model_b".to_string(), 3),
                ("namespace_a:model_a".to_string(), 2),
                ("namespace_b:model_c".to_string(), 0),
            ],
        };
        assert_eq!(counter, ModelElementsCounter::new(&pack))
    }

    fn namespace(name: &str, models: Vec<Model>) -> Namespace {
        Namespace {
            name: name.to_string(),
            path: Default::default(),
            blockstates: vec![],
            fonts: vec![],
            models,
            textures: vec![],
        }
    }

    fn model(path: &str, elements_len: usize) -> Model {
        Model {
            path: path.to_string(),
            parent: None,
            textures: Default::default(),
            elements: vec![Default::default(); elements_len],
            overrides: vec![],
        }
    }
}
