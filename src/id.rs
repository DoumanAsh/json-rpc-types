use serde::de::{Error, Visitor};
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
        formatter
            .write_str("Identifier must be either unsigned integer or 36 character long string")
    }

    #[inline]
    fn visit_u64<E: Error>(self, id: u64) -> Result<Self::Value, E> {
        Ok(Id::Num(id))
    }

    #[inline]
    fn visit_i64<E: Error>(self, id: i64) -> Result<Self::Value, E> {
        if id < 0 {
            Err(Error::invalid_value(
                serde::de::Unexpected::Signed(id),
                &self,
            ))
        } else {
            Ok(Id::Num(id as u64))
        }
    }

    #[inline]
    fn visit_f64<E: Error>(self, id: f64) -> Result<Self::Value, E> {
        Err(Error::invalid_value(
            serde::de::Unexpected::Float(id),
            &self,
        ))
    }

    #[inline]
    fn visit_str<E: Error>(self, id: &str) -> Result<Self::Value, E> {
        if id.len() > StrBuf::capacity() {
            Err(Error::invalid_length(id.len(), &self))
        } else {
            let mut res = StrBuf::new();
            unsafe {
                res.push_str_unchecked(id);
            }
            Ok(Id::Str(res))
        }
    }
}
