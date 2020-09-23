use serde::{Deserialize, Deserializer, Serialize, Serializer};

///Protocol Version
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum Version {
    ///v2.0
    V2,
}

impl Serialize for Version {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        match *self {
            Version::V2 => ser.serialize_str("2.0"),
        }
    }
}

impl<'a> Deserialize<'a> for Version {
    fn deserialize<D: Deserializer<'a>>(des: D) -> Result<Self, D::Error> {
        let text: &'a str = Deserialize::deserialize(des)?;
        match text {
            "2.0" => Ok(Version::V2),
            _ => Err(serde::de::Error::custom("Invalid version. Allowed: 2.0")),
        }
    }
}
