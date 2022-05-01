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
                Namespace {
                    name: "namespace_a".to_string(),
                    path: Default::default(),
                    blockstates: vec![],
                    fonts: vec![],
                    models: vec![
                        Model {
                            path: "model_a".to_string(),
                            parent: None,
                            textures: Default::default(),
                            elements: vec![Default::default(), Default::default()],
                            overrides: vec![],
                        },
                        Model {
                            path: "model_b".to_string(),
                            parent: None,
                            textures: Default::default(),
                            elements: vec![
                                Default::default(),
                                Default::default(),
                                Default::default(),
                            ],
                            overrides: vec![],
                        },
                    ],
                    textures: vec![],
                },
                Namespace {
                    name: "namespace_b".to_string(),
                    path: Default::default(),
                    blockstates: vec![],
                    fonts: vec![],
                    models: vec![
                        Model {
                            path: "model_c".to_string(),
                            parent: None,
                            textures: Default::default(),
                            elements: vec![],
                            overrides: vec![],
                        },
                        Model {
                            path: "model_d".to_string(),
                            parent: None,
                            textures: Default::default(),
                            elements: vec![
                                Default::default(),
                                Default::default(),
                                Default::default(),
                                Default::default(),
                                Default::default(),
                            ],
                            overrides: vec![],
                        },
                    ],
                    textures: vec![],
                },
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
}
