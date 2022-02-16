use crate::namespace::{get_namespaces, Namespace};
use crate::{pack_meta, PackMeta};
use std::path::Path;
use thiserror::Error;
use tokio::fs;

pub struct Pack {
    pub pack_meta: PackMeta,
    pub namespaces: Vec<Namespace>,
}

impl Pack {
    pub async fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let pack_meta = PackMeta::new(path.as_ref().join("pack.mcmeta")).await?;
        let namespaces: Vec<Namespace> =
            if let Ok(assets) = fs::read_dir(path.as_ref().join("assets")).await {
                get_namespaces(assets).await
            } else {
                Vec::new()
            };
        Ok(Pack {
            pack_meta,
            namespaces,
        })
    }
}

#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    #[error("[pack.mcmeta] {0}")]
    PackMetaError(#[from] pack_meta::Error),
}
