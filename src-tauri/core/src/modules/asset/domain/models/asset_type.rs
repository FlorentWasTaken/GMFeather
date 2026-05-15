use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetType {
    VTF,
    VMT,
    WAV,
    MP3,
    PNG,
    JPG,
    LUA,
    Unknown,
}

impl fmt::Display for AssetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            AssetType::VTF => "VTF",
            AssetType::VMT => "VMT",
            AssetType::WAV => "WAV",
            AssetType::MP3 => "MP3",
            AssetType::PNG => "PNG",
            AssetType::JPG => "JPG",
            AssetType::LUA => "LUA",
            AssetType::Unknown => "Unknown",
        };
        write!(f, "{}", name)
    }
}
