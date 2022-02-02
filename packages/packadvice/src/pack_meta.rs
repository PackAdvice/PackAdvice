pub struct PackMeta {
    pack_format: i32,
}

impl PackMeta {
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
            _ => PackMinecraftVersion::Unknown,
        }
    }
}

pub enum PackMinecraftVersion<'a> {
    Versions { from: &'a str, to: &'a str },
    Version(&'a str),
    Unknown,
}
