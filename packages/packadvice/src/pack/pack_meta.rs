use serde_json::Value;
use std::fmt;
use std::path::Path;
use tokio::{fs, io};

#[derive(Default)]
pub struct PackMeta {
    pub pack_format: i32,
}

impl PackMeta {
    pub async fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        const PACK_FORMAT_MUST_BE_INTEGER: &str = "\"pack_format\" must be an integer";
        const MISSING_PACK_KEY: &str = "Missing \"pack\" key in root object";
        const MISSING_PACK_FORMAT_KEY: &str = "Missing \"pack_format\" key in pack metadata object";
        const PACK_MUST_BE_OBJECT: &str = "\"pack\" key value must be a JSON object";
        const JSON_MUST_BE_OBJECT: &str = "JSON value is not an object";

        let bytes = fs::read(path).await?;
        return match serde_json::from_slice(&*bytes)? {
            Value::Object(root_object) => {
                match root_object
                    .get("pack")
                    .ok_or(Error::SyntaxError(MISSING_PACK_KEY))?
                {
                    Value::Object(pack_meta_object) => {
                        match pack_meta_object
                            .get("pack_format")
                            .ok_or(Error::SyntaxError(MISSING_PACK_FORMAT_KEY))?
                        {
                            Value::Number(pack_format_version_number) => {
                                let pack_format = i32::try_from(
                                    pack_format_version_number
                                        .as_i64()
                                        .ok_or(Error::SyntaxError(PACK_FORMAT_MUST_BE_INTEGER))?,
                                )
                                .map_err(|_| Error::SyntaxError(PACK_FORMAT_MUST_BE_INTEGER))?;
                                Ok(Self { pack_format })
                            }
                            _ => Err(Error::SyntaxError(PACK_FORMAT_MUST_BE_INTEGER)),
                        }
                    }
                    _ => Err(Error::SyntaxError(PACK_MUST_BE_OBJECT)),
                }
            }
            _ => Err(Error::SyntaxError(JSON_MUST_BE_OBJECT)),
        };
    }

    /// pack_format -> minecraft version
    ///
    /// https://minecraft.fandom.com/wiki/Pack_format
    pub fn minecraft_version(&self) -> PackMinecraftVersion {
        match self.pack_format {
            1 => PackMinecraftVersion::Versions {
                from: "1.6.1",
                to: "1.8.9",
            },
            2 => PackMinecraftVersion::Versions {
                from: "1.9",
                to: "1.10.2",
            },
            3 => PackMinecraftVersion::Versions {
                from: "1.11",
                to: "1.12.2",
            },
            4 => PackMinecraftVersion::Versions {
                from: "1.13",
                to: "1.14.4",
            },
            5 => PackMinecraftVersion::Versions {
                from: "1.15",
                to: "1.16.1",
            },
            6 => PackMinecraftVersion::Versions {
                from: "1.16.2",
                to: "1.16.5",
            },
            7 => PackMinecraftVersion::Version("1.17"),
            8 => PackMinecraftVersion::Version("1.18"),
            9 => PackMinecraftVersion::Version("1.19"),
            _ => PackMinecraftVersion::Unknown,
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    #[error("Json error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Syntax error: {0}")]
    SyntaxError(&'static str),
}

pub enum PackMinecraftVersion<'a> {
    Versions { from: &'a str, to: &'a str },
    Version(&'a str),
    Unknown,
}

impl fmt::Display for PackMinecraftVersion<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PackMinecraftVersion::Versions { from, to } => {
                write!(f, "{}-{}", from, to)
            }
            PackMinecraftVersion::Version(version) => {
                write!(f, "{}", version)
            }
            PackMinecraftVersion::Unknown => {
                write!(f, "unknown")
            }
        }
    }
}
