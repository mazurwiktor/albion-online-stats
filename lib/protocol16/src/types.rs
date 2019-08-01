use std::collections::HashMap;

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
            _ => TypeCode::Unknown
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EventData {
    pub code: u8,
    pub parameters: HashMap<u8, Value>
}

#[derive(Clone, Debug, PartialEq)]
pub struct OperationResponse {
    pub code: u8,
    pub return_code: i16,
    pub debug_message: String,
    pub parameters: HashMap<u8, Value>
}

#[derive(Clone, Debug, PartialEq)]
pub struct OperationRequest {
    pub code: u8,
    pub parameters: HashMap<u8, Value>
}


#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    None,
    Dictionary(HashMap<String, Value>),
    StringArray(Vec<String>),
    Byte(u8),
    Double(f64),
    EventData(EventData),
    Float(f32),
    Integer(u32),
    Short(i16),
    Long(i64),
    BooleanArray(Vec<bool>),
    Boolean(bool),
    OperationResponse(OperationResponse),
    OperationRequest(OperationRequest),
    String(String),
    ByteArray(Vec<u8>),
    Array(Vec<Value>),
    ObjectArray(Vec<Value>)
}

impl std::ops::Index<usize> for Value {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Value::Array(v) => &v[index],
            _ => panic!("Non indexable type")
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(v) => write!(f, "{}", v),
            Value::Byte(v) => write!(f, "{}", v),
            Value::Integer(v) => write!(f, "{}", v),
            v => write!(f, "{:?}", v)
        }
    }
}
