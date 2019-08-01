extern crate bytes;

use std::collections::HashMap;
use std::io::Cursor;

use bytes::Buf;

mod types;

use std::io::Read;

use types::TypeCode;
use types::Value;

fn deserialize_boolean(buf: &mut Cursor<&[u8]>) -> std::option::Option<Value> {
    Some(Value::Boolean(buf.get_u8() != 0))
}

fn deserialize_byte(buf: &mut Cursor<&[u8]>) -> std::option::Option<Value> {
    Some(Value::Byte(buf.get_u8()))
}

fn deserialize_float(buf: &mut Cursor<&[u8]>) -> std::option::Option<Value> {
    Some(Value::Float(buf.get_f32_be()))
}

fn deserialize_integer(buf: &mut Cursor<&[u8]>) -> std::option::Option<Value> {
    Some(Value::Integer(buf.get_u32_be()))
}

fn deserialize_long(buf: &mut Cursor<&[u8]>) -> std::option::Option<Value> {
    Some(Value::Long(buf.get_i64_be()))
}

fn deserialize_short(buf: &mut Cursor<&[u8]>) -> std::option::Option<Value> {
    Some(Value::Short(buf.get_i16_be()))
}

fn deserialize_double(buf: &mut Cursor<&[u8]>) -> std::option::Option<Value> {
    Some(Value::Double(buf.get_f64_be()))
}

fn deserialize_string(buf: &mut Cursor<&[u8]>) -> std::option::Option<Value> {
    let size = buf.get_i16_be();
    let mut local_buffer = vec![0; size as usize];

    buf.read_exact(&mut local_buffer[..]).unwrap();
    Some(Value::String(String::from_utf8(local_buffer).unwrap()))
}

fn deserialize_string_array(buf: &mut Cursor<&[u8]>) -> std::option::Option<Value> {
    let size = buf.get_i16_be() as usize;
    let mut value = vec![];

    for _ in 0..size {
        if let Some(v) = deserialize_string(buf) {
            match v {
                Value::String(s) => {
                    value.push(s);
                }
                _ => {}
            }
        }
    }

    Some(Value::StringArray(value))
}

fn deserialize_byte_array(buf: &mut Cursor<&[u8]>) -> std::option::Option<Value> {
    let size = buf.get_u32_be() as usize;
    let mut value = vec![];

    for _ in 0..size {
        value.push(buf.get_u8());
    }

    Some(Value::ByteArray(value))
}

// def deserialize_byte_array(byte_stream):
//     size = deserialize_integer(byte_stream)
//     if not size:
//         return []
//     return list(byte_stream.read(size))

fn deserialize_array(buf: &mut Cursor<&[u8]>) -> std::option::Option<Value> {
    let size = buf.get_i16_be() as usize;
    let mut value = vec![];

    if size == 0 {
        return None;
    }

    let type_code = buf.get_u8();

    for _ in 0..size {
        value.push(deserialize(type_code, buf).unwrap());
    }

    Some(Value::Array(value))
}

fn deserialize_dictionary(buf: &mut Cursor<&[u8]>) -> std::option::Option<Value> {
    let key_type_code = buf.get_u8();
    let value_type_code = buf.get_u8();
    let size = buf.get_i16_be() as usize;

    if size == 0 {
        return None;
    }

    let mut value: HashMap<String, Value> = HashMap::new();

    for _ in 0..size {
        let key_code = if key_type_code == 0 || key_type_code == 42 {
            buf.get_u8()
        } else {
            key_type_code
        };

        let key = deserialize(key_code, buf);

        let value_code = if value_type_code == 0 || value_type_code == 42 {
            buf.get_u8()
        } else {
            value_type_code
        };
        let val = deserialize(value_code, buf);
        value.insert(format!("{}", key.unwrap()), val.unwrap());
    }

    Some(Value::Dictionary(value))
}

fn deserialize_parameter_table(buf: &mut Cursor<&[u8]>) -> HashMap<u8, Value> {
    let size = buf.get_i16_be() as usize;

    let mut value: HashMap<u8, Value> = HashMap::new();

    for _ in 0..size {
        let key_type_code = buf.get_u8();
        let value_type_code = buf.get_u8();
        let val = deserialize(value_type_code, buf);

        value.insert(key_type_code, val.unwrap());
    }

    value
}

fn deserialize_event_data(buf: &mut Cursor<&[u8]>) -> std::option::Option<Value> {
    let code = buf.get_u8();

    Some(Value::EventData(types::EventData {
        code,
        parameters: deserialize_parameter_table(buf),
    }))
}

fn deserialize_operation_response(buf: &mut Cursor<&[u8]>) -> std::option::Option<Value> {
    let code = buf.get_u8();
    let return_code = buf.get_i16_be();
    let debug_message = if let Value::String(s) = deserialize(buf.get_u8(), buf).unwrap() {
        s
    } else {
        "None".to_owned()
    };
    let parameters = deserialize_parameter_table(buf);
    Some(Value::OperationResponse(types::OperationResponse {
        code,
        return_code,
        debug_message,
        parameters,
    }))
}

