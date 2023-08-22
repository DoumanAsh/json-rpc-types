#![rustfmt::skip]

type Request = json_rpc_types::Request<Vec<serde_json::Value>>;

use core::convert::TryInto;
use json_rpc_types::{Id, Version};

#[test]
fn method_call_serialize() {
    let m = Request {
        jsonrpc: Version::V2,
        method: "update".try_into().unwrap(),
        params: Some(vec![serde_json::Value::from(1), serde_json::Value::from(2)]),
        id: Some(Id::Num(1)),
    };

    let serialized = serde_json::to_string(&m).unwrap();
    assert_eq!(serialized, r#"{"jsonrpc":"2.0","method":"update","params":[1,2],"id":1}"#);
}

#[test]
fn notification_serialize() {
    let m = Request {
        jsonrpc: Version::V2,
        method: "update".try_into().unwrap(),
        params: Some(vec![serde_json::Value::from(1), serde_json::Value::from(2)]),
        id: None,
    };

    let serialized = serde_json::to_string(&m).unwrap();
    assert_eq!(serialized, r#"{"jsonrpc":"2.0","method":"update","params":[1,2]}"#);
}

#[test]
fn notification_deserialize() {
    let text = r#"{"jsonrpc":"2.0","method":"update","params":[1,2]}"#;
    let notification: Request = serde_json::from_str(text).unwrap();

    let expected = Request {
        jsonrpc: Version::V2,
        method: "update".try_into().unwrap(),
        params: Some(vec![serde_json::Value::from(1), serde_json::Value::from(2)]),
        id: None,
    };

    assert!(notification.is_notification());
    assert_eq!(expected, notification);

    let text = r#"{"jsonrpc":"2.0","method":"update"}"#;
    let notification: Request = serde_json::from_str(text).unwrap();

    let expected = Request {
        jsonrpc: Version::V2,
        method: "update".try_into().unwrap(),
        params: None,
        id: None,
    };

    assert!(notification.is_notification());
    assert_eq!(expected, notification);
}

#[test]
fn call_deserialize() {
    let text = r#"{"jsonrpc":"2.0","method":"update","params":[1,2],"id":1}"#;
    let notification: Request = serde_json::from_str(text).unwrap();

    let expected = Request {
        jsonrpc: Version::V2,
        method: "update".try_into().unwrap(),
        params: Some(vec![serde_json::Value::from(1), serde_json::Value::from(2)]),
        id: Some(Id::Num(1)),
    };

    assert!(!notification.is_notification());
    assert_eq!(expected, notification);

    let text = r#"{"jsonrpc":"2.0","method":"update","id":1}"#;
    let notification: Request = serde_json::from_str(text).unwrap();

    let expected = Request {
        jsonrpc: Version::V2,
        method: "update".try_into().unwrap(),
        params: None,
        id: Some(Id::Num(1)),
    };

    assert!(!notification.is_notification());
    assert_eq!(expected, notification);
}
