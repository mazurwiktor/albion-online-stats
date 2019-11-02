use std::collections::HashMap;
use std::io::{Cursor, Read};
use std::mem::size_of;

use bytes::Buf;

use crate::layout::*;

pub type PhotonDecodeError = &'static str;
pub type PhotonDecodeResult<T> = std::result::Result<T, PhotonDecodeError>;
pub type PhotonCursor<'a> = Cursor<&'a [u8]>;

pub trait Decode<T> {
    fn decode(&mut self) -> PhotonDecodeResult<T>;
}

trait TypedDecode {
    fn typed_decode(&mut self, type_code: u8) -> PhotonDecodeResult<Value>;
}

macro_rules! impl_decode {
    ($type:ty, $decode_func:ident, $bytes_to_consume:expr) => {
        impl Decode<$type> for PhotonCursor<'_> {
            fn decode(&mut self) -> PhotonDecodeResult<$type> {
                let v = if self.remaining() >= $bytes_to_consume {
                    self.$decode_func()
                } else {
                    return Err(concat!(
                        "Failed to decode",
                        stringify!($type),
                        ", not enough bytes"
                    ));
                };
                Ok(v)
            }
        }
    };
}

impl_decode!(u8, get_u8, 1);
impl_decode!(f32, get_f32_be, 4);
impl_decode!(u32, get_u32_be, 4);
impl_decode!(i64, get_i64_be, 8);
impl_decode!(i16, get_i16_be, 2);
impl_decode!(f64, get_f64_be, 8);

impl Decode<bool> for PhotonCursor<'_> {
    fn decode(&mut self) -> PhotonDecodeResult<bool> {
        let v = if self.remaining() >= 1 {
            self.get_u8()
        } else {
            return Err("Failed to decode bool, not enough bytes");
        };
        Ok(v != 0)
    }
}

impl Decode<String> for PhotonCursor<'_> {
    fn decode(&mut self) -> PhotonDecodeResult<String> {
        let size: i16 = self.decode()?;
        if size < 0 {
            return Err("Failed to decode String, unreasonable size");
        }

        let mut local_buffer = vec![0; size as usize];
        if let Ok(_) = self.read_exact(&mut local_buffer[..]) {
            if let Ok(s) = String::from_utf8(local_buffer) {
                return Ok(s);
            }
        }

        Err("Failed to decode String, not enough bytes")
    }
}

impl Decode<Vec<String>> for PhotonCursor<'_> {
    fn decode(&mut self) -> PhotonDecodeResult<Vec<String>> {
        let size: i16 = self.decode()?;
        if size < 0 {
            return Err("Failed to decode String, unreasonable size");
        }
        let mut value: Vec<String> = vec![];
        for _ in 0..size {
            value.push(self.decode()?);
        }

        Ok(value)
    }
}

impl Decode<Vec<u8>> for PhotonCursor<'_> {
    fn decode(&mut self) -> PhotonDecodeResult<Vec<u8>> {
        let size: u32 = self.decode()?;
        let mut value: Vec<u8> = vec![];
        for _ in 0..size {
            value.push(self.decode()?);
        }

        Ok(value)
    }
}

impl Decode<Vec<Value>> for PhotonCursor<'_> {
    fn decode(&mut self) -> PhotonDecodeResult<Vec<Value>> {
        let size: i16 = self.decode()?;
        if size <= 0 {
            return Err("Failed to decode Vec<Value>, unreasonable size");
        }
        let type_code: u8 = self.decode()?;

        let mut value: Vec<Value> = vec![];
        for _ in 0..size {
            if let Ok(v) = self.typed_decode(type_code) {
                value.push(v);
            } else {
                break;
            }
        }

        Ok(value)
    }
}

impl Decode<HashMap<String, Value>> for PhotonCursor<'_> {
    fn decode(&mut self) -> PhotonDecodeResult<HashMap<String, Value>> {
        let key_type_code: u8 = self.decode()?;
        let value_type_code: u8 = self.decode()?;
        let size: i16 = self.decode()?;
        if size <= 0 {
            return Err("Failed to decode HashMap<String, Value>, unreasonable size");
        }

        let mut value: HashMap<String, Value> = HashMap::new();
        for _ in 0..size {
            let key_code: u8 = if key_type_code == 0 || key_type_code == 42 {
                self.decode()?
            } else {
                key_type_code
            };

            let key = self.typed_decode(key_code);

            let value_code: u8 = if value_type_code == 0 || value_type_code == 42 {
                self.decode()?
            } else {
                value_type_code
            };
            let val = self.typed_decode(value_code);
            value.insert(format!("{}", key.unwrap()), val.unwrap());
        }

        Ok(value)
    }
}

