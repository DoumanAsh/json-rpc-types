type Response = json_rpc_types::Response<serde_json::Value, serde_json::Value>;

use json_rpc_types::{Id, Version, Error, ErrorCode};

fn create_error() -> Error<serde_json::Value> {
    let mut error = Error::from_code(ErrorCode::MethodNotFound);
    error.data = Some(serde_json::Value::from("text"));
    error
}

#[test]
fn success_result_serialize() {
    let result = Response::result(Version::V2, serde_json::Value::from(1), Some(Id::Num(1)));

    let serialized = serde_json::to_string(&result).unwrap();
    assert_eq!(serialized, r#"{"jsonrpc":"2.0","result":1,"id":1}"#);
}

#[test]
fn success_result_deserialize() {
    let expected = Response::result(Version::V2, serde_json::Value::from(1), Some(Id::Num(1)));
    let text = r#"{"jsonrpc":"2.0","result":1,"id":1}"#;

    let result: Response = serde_json::from_str(text).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn success_failure_serialize() {
    let result = Response::error(Version::V2, create_error(), Some(Id::Num(1)));

    let serialized = serde_json::to_string(&result).unwrap();
    assert_eq!(serialized, r#"{"jsonrpc":"2.0","error":{"code":-32601,"message":"Method not found","data":"text"},"id":1}"#);
}

#[test]
fn success_failure_deserialize() {
    let expected = Response::error(Version::V2, create_error(), None);
    let text = r#"{"jsonrpc":"2.0","error":{"data":"text","code":-32601,"message":"Method not found"},"id":null}"#;

    let result: Response = serde_json::from_str(text).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn response_deserialize_should_fail_on_unknown_field() {
    let text = r#"{"jsonrp":"2.0","error":{"data":"text","code":-32601,"message":"Method not found"}, "id": null, "result": 1}"#;
    let result: serde_json::Error = serde_json::from_str::<Response>(text).unwrap_err();
    assert_eq!(result.to_string(), "JSON-RPC Response contains unknown field jsonrp at line 1 column 9");
}

#[test]
fn response_deserialize_should_fail_on_mixing_result_error() {
    let text = r#"{"jsonrpc":"2.0","error":{"data":"text","code":-32601,"message":"Method not found"}, "id": null, "result": 1}"#;
    let result: serde_json::Error = serde_json::from_str::<Response>(text).unwrap_err();
    assert_eq!(result.to_string(), "JSON-RPC Response contains both result and error field at line 1 column 105");
}

#[test]
fn success_failure_deserialize_without_id() {
    let expected = Response::error(Version::V2, create_error(), None);
    let text = r#"{"jsonrpc":"2.0","error":{"data":"text","code":-32601,"message":"Method not found"}}"#;

    let result: Response = serde_json::from_str(text).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn success_serialize_null_id() {
    let result = Response::result(Version::V2, serde_json::Value::from(1), None);

    let serialized = serde_json::to_string(&result).unwrap();
    assert_eq!(serialized, r#"{"jsonrpc":"2.0","result":1,"id":null}"#);
}
