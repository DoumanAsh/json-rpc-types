#![cfg_attr(feature = "cargo-fmt", rustfmt::skip)]

use json_rpc_types::Id;

type StrBuf = str_buf::StrBuf<36>;

#[test]
fn id_deserialization_int() {
    let s = "2";
    let deserialized: Id = serde_json::from_str(s).unwrap();
    assert_eq!(deserialized, Id::Num(2));

    let s = "-2";
    assert!(serde_json::from_str::<Id>(s).is_err());
    let s = "2.1";
    assert!(serde_json::from_str::<Id>(s).is_err());

    let s = r#"2"#;
    let deserialized: Id = serde_json::from_str(s).unwrap();
    assert_eq!(deserialized, Id::Num(2));
}

#[test]
#[cfg(not(feature = "id-fixed-int"))]
fn id_deserialization_str() {
    let s = r#""2""#;
    let deserialized: Id = serde_json::from_str(s).unwrap();
    let mut buffer = StrBuf::new();
    buffer.push_str("2");
    assert_eq!(deserialized, Id::Str(buffer));

    let s = r#""2x""#;
    let deserialized: Id = serde_json::from_str(s).unwrap();
    let mut buffer = StrBuf::new();
    buffer.push_str("2x");
    assert_eq!(deserialized, Id::Str(buffer));

    let s = r#"[0, 2, "3"]"#;
    let deserialized: Vec<Id> = serde_json::from_str(s).unwrap();
    let mut buffer = StrBuf::new();
    buffer.push_str("3");
    assert_eq!(deserialized, vec![Id::Num(0), Id::Num(2), Id::Str(buffer)]);
}

#[test]
fn id_serialization() {
    let mut buffer1 = StrBuf::new();
    let mut buffer2 = StrBuf::new();

    buffer1.push_str("3");
    buffer2.push_str("test");

    let d = vec![
        Id::Num(0),
        Id::Num(2),
        Id::Num(3),
        Id::Str(buffer1),
        Id::Str(buffer2),
    ];
    let serialized = serde_json::to_string(&d).unwrap();
    assert_eq!(serialized, r#"[0,2,3,"3","test"]"#);
}
