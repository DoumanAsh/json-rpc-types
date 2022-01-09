use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

type StrBuf = str_buf::StrBuf<36>;

use core::fmt;

///Request identfier
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum Id {
    /// Numeric id
    Num(u64),
    /// String id, maximum 36 characters which works for UUID
    Str(StrBuf),
}

impl Serialize for Id {
    #[inline]
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        match self {
            Id::Num(id) => ser.serialize_u64(*id),
            Id::Str(id) => ser.serialize_str(id.as_str()),
        }
    }
}

impl<'a> Deserialize<'a> for Id {
    #[inline]
    fn deserialize<D: Deserializer<'a>>(des: D) -> Result<Self, D::Error> {
        des.deserialize_any(IdVisitor)
    }
}

struct IdVisitor;

impl<'a> Visitor<'a> for IdVisitor {
    type Value = Id;

    #[inline]
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Identifier must be a String or Number")
    }

    #[inline]
    fn visit_u64<E: Error>(self, id: u64) -> Result<Self::Value, E> {
        Ok(Id::Num(id))
    }

    #[inline]
    fn visit_str<E: Error>(self, id: &str) -> Result<Self::Value, E> {
        if id.len() > StrBuf::capacity() {
            Err(Error::custom(format_args!("Textual identifier cannot exceed {} bytes", StrBuf::capacity())))
        } else {
            let mut res = StrBuf::new();
            unsafe {
                res.push_str_unchecked(id);
            }
            Ok(Id::Str(res))
        }
    }

    fn visit_map<E>(self, mut map: E) -> Result<Self::Value, E::Error>
    where
        E: MapAccess<'a>,
    {
        let entry: StrBuf = map.next_value()?;
        self.visit_str(entry.as_str())
    }
}

