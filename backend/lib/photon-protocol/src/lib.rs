extern crate bytes;
mod types;

use std::{error, fmt};
use std::io::{Read, Cursor};
use std::collections::HashMap;

use bytes::Buf;


use types::TypeCode;

pub use types::Parameters;
pub use types::Value;

#[derive(Debug, Clone)]
pub struct DeserializationError {
    type_code: u8,
}

impl fmt::Display for DeserializationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to deserialize type_code {}", self.type_code)
    }
}

impl error::Error for DeserializationError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub fn deserialize_boolean(buf: &mut Cursor<&[u8]>) -> Option<bool> {
    let v = if buf.remaining() > 0 {
        buf.get_u8()
    } else {
        return None;
    };
    Some(v != 0)
}

pub fn deserialize_byte(buf: &mut Cursor<&[u8]>) -> Option<u8> {
    let v = if buf.remaining() > 0 {
        buf.get_u8()
    } else {
        return None;
    };
    Some(v)
}

pub fn deserialize_float(buf: &mut Cursor<&[u8]>) -> Option<f32> {
    let v = if buf.remaining() >= 4 {
        buf.get_f32_be()
    } else {
        return None;
    };
    Some(v)
}

pub fn deserialize_integer(buf: &mut Cursor<&[u8]>) -> Option<u32> {
    let v = if buf.remaining() >= 4 {
        buf.get_u32_be()
    } else {
        return None;
    };
    Some(v)
}

pub fn deserialize_long(buf: &mut Cursor<&[u8]>) -> Option<i64> {
    let v = if buf.remaining() >= 8 {
        buf.get_i64_be()
    } else {
        return None;
    };
    Some(v)
}

pub fn deserialize_short(buf: &mut Cursor<&[u8]>) -> Option<i16> {
    let v = if buf.remaining() >= 2 {
        buf.get_i16_be()
    } else {
        return None;
    };
    Some(v)
}

pub fn deserialize_double(buf: &mut Cursor<&[u8]>) -> Option<f64> {
    let v = if buf.remaining() >= 8 {
        buf.get_f64_be()
    } else {
        return None;
    };
    Some(v)
}

pub fn deserialize_string(buf: &mut Cursor<&[u8]>) -> Option<String> {
    let size = if buf.remaining() >= 2 {
        buf.get_i16_be()
    } else {
        return None;
    };
    let mut local_buffer = vec![0; size as usize];

    if let Ok(_) = buf.read_exact(&mut local_buffer[..]) {
        if let Ok(s) = String::from_utf8(local_buffer) {
            return Some(s);
        }
    }
    None
}

pub fn deserialize_string_array(buf: &mut Cursor<&[u8]>) -> Option<Vec<String>> {
    let size = if buf.remaining() >= 2 {
        buf.get_i16_be() as usize
    } else {
        return None;
    };
    let mut value = vec![];

    for _ in 0..size {
        if let Some(v) = deserialize_string(buf) {
            value.push(v);
        }
    }

    Some(value)
}

pub fn deserialize_byte_array(buf: &mut Cursor<&[u8]>) -> Option<Vec<u8>> {
    let size = buf.get_u32_be() as usize;
    let mut value = vec![];

    for _ in 0..size {
        if buf.remaining() > 0 {
            value.push(buf.get_u8());
        } else {
            return None;
        }
    }

    Some(value)
}

pub fn deserialize_array(buf: &mut Cursor<&[u8]>) -> Option<Vec<Value>> {
    let size = if buf.remaining() >= 2 {
        buf.get_i16_be()
    } else {
        return None;
    };
    let mut value = vec![];

    if size == 0 {
        return None;
    }

    let type_code = buf.get_u8();

    for _ in 0..size {
        if let Ok(v) = deserialize(type_code, buf) {
            value.push(v);
        } else {
            break;
        }
    }

    Some(value)
}

pub fn deserialize_dictionary(buf: &mut Cursor<&[u8]>) -> Option<HashMap<String, Value>> {
    let key_type_code = buf.get_u8();
    let value_type_code = buf.get_u8();
    let size = if buf.remaining() >= 2 {
        buf.get_i16_be()
    } else {
        return None;
    };

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

    Some(value)
}

fn deserialize_parameter_table(buf: &mut Cursor<&[u8]>) -> HashMap<u8, Value> {
    let mut value: HashMap<u8, Value> = HashMap::new();

    let size = if buf.remaining() >= 2 {
        buf.get_i16_be()
    } else {
        return value;
    };

    for _ in 0..size {
        let key_type_code = if buf.remaining() > 0 {
            buf.get_u8()
        } else {
            break;
        };
        let value_type_code = if buf.remaining() > 0 {
            buf.get_u8()
        } else {
            break;
        };
        let val = deserialize(value_type_code, buf);
        if let Ok(val) = val {
            value.insert(key_type_code, val);
        }
    }

    value
}

pub fn deserialize_event_data(buf: &mut Cursor<&[u8]>) -> Option<types::EventData> {
    let code = buf.get_u8();

    Some(types::EventData {
        code,
        parameters: deserialize_parameter_table(buf),
    })
}

