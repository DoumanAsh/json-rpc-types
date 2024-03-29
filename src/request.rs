use serde_derive::{Serialize, Deserialize};
type StrBuf = str_buf::StrBuf<32>;

use crate::version::Version;
use crate::id::Id;

///Request representation.
///
///Note that omitting `id` means that request is notification, rather than call, which expects
///response.
///This can be used to indicate lack of interest in response.
///
///Type parameters:
///
///- `P` - to specify type of `params` field, which is optional. Normally it should be collection of values or object. But choice is yours.
///- `T` - specifies textual type. By default it uses static buffer of 32 bytes, which is more than enough in normal cases.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request<P, T=StrBuf> {
    ///A String specifying the version of the JSON-RPC protocol.
    #[serde(default)]
    pub jsonrpc: Version,
    ///A String containing the name of the method to be invoked
    ///
    ///By default is static buffer of 32 bytes.
    pub method: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    ///A Structured value that holds the parameter values to be used during the invocation of the method
    pub params: Option<P>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ///An identifier established by the Client.
    ///
    ///If not present, request is notification to which
    ///there should be no response.
    pub id: Option<Id>,
}

impl<P, T> Request<P, T> {
    ///Returns whether request is notification.
    pub const fn is_notification(&self) -> bool {
        self.id.is_none()
    }
}
