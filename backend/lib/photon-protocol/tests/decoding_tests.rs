use photon_protocol::*;

use std::io::{Read, Cursor};

#[test]
fn deserialize_dictionary() {
    let value = vec![
        68, 115, 115, 0, 2, 0, 8, 116, 101, 115, 116, 75, 101, 121, 49, 0, 10, 116, 101, 115,
        116, 86, 97, 108, 117, 101, 49, 0, 8, 116, 101, 115, 116, 75, 101, 121, 50, 0, 10, 116,
        101, 115, 116, 86, 97, 108, 117, 101, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let mut buf = Cursor::new(&value[..]);
    let result = buf.decode();
    assert!(result.is_ok(), "Unimplemented!");
    let value = result.unwrap();
    match value {
        Value::Dictionary(v) => {
            return assert_eq!(
                v.get("testKey1"),
                Some(&Value::String("testValue1".to_owned()))
            )
        }
        _ => assert!(false),
    }
}

#[test]
fn deserialize_string_array() {
    let value = vec![
        121, 0, 2, 115, 0, 5, 116, 101, 115, 116, 49, 0, 5, 116, 101, 115, 116, 50, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let mut buf = Cursor::new(&value[..]);
    let result = buf.decode();
    assert!(result.is_ok(), "Unimplemented!");
    let value = result.unwrap();

    match value {
        Value::Array(v) => {
            if let Value::String(val) = &v[0] {
                assert_eq!(val, &"test1".to_owned());
            }
            if let Value::String(val) = &v[1] {
                assert_eq!(val, &"test2".to_owned());
            }
            return;
        }
        Value::StringArray(v) => {
            return assert_eq!(vec!["test1".to_owned(), "test2".to_owned()], v)
        }
        _ => assert!(false),
    }
}

#[test]
fn test_deserialize_byte() {
    let value = vec![98, 6];
    let mut buf = Cursor::new(&value[..]);
    let result = buf.decode();
    assert!(result.is_ok(), "Unimplemented!");
    let value = result.unwrap();
    match value {
        Value::Byte(v) => return assert_eq!(v, 6),
        _ => assert!(false),
    }
}

#[test]
fn deserialize_double() {
    let value = vec![100, 64, 147, 74, 51, 51, 51, 51, 51, 0, 0, 0, 0, 0, 0, 0];
    let mut buf = Cursor::new(&value[..]);
    let result = buf.decode();
    assert!(result.is_ok(), "Unimplemented!");
    let value = result.unwrap();
    match value {
        Value::Double(v) => return assert_eq!(v, 1234.55),
        _ => assert!(false),
    }
}

#[test]
fn deserialize_event_data() {
    let value = vec![
        101, 100, 0, 2, 0, 115, 0, 5, 116, 101, 115, 116, 49, 1, 115, 0, 5, 116, 101, 115, 116,
        50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let mut buf = Cursor::new(&value[..]);
    let result = buf.decode();
    assert!(result.is_ok(), "Unimplemented!");
    let value = result.unwrap();
    match value {
        Value::EventData(v) => {
            assert_eq!(v.code, 100);
            assert_eq!(
                v.parameters.get(&0).unwrap(),
                &Value::String("test1".to_owned())
            );
            return;
        }
        _ => assert!(false),
    }
}

#[test]
fn deserialize_float() {
    let value = vec![102, 68, 154, 81, 154, 0, 0, 0];
    let mut buf = Cursor::new(&value[..]);
    let result = buf.decode();
    assert!(result.is_ok(), "Unimplemented!");
    let value = result.unwrap();
    match value {
        Value::Float(v) => return assert_eq!(v, 1234.55),
        _ => assert!(false),
    }
}

#[test]
fn deserialize_integer() {
    let value = vec![105, 0, 0, 4, 210, 0, 0, 0];
    let mut buf = Cursor::new(&value[..]);
    let result = buf.decode();
    assert!(result.is_ok(), "Unimplemented!");
    let value = result.unwrap();
    match value {
        Value::Integer(v) => return assert_eq!(v, 1234),
        _ => assert!(false),
    }
}

#[test]
fn deserialize_short() {
    let value = vec![107, 4, 210, 0];
    let mut buf = Cursor::new(&value[..]);
    let result = buf.decode();
    assert!(result.is_ok(), "Unimplemented!");
    let value = result.unwrap();
    match value {
        Value::Short(v) => return assert_eq!(v, 1234),
        _ => assert!(false),
    }
}

#[test]
fn deserialize_long() {
    let value = vec![108, 0, 0, 0, 0, 0, 0, 4, 210, 0, 0, 0, 0, 0, 0, 0];
    let mut buf = Cursor::new(&value[..]);
    let result = buf.decode();
    assert!(result.is_ok(), "Unimplemented!");
    let value = result.unwrap();
    match value {
        Value::Long(v) => return assert_eq!(v, 1234),
        _ => assert!(false),
    }
}

#[test]
fn deserialize_integer_array() {
    let value = vec![121, 0, 2, 105, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0];
    let mut buf = Cursor::new(&value[..]);
    let result = buf.decode();
    assert!(result.is_ok(), "Unimplemented!");
    let value = result.unwrap();
    match value {
        Value::Array(v) => {
            if let Value::Integer(val) = v[0] {
                assert_eq!(val, 0);
            }
            if let Value::Integer(val) = v[1] {
                assert_eq!(val, 1);
            }
            return;
        }
        _ => assert!(false),
    }
}

#[test]
fn deserialize_boolean() {
    let value = vec![111, 1];
    let mut buf = Cursor::new(&value[..]);
    let result = buf.decode();
    assert!(result.is_ok(), "Unimplemented!");
    let value = result.unwrap();
    match value {
        Value::Boolean(v) => return assert_eq!(v, true),
        _ => assert!(false),
    }
}

#[test]
fn deserialize_operation_response() {
    let value = vec![
        112, 100, 0, 100, 42, 0, 2, 0, 115, 0, 5, 116, 101, 115, 116, 49, 1, 115, 0, 5, 116,
        101, 115, 116, 50, 0, 0, 0, 0, 0, 0, 0,
    ];
    let mut buf = Cursor::new(&value[..]);
    let result = buf.decode();
    assert!(result.is_ok(), "Unimplemented!");
    let value = result.unwrap();
    match value {
        Value::OperationResponse(v) => {
            assert_eq!(v.code, 100);
            assert_eq!(v.return_code, 100);
            assert_eq!(
                v.parameters.get(&1).unwrap(),
                &Value::String("test2".to_owned())
            );
            return;
        }
        _ => assert!(false),
    }
}

#[test]
fn deserialize_operation_request() {
    let value = vec![
        113, 100, 0, 2, 0, 115, 0, 5, 116, 101, 115, 116, 49, 1, 115, 0, 5, 116, 101, 115, 116,
        50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let mut buf = Cursor::new(&value[..]);
    let result = buf.decode();
    assert!(result.is_ok(), "Unimplemented!");
    let value = result.unwrap();
    match value {
        Value::OperationRequest(v) => {
            assert_eq!(v.code, 100);
            assert_eq!(
                v.parameters.get(&1).unwrap(),
                &Value::String("test2".to_owned())
            );
            return;
        }
        _ => assert!(false),
    }
}

#[test]
fn deserialize_string() {
    let value = vec![
        115, 0, 12, 116, 101, 115, 116, 95, 109, 101, 115, 115, 97, 103, 101, 0,
    ];
    let mut buf = Cursor::new(&value[..]);
    let result = buf.decode();
    assert!(result.is_ok(), "Unimplemented!");
    let value = result.unwrap();
    match value {
        Value::String(v) => return assert_eq!(&v, "test_message"),
        _ => assert!(false),
    }
}

#[test]
fn deserialize_byte_array() {
    let value = vec![120, 0, 0, 0, 2, 6, 7, 0];
    let mut buf = Cursor::new(&value[..]);
    let result = buf.decode();
    assert!(result.is_ok(), "Unimplemented!");
    let value = result.unwrap();
    match value {
        Value::ByteArray(v) => {
            assert_eq!(&v[0], &6);
            assert_eq!(&v[1], &7);
            return;
        }
        _ => assert!(false),
    }
}

#[test]
fn deserialize_array_dictionary() {
    let value = vec![
        121, 0, 1, 68, 105, 115, 0, 1, 0, 0, 0, 0, 0, 5, 116, 101, 115, 116, 49, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let mut buf = Cursor::new(&value[..]);
    let result = buf.decode();
    assert!(result.is_ok(), "Unimplemented!");
    let value = result.unwrap();
    match value {
        Value::Array(v) => {
            if let Value::Dictionary(val) = &v[0] {
                assert_eq!(val.get("0").unwrap(), &Value::String("test1".to_owned()));
            }
            return;
        }
        _ => assert!(false),
    }
}

#[test]
fn deserialize_array_byte_array() {
    let value = vec![121, 0, 1, 120, 0, 0, 0, 4, 0, 2, 4, 8, 0, 0, 0, 0];
    let mut buf = Cursor::new(&value[..]);
    let result = buf.decode();
    assert!(result.is_ok(), "Unimplemented!");
    let value = result.unwrap();

    match value {
        Value::Array(v) => {
            if let Value::ByteArray(val) = &v[0] {
                assert_eq!(val[0], 0);
                assert_eq!(val[1], 2);
                assert_eq!(val[2], 4);
                assert_eq!(val[3], 8);
                return;
            }
        }
        _ => assert!(false),
    }
}

#[test]
fn deserialize_array_array() {
    let value = vec![
        121, 0, 1, 121, 0, 3, 105, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ];
    let mut buf = Cursor::new(&value[..]);
    let result = buf.decode();
    assert!(result.is_ok(), "Unimplemented!");
    let value = result.unwrap();
    match value {
        Value::Array(v) => {
            if let Value::Integer(val) = &v[0][0] {
                assert_eq!(val, &1);
            }
            if let Value::Integer(val) = &v[0][1] {
                assert_eq!(val, &2);
            }
            if let Value::Integer(val) = &v[0][2] {
                assert_eq!(val, &3);
            }
            return;
        }
        _ => assert!(false),
    }
}

#[test]
fn deserialize_object_array() {
    let value = vec![
        122, 0, 2, 115, 0, 5, 116, 101, 115, 116, 49, 115, 0, 5, 116, 101, 115, 116, 50, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let mut buf = Cursor::new(&value[..]);
    let result = buf.decode();
    assert!(result.is_ok(), "Unimplemented!");
    let value = result.unwrap();

    match value {
        Value::ObjectArray(v) => {
            if let Value::Object(val) = &*v[0] {
                assert_eq!(**val, Value::String("test1".to_owned()));
            }
            if let Value::Object(val) = &*v[1] {
                assert_eq!(**val, Value::String("test2".to_owned()));
            }    
        }
        _ => assert!(false),
    };
}