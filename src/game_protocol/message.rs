use log::*;

use super::packet::Packet;
use protocol16::Parameters;
use protocol16::Value;

macro_rules! deserialize_number {
    ($val:expr, $index:expr, $name:expr) => {
        if let Some(p) = $val.get(&$index) {
            match p {
                Value::Short(v) => Some(*v as usize),
                Value::Integer(v) => Some(*v as usize),
                Value::Byte(v) => Some(*v as usize),
                _ => {
                    error!("Failed to deserialize {}", $name);
                    None
                } 
            }
        } else {
            error!("Index {} not found in {}", $index, $name);
            None
        }
    };
}

macro_rules! deserialize_string {
    ($val:expr, $index:expr, $name:expr) => {
        if let Some(p) = $val.get(&$index) {
            match p {
                Value::String(v) => Some(v.clone()),
                _ => {
                    error!("Failed to deserialize {}", $name);
                    None
                } 
            }
        } else {
            None
        }
    };
}

macro_rules! deserialize_float {
    ($val:expr, $index:expr, $name:expr) => {
        if let Some(p) = $val.get(&$index) {
            match p {
                Value::Float(v) => Some(*v as f32),
                _ => {
                    error!("Failed to deserialize {}", $name);
                    None
                } 
            }
        } else {
            None
        }
    };
}


#[derive(Debug)]
pub struct ChatSay {
    pub source: usize,
    pub source_name: String,
    pub text: String,
}

impl ChatSay {
    fn encode(val: Parameters) -> Option<Message> {
        let source = deserialize_number!(val, 0, "ChatSay::source")?;
        let source_name = deserialize_string!(val, 1, "ChatSay::source_name")?;
        let text = deserialize_string!(val, 2, "ChatSay::text")?;
        Some(Message::ChatSay(Self {
            source,
            source_name,
            text,
        }))
    }
}

#[derive(Debug)]
pub struct NewCharacter {
    pub source: usize,
    pub character_name: String,
    pub health: f32,
    pub max_health: f32,
    pub energy: f32,
    pub max_energy: f32,
}

impl NewCharacter {
    fn encode(val: Parameters) -> Option<Message> {
        let source = deserialize_number!(val, 0, "NewCharacter::source")?;

        let character_name = deserialize_string!(val, 1, "NewCharacter::character_name")?;

        let health = deserialize_float!(val, 18, "NewCharacter::health")?;
        let max_health = deserialize_float!(val, 19, "NewCharacter::max_health")?;

        let energy = deserialize_float!(val, 22, "NewCharacter::energy")?;
        let max_energy = deserialize_float!(val, 23, "NewCharacter::max_energy")?;

        Some(Message::NewCharacter(Self {
            source,
            character_name,
            health,
            max_health,
            energy,
            max_energy,
        }))
    }
}

#[derive(Debug)]
pub struct HealthUpdate {
    pub source: usize,
    pub target: usize,
    pub value: f32,
}

impl HealthUpdate {
    fn encode(val: Parameters) -> Option<Message> {
        let source = deserialize_number!(val, 0, "HealthUpdate::source")?;
        let target = deserialize_number!(val, 6, "HealthUpdate::target")?;
        let value = deserialize_float!(val, 2, "HealthUpdate::value")?;

        Some(Message::HealthUpdate(Self {
            source,
            target,
            value,
        }))
    }
}

#[derive(Debug)]
pub struct RegenerationHealthChanged {
    pub source: usize,
    pub health: f32,
    pub max_health: f32,
    pub regeneration_rate: Option<f32>,
}

impl RegenerationHealthChanged {
    fn encode(val: Parameters) -> Option<Message> {
        let source = deserialize_number!(val, 0, "RegenerationHealthChanged::source")?;
        let health = deserialize_float!(val, 2, "RegenerationHealthChanged::health")?;
        let max_health = deserialize_float!(val, 3, "RegenerationHealthChanged::max_health")?;
        let regeneration_rate = deserialize_float!(val, 4, "RegenerationHealthChanged::regeneration_rate");

        Some(Message::RegenerationHealthChanged(Self {
            source,
            health,
            max_health,
            regeneration_rate,
        }))
    }
}

#[derive(Debug)]
pub struct CharacterStats {
    pub source: usize,
    pub character_name: String,
    pub health: f32,
    pub max_health: f32,
    pub energy: f32,
    pub max_energy: f32,
}

impl CharacterStats {
    fn encode(val: Parameters) -> Option<Message> {
        if val.len() < 40 {
            return None;
        }

        let source = deserialize_number!(val, 0, "CharacterStats::source")?;

        let character_name = deserialize_string!(val, 2, "CharacterStats::character_name")?;

        let health = deserialize_float!(val, 11, "CharacterStats::health")?;
        let max_health = deserialize_float!(val, 12, "CharacterStats::max_health")?;

        let energy = deserialize_float!(val, 15, "CharacterStats::energy")?;
        let max_energy = deserialize_float!(val, 16, "CharacterStats::max_energy")?;


        Some(Message::CharacterStats(Self {
            source,
            character_name,
            health,
            max_health,
            energy,
            max_energy,
        }))
    }
}

#[derive(Debug)]
pub struct Leave {
    pub source: usize
}

impl Leave {
    fn encode(val: Parameters) -> Option<Message> {
        let source = deserialize_number!(val, 0, "Leave::source")?;

        Some(Message::Leave(Leave{source}))
    }
}

#[derive(Debug)]
pub enum Message {
    Leave(Leave),
    ChatSay(ChatSay),
    NewCharacter(NewCharacter),
    HealthUpdate(HealthUpdate),
    RegenerationHealthChanged(RegenerationHealthChanged),
    CharacterStats(CharacterStats)
}

impl Packet {
    pub fn decode(self) -> Option<Message> {
        match self.code {
            1 => Leave::encode(self.parameters),
            6 => HealthUpdate::encode(self.parameters),
            24 => NewCharacter::encode(self.parameters),
            63 => ChatSay::encode(self.parameters),
            79 => RegenerationHealthChanged::encode(self.parameters),
            1001 => CharacterStats::encode(self.parameters),
            _ => None,
        }
    }
}
