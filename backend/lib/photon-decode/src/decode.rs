use std::collections::HashMap;
use std::io::{Cursor, Read};
use std::mem::size_of;

use bytes::Buf;

use crate::error::*;
use crate::layout::*;

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
                    return Err(PhotonDecodeError::from(concat!(
                        "Failed to decode ",
                        stringify!($type),
                        ", not enough bytes"
                    )));
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
            return Err(PhotonDecodeError::from(
                "Failed to decode bool, not enough bytes",
            ));
        };
        Ok(v != 0)
    }
}

impl Decode<String> for PhotonCursor<'_> {
    fn decode(&mut self) -> PhotonDecodeResult<String> {
        let size: i16 = self.decode()?;
        if size < 0 {
            return Err(PhotonDecodeError::from(
                "Failed to decode String, unreasonable size",
            ));
        }

        let mut local_buffer = vec![0; size as usize];
        if let Ok(_) = self.read_exact(&mut local_buffer[..]) {
            if let Ok(s) = String::from_utf8(local_buffer) {
                return Ok(s);
            }
        }

        Err(PhotonDecodeError::from(
            "Failed to decode String, not enough bytes",
        ))
    }
}

impl Decode<Vec<String>> for PhotonCursor<'_> {
    fn decode(&mut self) -> PhotonDecodeResult<Vec<String>> {
        let size: i16 = self.decode()?;
        if size < 0 {
            return Err(PhotonDecodeError::from(
                "Failed to decode String, unreasonable size",
            ));
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
        if size < 0 {
            return Err(PhotonDecodeError::from(
                "Failed to decode Vec<Value>, unreasonable size",
            ));
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
        if size < 0 {
            return Err(PhotonDecodeError::from(
                "Failed to decode HashMap<String, Value>, unreasonable size",
            ));
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
            if let (Ok(key), Ok(val)) = (key, val) {
                value.insert(format!("{}", key), val);
            }
        }

        Ok(value)
    }
}

impl Decode<HashMap<u8, Value>> for PhotonCursor<'_> {
    fn decode(&mut self) -> PhotonDecodeResult<HashMap<u8, Value>> {
        let size: i16 = self.decode()?;
        if size < 0 {
            return Err(PhotonDecodeError::from(
                "Failed to decode HashMap<u8, Value>, unreasonable size",
            ));
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
        if size < 0 {
            return Err(PhotonDecodeError::from(
                "Failed to decode Vec<Box<Value>>, unreasonable size",
            ));
        }
        let mut value = vec![];
        for _ in 0..size {
            let type_code: u8 = if let Ok(v) = self.decode() {
                v
            } else {
                break;
            };
            if let Ok(val) = self.typed_decode(type_code) {
                value.push(Box::new(val));
            }
        }
        Ok(value)
    }
}

impl Decode<Vec<bool>> for PhotonCursor<'_> {
    fn decode(&mut self) -> PhotonDecodeResult<Vec<bool>> {
        let size: i16 = self.decode()?;
        if size < 0 {
            return Err(PhotonDecodeError::from(
                "Failed to decode Vec<bool>, unreasonable size",
            ));
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
            TypeCode::None => Ok(Value::None),
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
            _ => Err(PhotonDecodeError::from(format!(
                "Failed to decode Value, unknown type code ({:#X})",
                type_code
            ))),
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
        let msg_len = length.checked_sub(size_of::<ReliableCommand>() as u32)
            .map_or(Err(PhotonDecodeError::from("Invalid ReliableCommand length")), |v| Ok(v))?;
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
        reliable_command.msg_len = reliable_command.msg_len.checked_sub(size_of::<u32>() as u32)
            .map_or(Err(PhotonDecodeError::from("Invalid UnreliableCommand length")), |v| Ok(v))?;
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

        reliable_command.msg_len = reliable_command.msg_len.checked_sub((size_of::<u32>() * 5) as u32)
            .map_or(Err(PhotonDecodeError::from("Invalid ReliableFragment length")), |v| Ok(v))?;
        let mut payload = vec![0u8; reliable_command.msg_len as usize];
        self.read_exact(&mut payload)
            .map_err(|e| PhotonDecodeError::from(format!("{}", e)))?;

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
    None = 0x00,
    Null = 0x2A,
    Dictionary = 0x44,
    StringArray = 0x61,
    Byte = 0x62,
    Double = 0x64,
    EventData = 0x65,
    Float = 0x66,
    Integer = 0x69,
    Short = 0x6B,
    Long = 0x6C,
    BooleanArray = 0x6E,
    Boolean = 0x6F,
    OperationResponse = 0x70,
    OperationRequest = 0x71,
    String = 0x73,
    ByteArray = 0x78,
    Array = 0x79,
    ObjectArray = 0x7A,
    Unknown,
}

impl From<u8> for TypeCode {
    fn from(v: u8) -> Self {
        match v {
            0x00 => TypeCode::None,
            0x2A => TypeCode::Null,
            0x44 => TypeCode::Dictionary,
            0x61 => TypeCode::StringArray,
            0x62 => TypeCode::Byte,
            0x64 => TypeCode::Double,
            0x65 => TypeCode::EventData,
            0x66 => TypeCode::Float,
            0x69 => TypeCode::Integer,
            0x6B => TypeCode::Short,
            0x6C => TypeCode::Long,
            0x6E => TypeCode::BooleanArray,
            0x6F => TypeCode::Boolean,
            0x70 => TypeCode::OperationResponse,
            0x71 => TypeCode::OperationRequest,
            0x73 => TypeCode::String,
            0x78 => TypeCode::ByteArray,
            0x79 => TypeCode::Array,
            0x7A => TypeCode::ObjectArray,
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
