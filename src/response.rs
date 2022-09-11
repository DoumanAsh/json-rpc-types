use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::Error;
use crate::version::Version;
use crate::id::Id;
use crate::utils::Key;

///Response representation.
///
///When omitting `id`, it shall be serialized as `null` and means you're unable to identify `id` of
///`Request`.
///Note that JSON-RPCv2 specifies that `id` must be always present, therefore you're encouraged to
///treat missing `id` as error, unless response is error itself, in which case it might be
///indication that server treats request as invalid (e.g. unable to parse request's id).
///
///`jsonrpc` may be omitted during deserialization and defaults to v2.
///
///Type parameters:
///
///- `R`  - Type of payload for successful response
///- `E`  - Type of optional data for `Error`.
///- `EM` - Type of `E::M`, which is used for `message` field of error.
#[derive(Clone, Debug, PartialEq)]
pub struct Response<R, E, EM=crate::error::StrBuf> {
    ///A String specifying the version of the JSON-RPC protocol.
    pub jsonrpc: Version,

    ///Content of response, depending on whether it is success or failure.
    pub payload: Result<R, Error<E, EM>>,

    ///An identifier established by the Client.
    ///
    ///If not present, it is sent in response to invalid request (e.g. unable to recognize id).
    ///
    ///Must be present always, so `None` is serialized as `null`
    pub id: Option<Id>,
}

impl<R: Serialize, E: Serialize, EM: Serialize> Serialize for Response<R, E, EM> {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;

        let mut state = ser.serialize_map(Some(3))?;

        state.serialize_entry("jsonrpc", &self.jsonrpc)?;
        match self.payload {
            Ok(ref result) => state.serialize_entry("result", result),
            Err(ref error) => state.serialize_entry("error", error),
        }?;
        match self.id {
            Some(ref id) => state.serialize_entry("id", id),
            None => state.serialize_entry("id", &()),
        }?;

        state.end()
    }
}

impl<'de, R: Deserialize<'de>, E: Deserialize<'de>, EM: Deserialize<'de>> Deserialize<'de> for Response<R, E, EM> {
    fn deserialize<D: Deserializer<'de>>(der: D) -> Result<Self, D::Error> {
        use core::marker::PhantomData;
        use serde::de::{self, Visitor};

        struct MapVisit<R, E, EM>(PhantomData<(R, E, EM)>);

        impl<'de, R: Deserialize<'de>, E: Deserialize<'de>, EM: Deserialize<'de>> Visitor<'de> for MapVisit<R, E, EM> {
            type Value = Response<R, E, EM>;

            #[inline]
            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("Object resembling JSON-RPC response type")
            }

            fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                //Normally you'd use unitialized struct, but it is highly unlikely to guarantee
                //safety of field-by-field initialization
                let mut version = None;
                let mut result = None;
                let mut id = None;

                while let Some(key) = map.next_key::<Key>()? {
                    match key {
                        Key::JsonRpc => {
                            version = Some(map.next_value::<Version>()?);
                        },
                        Key::Result => if result.is_none() {
                            result = Some(Ok(map.next_value::<R>()?));
                        } else {
                            return Err(serde::de::Error::custom("JSON-RPC Response contains both result and error field"));
                        },
                        Key::Error => if result.is_none() {
                            result = Some(Err(map.next_value::<Error<E, EM>>()?));
                        } else {
                            return Err(serde::de::Error::custom("JSON-RPC Response contains both error and result field"));
                        },
                        Key::Id => {
                            id = map.next_value::<Option<Id>>()?;
                        },
                    }
                }

                Ok(Self::Value {
                    jsonrpc: match version {
                        Some(version) => version,
                        None => Version::V2,
                    },
                    payload: match result {
                        Some(payload) => payload,
                        None => return Err(serde::de::Error::custom("JSON-RPC Response is missing either result or error field.")),
                    },
                    id,
                })
            }
        }

        der.deserialize_map(MapVisit(PhantomData))
    }
}

impl<R, E, EM> Response<R, E, EM> {
    #[inline]
    ///Creates successful response.
    pub const fn result(jsonrpc: Version, result: R, id: Option<Id>) -> Self {
        Self {
            jsonrpc,
            payload: Ok(result),
            id,
        }
    }

    #[inline]
    ///Creates error response.
    pub const fn error(jsonrpc: Version, error: Error<E, EM>, id: Option<Id>) -> Self {
        Self {
            jsonrpc,
            payload: Err(error),
            id,
        }
    }
}