fn deserialize_operation_request(buf: &mut Cursor<&[u8]>) -> std::option::Option<Value> {
    let code = buf.get_u8();

    Some(Value::OperationRequest(types::OperationRequest {
        code,
        parameters: deserialize_parameter_table(buf),
    }))
}

fn deserialize_object_array(buf: &mut Cursor<&[u8]>) -> std::option::Option<Value> {
    let size = buf.get_i16_be() as usize;
    let mut value = vec![];

    if size == 0 {
        return None;
    }

    for _ in 0..size {
        let type_code = buf.get_u8();
        value.push(deserialize(type_code, buf).unwrap());
    }

    Some(Value::Array(value))
}

pub fn deserialize(type_code: u8, buf: &mut Cursor<&[u8]>) -> std::option::Option<Value> {
    match TypeCode::from(type_code) {
        TypeCode::Null => Some(Value::None),
        TypeCode::Boolean => deserialize_boolean(buf),
        TypeCode::Byte => deserialize_byte(buf),
        TypeCode::Double => deserialize_double(buf),
        TypeCode::Float => deserialize_float(buf),
        TypeCode::Integer => deserialize_integer(buf),
        TypeCode::Long => deserialize_long(buf),
        TypeCode::Short => deserialize_short(buf),
        TypeCode::String => deserialize_string(buf),
        TypeCode::StringArray => deserialize_string_array(buf),
        TypeCode::ByteArray => deserialize_byte_array(buf),
        TypeCode::Array => deserialize_array(buf),
        TypeCode::Dictionary => deserialize_dictionary(buf),
        TypeCode::OperationRequest => deserialize_operation_request(buf),
        TypeCode::OperationResponse => deserialize_operation_response(buf),
        TypeCode::EventData => deserialize_event_data(buf),
        TypeCode::ObjectArray => deserialize_object_array(buf),
        _ => panic!("Unimplemented type code {}", type_code),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_dictionary() {
        let value = vec![
            68, 115, 115, 0, 2, 0, 8, 116, 101, 115, 116, 75, 101, 121, 49, 0, 10, 116, 101, 115,
            116, 86, 97, 108, 117, 101, 49, 0, 8, 116, 101, 115, 116, 75, 101, 121, 50, 0, 10, 116,
            101, 115, 116, 86, 97, 108, 117, 101, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let mut buf = Cursor::new(&value[..]);
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
        if let Some(value) = result {
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
        panic!("Unimplemented!")
    }

    #[test]
    fn deserialize_string_array() {
        let value = vec![
            121, 0, 2, 115, 0, 5, 116, 101, 115, 116, 49, 0, 5, 116, 101, 115, 116, 50, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let mut buf = Cursor::new(&value[..]);
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
        if let Some(value) = result {
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
        panic!("Unimplemented!")
    }

    #[test]
    fn test_deserialize_byte() {
        let value = vec![98, 6];
        let mut buf = Cursor::new(&value[..]);
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
        if let Some(value) = result {
            match value {
                Value::Byte(v) => return assert_eq!(v, 6),
                _ => assert!(false),
            }
        }
        panic!("Unimplemented!")
    }

    #[test]
    fn deserialize_double() {
        let value = vec![100, 64, 147, 74, 51, 51, 51, 51, 51, 0, 0, 0, 0, 0, 0, 0];
        let mut buf = Cursor::new(&value[..]);
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
        if let Some(value) = result {
            match value {
                Value::Double(v) => return assert_eq!(v, 1234.55),
                _ => assert!(false),
            }
        }
        panic!("Unimplemented!")
    }
    //     typed_code = struct.unpack("B", byte_stream.read(1))[0]
    //     result = deseliarizer.deserialize(byte_stream, typed_code)

    //     assert result
    //     assert result == 1234.55

    #[test]
    fn deserialize_event_data() {
        let value = vec![
            101, 100, 0, 2, 0, 115, 0, 5, 116, 101, 115, 116, 49, 1, 115, 0, 5, 116, 101, 115, 116,
            50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let mut buf = Cursor::new(&value[..]);
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
        if let Some(value) = result {
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
        panic!("Unimplemented!")
    }

    #[test]
    fn deserialize_float() {
        let value = vec![102, 68, 154, 81, 154, 0, 0, 0];
        let mut buf = Cursor::new(&value[..]);
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
        if let Some(value) = result {
            match value {
                Value::Float(v) => return assert_eq!(v, 1234.55),
                _ => assert!(false),
            }
        }
        panic!("Unimplemented!")
    }

    #[test]
    fn deserialize_integer() {
        let value = vec![105, 0, 0, 4, 210, 0, 0, 0];
        let mut buf = Cursor::new(&value[..]);
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
        if let Some(value) = result {
            match value {
                Value::Integer(v) => return assert_eq!(v, 1234),
                _ => assert!(false),
            }
        }
        panic!("Unimplemented!")
    }

    #[test]
    fn deserialize_short() {
        let value = vec![107, 4, 210, 0];
        let mut buf = Cursor::new(&value[..]);
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
        if let Some(value) = result {
            match value {
                Value::Short(v) => return assert_eq!(v, 1234),
                _ => assert!(false),
            }
        }
        panic!("Unimplemented!")
    }

    #[test]
    fn deserialize_long() {
        let value = vec![108, 0, 0, 0, 0, 0, 0, 4, 210, 0, 0, 0, 0, 0, 0, 0];
        let mut buf = Cursor::new(&value[..]);
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
        if let Some(value) = result {
            match value {
                Value::Long(v) => return assert_eq!(v, 1234),
                _ => assert!(false),
            }
        }
        panic!("Unimplemented!")
    }

    #[test]
    fn deserialize_integer_array() {
        let value = vec![121, 0, 2, 105, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0];
        let mut buf = Cursor::new(&value[..]);
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
        if let Some(value) = result {
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
        panic!("Unimplemented!")
    }
    //     typed_code = struct.unpack("B", byte_stream.read(1))[0]
    //     result = deseliarizer.deserialize(byte_stream, typed_code)

    //     assert result
    //     assert result[0] == 0
    //     assert result[1] == 1

    #[test]
    fn deserialize_boolean() {
        let value = vec![111, 1];
        let mut buf = Cursor::new(&value[..]);

        let type_code = buf.get_u8();
        let result = deserialize(type_code, &mut buf);
        if let Some(value) = result {
            match value {
                Value::Boolean(v) => return assert_eq!(v, true),
                _ => assert!(false),
            }
        }
        assert!(false);
    }
    #[test]
    fn deserialize_operation_response() {
        let value = vec![
            112, 100, 0, 100, 42, 0, 2, 0, 115, 0, 5, 116, 101, 115, 116, 49, 1, 115, 0, 5, 116,
            101, 115, 116, 50, 0, 0, 0, 0, 0, 0, 0,
        ];
        let mut buf = Cursor::new(&value[..]);
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
        if let Some(value) = result {
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
        panic!("Unimplemented!")
    }

    #[test]
    fn deserialize_operation_request() {
        let value = vec![
            113, 100, 0, 2, 0, 115, 0, 5, 116, 101, 115, 116, 49, 1, 115, 0, 5, 116, 101, 115, 116,
            50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let mut buf = Cursor::new(&value[..]);
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
        if let Some(value) = result {
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
        panic!("Unimplemented!")
    }

    #[test]
    fn deserialize_string() {
        let value = vec![
            115, 0, 12, 116, 101, 115, 116, 95, 109, 101, 115, 115, 97, 103, 101, 0,
        ];
        let mut buf = Cursor::new(&value[..]);
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
        if let Some(value) = result {
            match value {
                Value::String(v) => return assert_eq!(&v, "test_message"),
                _ => assert!(false),
            }
        }
        panic!("Unimplemented!")
    }

    #[test]
    fn deserialize_byte_array() {
        let value = vec![120, 0, 0, 0, 2, 6, 7, 0];
        let mut buf = Cursor::new(&value[..]);
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
        if let Some(value) = result {
            match value {
                Value::ByteArray(v) => {
                    assert_eq!(&v[0], &6);
                    assert_eq!(&v[1], &7);
                    return;
                }
                _ => assert!(false),
            }
        }
        panic!("Unimplemented!")
    }

    #[test]
    fn deserialize_array_dictionary() {
        let value = vec![
            121, 0, 1, 68, 105, 115, 0, 1, 0, 0, 0, 0, 0, 5, 116, 101, 115, 116, 49, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let mut buf = Cursor::new(&value[..]);
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
        if let Some(value) = result {
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
        panic!("Unimplemented!")
    }

    #[test]
    fn deserialize_array_byte_array() {
        let value = vec![121, 0, 1, 120, 0, 0, 0, 4, 0, 2, 4, 8, 0, 0, 0, 0];
        let mut buf = Cursor::new(&value[..]);
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
        if let Some(value) = result {
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
        panic!("Unimplemented!")
    }

    #[test]
    fn deserialize_array_array() {
        let value = vec![
            121, 0, 1, 121, 0, 3, 105, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ];
        let mut buf = Cursor::new(&value[..]);
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
        if let Some(value) = result {
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
        panic!("Unimplemented!")
    }

    #[test]
    fn deserialize_object_array() {
        let value = vec![
            122, 0, 2, 115, 0, 5, 116, 101, 115, 116, 49, 115, 0, 5, 116, 101, 115, 116, 50, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let mut buf = Cursor::new(&value[..]);
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
        if let Some(value) = result {
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
                _ => assert!(false),
            }
        }
        panic!("Unimplemented!")
    }
}
