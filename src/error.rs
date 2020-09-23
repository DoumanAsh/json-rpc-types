use serde::{Serialize, Deserialize};
use serde::de::{Deserializer};
use serde::ser::{Serializer};

type StrBuf = str_buf::StrBuf<[u8; 24]>;

///JSON-RPC error code
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ErrorCode {
    ///Invalid JSON was received by the server.
    ///An error occurred on the server while parsing the JSON text.
    ParseError,
    ///The JSON sent is not a valid Request object.
    InvalidRequest,
    ///The method does not exist / is not available.
    MethodNotFound,
    ///Invalid method parameters.
    InvalidParams,
    ///Internal JSON-RPC error.
    InternalError,
    ///Reserved for implementation-defined server-errors.
    ServerError(i64),
}

impl ErrorCode {
    ///Creates new instance from code.
    pub const fn from_code(code: i64) -> Self {
        match code {
            -32700 => ErrorCode::ParseError,
            -32600 => ErrorCode::InvalidRequest,
            -32601 => ErrorCode::MethodNotFound,
            -32602 => ErrorCode::InvalidParams,
            -32603 => ErrorCode::InternalError,
            code => ErrorCode::ServerError(code),
        }
    }

    ///Returns integer code value
    pub const fn code(&self) -> i64 {
        match self {
            ErrorCode::ParseError => -32700,
            ErrorCode::InvalidRequest => -32600,
            ErrorCode::MethodNotFound => -32601,
            ErrorCode::InvalidParams => -32602,
            ErrorCode::InternalError => -32603,
            ErrorCode::ServerError(code) => *code,
        }
    }

    ///Returns textual representation of the code.
    pub const fn message(&self) -> &'static str {
        match self {
            ErrorCode::ParseError => "Parse error",
            ErrorCode::InvalidRequest => "Invalid Request",
            ErrorCode::MethodNotFound => "Method not found",
            ErrorCode::InvalidParams => "Invalid params",
            ErrorCode::InternalError => "Internal error",
            ErrorCode::ServerError(_) => "Server error",
        }
    }
}

impl<'a> Deserialize<'a> for ErrorCode {
    #[inline]
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<ErrorCode, D::Error> {
        let code: i64 = Deserialize::deserialize(deserializer)?;
        Ok(ErrorCode::from_code(code))
    }
}

impl Serialize for ErrorCode {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(self.code())
    }
}

///Error object, defined by JSON-RPC
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Error<T> {
    ///Code
    pub code: ErrorCode,
    ///Message
    pub message: StrBuf,
    ///Optional data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> Error<T> {
    ///Constructs error with custom message
    pub const fn with_custom_msg(code: ErrorCode, message: &str) -> Self {
        let mut storage = [0; 24];
        let msg = message.as_bytes();
        let mut idx = 0;
        loop {
            storage[idx] = msg[idx];
            idx += 1;
            if idx == msg.len() {
                break;
            }
        }

        let message = unsafe {
            StrBuf::from_storage(storage, msg.len() as u8)
        };

        Self {
            code,
            message,
            data: None,
        }
    }

    #[inline]
    ///Creates new error, deriving message from code.
    pub const fn from_code(code: ErrorCode) -> Self {
        Self::with_custom_msg(code, code.message())
    }
}
