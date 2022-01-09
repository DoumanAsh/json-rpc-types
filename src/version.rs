use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use core::fmt;

///Protocol Version
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum Version {
    ///v2.0
    V2,
}

impl Default for Version {
    #[inline(always)]
    fn default() -> Self {
        Version::V2
    }
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
        des.deserialize_any(VersionVisitor)
    }
}

struct VersionVisitor;
 
impl<'a> Visitor<'a> for VersionVisitor {
    type Value = Version;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Identifier must be a string")
    }

    #[inline]
    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        match v {
            "2.0" => Ok(Version::V2),
            _ => Err(serde::de::Error::custom("Invalid version. Allowed: 2.0")),
        }
    }
}
