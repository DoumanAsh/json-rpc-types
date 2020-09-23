use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::Error;
use crate::version::Version;
use crate::id::Id;

///Response representation.
///
///When omitting `id`, it shall be serialized as `null` and means you're unable to identify `id` of
///`Request`.
///But as JSON-RPCv2 specifies that `id` must be always present, deserialization fails on it being
///missed.
///
///`jsonrpc` may be omitted during deserialization and defaults to v2.
///
///Type parameters:
///
///- `R` - Type of payload for successful response
///- `E` - Type of optional data for `Error`.
#[derive(Clone, Debug, PartialEq)]
pub struct Response<R, E> {
    ///A String specifying the version of the JSON-RPC protocol.
    pub jsonrpc: Version,

    ///Content of response, depending on whether it is success or failure.
    pub payload: Result<R, Error<E>>,

    ///An identifier established by the Client.
    ///
    ///If not present, it is sent in response to invalid request (e.g. unable to recognize id).
    ///
    ///Must be present always, so `None` is serialized as `null`
    pub id: Option<Id>,
}

impl<R: Serialize, E: Serialize> Serialize for Response<R, E> {
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
            None => state.serialize_key("id"),
        }?;

        state.end()
    }
}

impl<'de, R: Deserialize<'de>, E: Deserialize<'de>> Deserialize<'de> for Response<R, E> {
    fn deserialize<D: Deserializer<'de>>(der: D) -> Result<Self, D::Error> {
        use core::marker::PhantomData;
        use serde::de::{self, Visitor};

        struct MapVisit<R, E>(PhantomData<Result<R, E>>);

        impl<'de, R: Deserialize<'de>, E: Deserialize<'de>> Visitor<'de> for MapVisit<R, E> {
            type Value = Response<R, E>;

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
                let mut id_set = false;

                while let Some(key) = map.next_key::<&'de str>()? {
                    match key {
                        "jsonrpc" => {
                            version = Some(map.next_value::<Version>()?);
                        },
                        "result" => if result.is_none() {
                            result = Some(Ok(map.next_value::<R>()?));
                        } else {
                            return Err(serde::de::Error::custom("JSON-RPC Response contains both result and error field"));
                        },
                        "error" => if result.is_none() {
                            result = Some(Err(map.next_value::<Error<E>>()?));
                        } else {
                            return Err(serde::de::Error::custom("JSON-RPC Response contains both error and result field"));
                        },
                        "id" => {
                            id_set = true;
                            id = map.next_value::<Option<Id>>()?;
                        },
                        unknown => {
                            return Err(serde::de::Error::custom(format_args!("JSON-RPC Response contains unknown field {}", unknown)));
                        }
                    }
                }

                if id_set == false {
                    return Err(serde::de::Error::custom("JSON-RPC Response is missing a id field."))
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

impl<R, E> Response<R, E> {
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
    pub const fn error(jsonrpc: Version, error: Error<E>, id: Option<Id>) -> Self {
        Self {
            jsonrpc,
            payload: Err(error),
            id,
        }
    }
}
