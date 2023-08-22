#[rustfmt::skip]

use serde::{Serialize, Deserialize};
use serde::de::{Deserializer};
use serde::ser::{Serializer};

use core::fmt::Display;
use core::mem;

pub(crate) type StrBuf = str_buf::StrBuf<31>;

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
pub struct Error<T, M=StrBuf> {
    ///Code
    pub code: ErrorCode,
    ///Message
    pub message: M,
    ///Optional data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<'a, T, EM: From<&'a str>> Error<T, EM> {
    #[inline]
    ///Constructs error by converting message from string.
    pub fn with_text_message(code: ErrorCode, message: &'a str) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }
}

impl<T> Display for Error<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.code.message())
    }
}

impl<const N: usize, T> Error<T, str_buf::StrBuf<N>> {
    ///Constructs error with custom message
    pub const fn with_custom_msg_truncated(code: ErrorCode, message: &str) -> Self {
        let mut storage = [mem::MaybeUninit::uninit(); N];
        let msg = message.as_bytes();
        let mut idx = 0;

        let idx_limit = if storage.len() > msg.len() {
            msg.len()
        } else {
            storage.len()
        };

        loop {
            storage[idx] = mem::MaybeUninit::new(msg[idx]);
            idx += 1;
            if idx == idx_limit {
                break;
            }
        }

        let message = unsafe {
            str_buf::StrBuf::from_storage(storage, idx as u8)
        };

        Self {
            code,
            message,
            data: None,
        }
    }

    ///Constructs error with custom message
    pub const fn with_custom_msg(code: ErrorCode, message: &str) -> Self {
        let mut storage = [mem::MaybeUninit::uninit(); N];
        let msg = message.as_bytes();
        let mut idx = 0;
        loop {
            storage[idx] = mem::MaybeUninit::new(msg[idx]);
            idx += 1;
            if idx == msg.len() {
                break;
            }
        }

        let message = unsafe {
            str_buf::StrBuf::from_storage(storage, msg.len() as u8)
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

    #[inline(always)]
    ///Adds optional payload to instance
    pub fn set_data(self, data: T) -> Self {
        Self {
            code: self.code,
            message: self.message,
            data: Some(data)
        }
    }
}
