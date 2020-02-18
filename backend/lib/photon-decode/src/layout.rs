use std::collections::HashMap;

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
    Object(Box<Value>),
    ObjectArray(Vec<Box<Value>>),
}

pub type Parameters = HashMap<u8, Value>;

#[derive(Clone, Debug, PartialEq)]
pub struct EventData {
    pub code: u8,
    pub parameters: Parameters,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OperationResponse {
    pub code: u8,
    pub return_code: i16,
    pub debug_message: String,
    pub parameters: Parameters,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OperationRequest {
    pub code: u8,
    pub parameters: Parameters,
}

#[derive(Debug, PartialEq)]
pub enum Message {
    Request(OperationRequest),
    Response(OperationResponse),
    Event(EventData),
}

#[derive(Debug)]
pub struct PhotonHeader {
    pub peer_id: i16,
    pub crc_enabled: bool,
    pub command_count: u8,
    pub timestamp: u32,
    pub challenge: u32,
}

#[derive(Debug, Clone)]
pub struct ReliableCommand {
    pub channel_id: u8,
    pub flags: u8,
    pub reserved_byte: u8,
    pub msg_len: u32,
    pub reliable_sequence_number: u32,
}

pub struct UnreliableCommand {
    pub reliable_command: ReliableCommand,
    pub unknown: u32,
}

#[derive(Debug, Clone)]
pub struct ReliableFragment {
    pub reliable_command: ReliableCommand,
    pub sequence_number: u32,
    pub fragment_count: u32,
    pub fragment_number: u32,
    pub total_length: u32,
    pub operation_length: u32,
    pub payload: Vec<u8>,
}

pub enum Command {
    LogOut,
    SendUnreliable(ReliableCommand),
    SendReliable(ReliableCommand),
    SendReliableFragment(ReliableFragment),
}