pub fn deserialize_operation_response(buf: &mut Cursor<&[u8]>) -> Option<types::OperationResponse> {
    let code = buf.get_u8();
    let return_code = buf.get_i16_be();
    let debug_message = if let Value::String(s) = deserialize(buf.get_u8(), buf).unwrap() {
        s
    } else {
        "None".to_owned()
    };
    let parameters = deserialize_parameter_table(buf);
    Some(types::OperationResponse {
        code,
        return_code,
        debug_message,
        parameters,
    })
}

pub fn deserialize_operation_request(buf: &mut Cursor<&[u8]>) -> Option<types::OperationRequest> {
    let code = buf.get_u8();

    Some(types::OperationRequest {
        code,
        parameters: deserialize_parameter_table(buf),
    })
}

pub fn deserialize_object_array(buf: &mut Cursor<&[u8]>) -> Option<Vec<Value>> {
    let size = if buf.remaining() >= 2 {
        buf.get_i16_be()
    } else {
        return None;
    };
    let mut value = vec![];

    if size == 0 {
        return None;
    }

    for _ in 0..size {
        let type_code = if buf.remaining() > 0 {
            buf.get_u8()
        } else {
            break;
        };
        value.push(deserialize(type_code, buf).unwrap());
    }

    Some(value)
}

pub fn deserialize(type_code: u8, buf: &mut Cursor<&[u8]>) -> Result<Value, DeserializationError> {
    match TypeCode::from(type_code) {
        TypeCode::Null => Ok(Value::None),
        TypeCode::Boolean => deserialize_boolean(buf)
            .map(|v| Ok(Value::Boolean(v)))
            .unwrap_or(Err(DeserializationError { type_code })),
        TypeCode::Byte => deserialize_byte(buf)
            .map(|v| Ok(Value::Byte(v)))
            .unwrap_or(Err(DeserializationError { type_code })),
        TypeCode::Double => deserialize_double(buf)
            .map(|v| Ok(Value::Double(v)))
            .unwrap_or(Err(DeserializationError { type_code })),
        TypeCode::Float => deserialize_float(buf)
            .map(|v| Ok(Value::Float(v)))
            .unwrap_or(Err(DeserializationError { type_code })),
        TypeCode::Integer => deserialize_integer(buf)
            .map(|v| Ok(Value::Integer(v)))
            .unwrap_or(Err(DeserializationError { type_code })),
        TypeCode::Long => deserialize_long(buf)
            .map(|v| Ok(Value::Long(v)))
            .unwrap_or(Err(DeserializationError { type_code })),
        TypeCode::Short => deserialize_short(buf)
            .map(|v| Ok(Value::Short(v)))
            .unwrap_or(Err(DeserializationError { type_code })),
        TypeCode::String => deserialize_string(buf)
            .map(|v| Ok(Value::String(v)))
            .unwrap_or(Err(DeserializationError { type_code })),
        TypeCode::StringArray => deserialize_string_array(buf)
            .map(|v| Ok(Value::StringArray(v)))
            .unwrap_or(Err(DeserializationError { type_code })),
        TypeCode::ByteArray => deserialize_byte_array(buf)
            .map(|v| Ok(Value::ByteArray(v)))
            .unwrap_or(Err(DeserializationError { type_code })),
        TypeCode::Array => deserialize_array(buf)
            .map(|v| Ok(Value::Array(v)))
            .unwrap_or(Err(DeserializationError { type_code })),
        TypeCode::Dictionary => deserialize_dictionary(buf)
            .map(|v| Ok(Value::Dictionary(v)))
            .unwrap_or(Err(DeserializationError { type_code })),
        TypeCode::OperationRequest => deserialize_operation_request(buf)
            .map(|v| Ok(Value::OperationRequest(v)))
            .unwrap_or(Err(DeserializationError { type_code })),
        TypeCode::OperationResponse => deserialize_operation_response(buf)
            .map(|v| Ok(Value::OperationResponse(v)))
            .unwrap_or(Err(DeserializationError { type_code })),
        TypeCode::EventData => deserialize_event_data(buf)
            .map(|v| Ok(Value::EventData(v)))
            .unwrap_or(Err(DeserializationError { type_code })),
        TypeCode::ObjectArray => deserialize_object_array(buf)
            .map(|v| Ok(Value::ObjectArray(v)))
            .unwrap_or(Err(DeserializationError { type_code })),
        _ => Err(DeserializationError { type_code }),
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
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
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
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
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
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
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
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
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
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
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
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
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
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
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
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
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
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
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

        let type_code = buf.get_u8();
        let result = deserialize(type_code, &mut buf);
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
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
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
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
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
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
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
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
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
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
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
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
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
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
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
        let type_code = buf.get_u8();

        let result = deserialize(type_code, &mut buf);
        assert!(result.is_ok(), "Unimplemented!");
        let value = result.unwrap();

        match value {
            Value::ObjectArray(v) => {
                if let Value::String(val) = &v[0] {
                    assert_eq!(val, &"test1".to_owned());
                }
                if let Value::String(val) = &v[1] {
                    assert_eq!(val, &"test2".to_owned());
                }
            }
            _ => assert!(false),
        };
    }
}