impl Decode<HashMap<u8, Value>> for PhotonCursor<'_> {
    fn decode(&mut self) -> PhotonDecodeResult<HashMap<u8, Value>> {
        let size: i16 = self.decode()?;
        if size <= 0 {
            return Err("Failed to decode HashMap<u8, Value>, unreasonable size");
        }

        let mut value: HashMap<u8, Value> = HashMap::new();
        for _ in 0..size {
            let key_type_code: u8 = if let Ok(v) = self.decode() {
                v
            } else {
                break;
            };
            let val: Value = if let Ok(v) = self.decode() {
                v
            } else {
                break;
            };
            value.insert(key_type_code, val);
        }

        Ok(value)
    }
}

impl Decode<EventData> for PhotonCursor<'_> {
    fn decode(&mut self) -> PhotonDecodeResult<EventData> {
        let code: u8 = self.decode()?;
        let parameters: HashMap<u8, Value> = self.decode()?;
        Ok(EventData { code, parameters })
    }
}

impl Decode<OperationResponse> for PhotonCursor<'_> {
    fn decode(&mut self) -> PhotonDecodeResult<OperationResponse> {
        let code: u8 = self.decode()?;
        let return_code: i16 = self.decode()?;
        let maybe_debug_message: Value = self.decode()?;
        let debug_message = if let Value::String(s) = maybe_debug_message {
            s
        } else {
            "None".to_owned()
        };
        let parameters: HashMap<u8, Value> = self.decode()?;

        Ok(OperationResponse {
            code,
            return_code,
            debug_message,
            parameters,
        })
    }
}

impl Decode<OperationRequest> for PhotonCursor<'_> {
    fn decode(&mut self) -> PhotonDecodeResult<OperationRequest> {
        let code: u8 = self.decode()?;
        let parameters: HashMap<u8, Value> = self.decode()?;
        Ok(OperationRequest { code, parameters })
    }
}

impl Decode<Vec<Box<Value>>> for PhotonCursor<'_> {
    fn decode(&mut self) -> PhotonDecodeResult<Vec<Box<Value>>> {
        let size: i16 = self.decode()?;
        if size <= 0 {
            return Err("Failed to decode Vec<Box<Value>>, unreasonable size");
        }
        let mut value = vec![];
        for _ in 0..size {
            let type_code: u8 = if let Ok(v) = self.decode() {
                v
            } else {
                break;
            };
            value.push(Box::new(self.typed_decode(type_code).unwrap()));
        }
        Ok(value)
    }
}

impl Decode<Vec<bool>> for PhotonCursor<'_> {
    fn decode(&mut self) -> PhotonDecodeResult<Vec<bool>> {
        let size: i16 = self.decode()?;
        if size <= 0 {
            return Err("Failed to decode Vec<bool>, unreasonable size");
        }
        let mut value = vec![];
        for _ in 0..size {
            value.push(self.decode()?);
        }
        Ok(value)
    }
}

impl TypedDecode for PhotonCursor<'_> {
    fn typed_decode(&mut self, type_code: u8) -> PhotonDecodeResult<Value> {
        match TypeCode::from(type_code) {
            TypeCode::Null => Ok(Value::None),
            TypeCode::Boolean => Ok(Value::Boolean(self.decode()?)),
            TypeCode::Byte => Ok(Value::Byte(self.decode()?)),
            TypeCode::Double => Ok(Value::Double(self.decode()?)),
            TypeCode::Float => Ok(Value::Float(self.decode()?)),
            TypeCode::Integer => Ok(Value::Integer(self.decode()?)),
            TypeCode::Long => Ok(Value::Long(self.decode()?)),
            TypeCode::Short => Ok(Value::Short(self.decode()?)),
            TypeCode::String => Ok(Value::String(self.decode()?)),
            TypeCode::StringArray => Ok(Value::StringArray(self.decode()?)),
            TypeCode::ByteArray => Ok(Value::ByteArray(self.decode()?)),
            TypeCode::Dictionary => Ok(Value::Dictionary(self.decode()?)),
            TypeCode::EventData => Ok(Value::EventData(self.decode()?)),
            TypeCode::OperationRequest => Ok(Value::OperationRequest(self.decode()?)),
            TypeCode::OperationResponse => Ok(Value::OperationResponse(self.decode()?)),
            TypeCode::BooleanArray => Ok(Value::BooleanArray(self.decode()?)),
            TypeCode::Array => Ok(Value::Array(self.decode()?)),
            TypeCode::ObjectArray => Ok(Value::ObjectArray(self.decode()?)),
            _ => Err("Failed to decode Value, unknown type code"),
        }
    }
}

impl Decode<Value> for PhotonCursor<'_> {
    fn decode(&mut self) -> PhotonDecodeResult<Value> {
        let type_code: u8 = self.decode()?;
        self.typed_decode(type_code)
    }
}

