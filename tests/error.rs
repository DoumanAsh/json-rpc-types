#![cfg_attr(feature = "cargo-fmt", rustfmt::skip)]

type Error = json_rpc_types::Error<()>;
use json_rpc_types::ErrorCode;

#[test]
fn error_should_truncated_message() {
    let message = "12345678912345678912345678912345";
    let error = Error::with_custom_msg_truncated(ErrorCode::ParseError, message);
    assert_eq!(error.message.len(), 31);
    assert_eq!(error.message, &message[..31]);
    let error = Error::with_custom_msg_truncated(ErrorCode::ParseError, &message[..31]);
    assert_eq!(error.message.len(), 31);
    assert_eq!(error.message, &message[..31]);
    let error = Error::with_custom_msg_truncated(ErrorCode::ParseError, &message[..29]);
    assert_eq!(error.message.len(), 29);
    assert_eq!(error.message, &message[..29]);
}

#[should_panic]
#[test]
fn error_should_panic_on_overflow() {
    let message = "12345678912345678912345678912345";
    let _error = Error::with_custom_msg(ErrorCode::ParseError, message);
}

#[test]
fn error_should_not_panic_message_within_limit() {
    let message = "12345678912345678912345678912345";
    let error = Error::with_custom_msg_truncated(ErrorCode::ParseError, &message[..31]);
    assert_eq!(error.message.len(), 31);
    assert_eq!(error.message, &message[..31]);
    let error = Error::with_custom_msg_truncated(ErrorCode::ParseError, &message[..30]);
    assert_eq!(error.message.len(), 30);
    assert_eq!(error.message, &message[..30]);
}