impl Decode<PhotonHeader> for PhotonCursor<'_> {
    fn decode(&mut self) -> PhotonDecodeResult<PhotonHeader> {
        let peer_id = self.decode()?;
        let crc_enabled = self.decode()?;
        let command_count = self.decode()?;
        let timestamp = self.decode()?;
        let challenge = self.decode()?;

        Ok(PhotonHeader {
            peer_id,
            crc_enabled,
            command_count,
            timestamp,
            challenge,
        })
    }
}

impl Decode<ReliableCommand> for PhotonCursor<'_> {
    fn decode(&mut self) -> PhotonDecodeResult<ReliableCommand> {
        let channel_id = self.decode()?;
        let flags = self.decode()?;
        let reserved_byte = self.decode()?;
        let length: u32 = self.decode()?;
        let reliable_sequence_number = self.decode()?;
        let msg_len = length - size_of::<ReliableCommand>() as u32;
        Ok(ReliableCommand {
            channel_id,
            flags,
            reserved_byte,
            msg_len,
            reliable_sequence_number,
        })
    }
}

impl Decode<UnreliableCommand> for PhotonCursor<'_> {
    fn decode(&mut self) -> PhotonDecodeResult<UnreliableCommand> {
        let mut reliable_command: ReliableCommand = self.decode()?;
        let unknown = self.decode()?;
        reliable_command.msg_len = reliable_command.msg_len - size_of::<u32>() as u32;
        Ok(UnreliableCommand {
            reliable_command,
            unknown,
        })
    }
}

impl Decode<ReliableFragment> for PhotonCursor<'_> {
    fn decode(&mut self) -> PhotonDecodeResult<ReliableFragment> {
        let mut reliable_command: ReliableCommand = self.decode()?;
        let sequence_number = self.decode()?;
        let fragment_count = self.decode()?;
        let fragment_number = self.decode()?;
        let total_length = self.decode()?;
        let operation_length = self.decode()?;

        reliable_command.msg_len = reliable_command.msg_len - (size_of::<u32>() * 5) as u32;
        let mut payload = vec![0u8; reliable_command.msg_len as usize];
        self.read_exact(&mut payload).unwrap();

        Ok(ReliableFragment {
            reliable_command,
            sequence_number,
            fragment_count,
            fragment_number,
            total_length,
            operation_length,
            payload,
        })
    }
}

impl Decode<Command> for PhotonCursor<'_> {
    fn decode(&mut self) -> PhotonDecodeResult<Command> {
        let cmd_type_id: u8 = self.decode()?;
        match cmd_type_id {
            4 => Ok(Command::LogOut),
            6 => Ok(Command::SendReliable(self.decode()?)),
            7 => Ok(Command::SendUnreliable(
                Decode::<UnreliableCommand>::decode(self)?.reliable_command,
            )),
            8 => Ok(Command::SendReliableFragment(self.decode()?)),
            _ => Ok(Command::SendReliable(self.decode()?)),
        }
    }
}

pub enum TypeCode {
    Unknown = 0,
    Null = 42,
    Dictionary = 68,
    StringArray = 97,
    Byte = 98,
    Double = 100,
    EventData = 101,
    Float = 102,
    Integer = 105,
    Short = 107,
    Long = 108,
    BooleanArray = 110,
    Boolean = 111,
    OperationResponse = 112,
    OperationRequest = 113,
    String = 115,
    ByteArray = 120,
    Array = 121,
    ObjectArray = 122,
}

impl From<u8> for TypeCode {
    fn from(v: u8) -> Self {
        match v {
            0 => TypeCode::Unknown,
            42 => TypeCode::Null,
            68 => TypeCode::Dictionary,
            97 => TypeCode::StringArray,
            98 => TypeCode::Byte,
            100 => TypeCode::Double,
            101 => TypeCode::EventData,
            102 => TypeCode::Float,
            105 => TypeCode::Integer,
            107 => TypeCode::Short,
            108 => TypeCode::Long,
            110 => TypeCode::BooleanArray,
            111 => TypeCode::Boolean,
            112 => TypeCode::OperationResponse,
            113 => TypeCode::OperationRequest,
            115 => TypeCode::String,
            120 => TypeCode::ByteArray,
            121 => TypeCode::Array,
            122 => TypeCode::ObjectArray,
            _ => TypeCode::Unknown,
        }
    }
}

impl std::ops::Index<usize> for Value {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Value::Array(v) => &v[index],
            _ => panic!("Non indexable type"),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(v) => write!(f, "{}", v),
            Value::Byte(v) => write!(f, "{}", v),
            Value::Integer(v) => write!(f, "{}", v),
            v => write!(f, "{:?}", v),
        }
    }
}
