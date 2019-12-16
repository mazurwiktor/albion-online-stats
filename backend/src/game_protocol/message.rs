use log::*;

use super::Items;
use photon_decode::Parameters;
use photon_decode::Value;

macro_rules! decode_number {
    ($val:expr, $index:expr, $name:expr) => {
        if let Some(p) = $val.get(&$index) {
            match p {
                Value::Short(v) => Some(*v as usize),
                Value::Integer(v) => Some(*v as usize),
                Value::Byte(v) => Some(*v as usize),
                _ => {
                    error!("Failed to decode {}", $name);
                    None
                }
            }
        } else {
            error!("Index {} not found in {}", $index, $name);
            None
        }
    };
}

macro_rules! decode_string {
    ($val:expr, $index:expr, $name:expr) => {
        if let Some(p) = $val.get(&$index) {
            match p {
                Value::String(v) => Some(v.clone()),
                _ => {
                    error!("Failed to decode {}", $name);
                    None
                }
            }
        } else {
            None
        }
    };
}

macro_rules! decode_string_vec {
    ($val:expr, $index:expr, $name:expr) => {
        if let Some(p) = $val.get(&$index) {
            match p {
                Value::Array(arr) => {
                    let mut ret = vec![];
                    for v in arr {
                        if let Value::String(s) = v {
                            ret.push(s.clone());
                        }
                    }

                    Some(ret)
                }
                _ => {
                    error!("Failed to decode {}", $name);
                    None
                }
            }
        } else {
            None
        }
    };
}

macro_rules! decode_number_vec {
    ($val:expr, $index:expr, $name:expr) => {
        if let Some(p) = $val.get(&$index) {
            match p {
                Value::Array(arr) => {
                    let mut ret = vec![];
                    for v in arr {
                        match v {
                            Value::Short(v) => {
                                ret.push(*v as u32);
                            },
                            Value::Byte(v) => {
                                ret.push(*v as u32);
                            },
                            _ => {}
                        }
                    }

                    Some(ret)
                },
                Value::ByteArray(v) => {
                    Some(v.iter().map(|b| *b as u32).collect::<Vec<u32>>())
                },
                _ => {
                    error!("Failed to decode {}", $name);
                    None
                }
            }
        } else {
            None
        }
    };
}

macro_rules! decode_float {
    ($val:expr, $index:expr, $name:expr) => {
        if let Some(p) = $val.get(&$index) {
            match p {
                Value::Float(v) => Some(*v as f32),
                _ => {
                    error!("Failed to decode {}", $name);
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
    pub fn parse(val: Parameters) -> Option<Message> {
        let source = decode_number!(val, 0, "ChatSay::source")?;
        let source_name = decode_string!(val, 1, "ChatSay::source_name")?;
        let text = decode_string!(val, 2, "ChatSay::text")?;
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
    pub items: Items,
}

impl NewCharacter {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("NewCharacter parameters: {:?}", val);
        let source = decode_number!(val, 0, "NewCharacter::source")?;

        let character_name = decode_string!(val, 1, "NewCharacter::character_name")?;

        let health = decode_float!(val, 18, "NewCharacter::health")?;
        let max_health = decode_float!(val, 19, "NewCharacter::max_health")?;

        let energy = decode_float!(val, 22, "NewCharacter::energy")?;
        let max_energy = decode_float!(val, 23, "NewCharacter::max_energy")?;
        let item_array = decode_number_vec!(val, 33, "NewCharacter::items")?;
        let items = Items::from(&item_array);

        Some(Message::NewCharacter(Self {
            source,
            character_name,
            health,
            max_health,
            energy,
            max_energy,
            items,
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
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("HealthUpdate parameters: {:?}", val);
        let source = decode_number!(val, 0, "HealthUpdate::source")?;
        let target = decode_number!(val, 7, "HealthUpdate::target")?;
        let value = decode_float!(val, 3, "HealthUpdate::value")?;

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
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("RegenerationHealthChanged parameters: {:?}", val);
        let source = decode_number!(val, 0, "RegenerationHealthChanged::source")?;
        let health = decode_float!(val, 3, "RegenerationHealthChanged::health")?;
        let max_health = decode_float!(val, 4, "RegenerationHealthChanged::max_health")?;
        let regeneration_rate =
            decode_float!(val, 5, "RegenerationHealthChanged::regeneration_rate");

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
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("CharacterStats parameters: {:?}", val);

        let source = decode_number!(val, 0, "CharacterStats::source")?;

        let character_name = decode_string!(val, 2, "CharacterStats::character_name")?;

        let health = decode_float!(val, 11, "CharacterStats::health")?;
        let max_health = decode_float!(val, 12, "CharacterStats::max_health")?;

        let energy = decode_float!(val, 15, "CharacterStats::energy")?;
        let max_energy = decode_float!(val, 16, "CharacterStats::max_energy")?;

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
    pub source: usize,
}

impl Leave {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("Leave parameters: {:?}", val);
        let source = decode_number!(val, 0, "Leave::source")?;

        Some(Message::Leave(Leave { source }))
    }
}

#[derive(Debug)]
pub struct Died {
    pub source: usize,
    pub target: usize,
    pub target_name: String,
}

impl Died {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("Died parameters: {:?}", val);
        let source = decode_number!(val, 0, "Died::source")?;
        let target = decode_number!(val, 3, "Died::target")?;
        let target_name = decode_string!(val, 4, "Died::target_name")?;

        Some(Message::Died(Died {
            source,
            target,
            target_name,
        }))
    }
}

#[derive(Debug)]
pub struct PartyNew {
    pub source: usize,
    pub players: Vec<String>,
}

impl PartyNew {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("PartyNew parameters: {:?}", val);
        let source = decode_number!(val, 0, "PartyNew::source")?;
        let players = decode_string_vec!(val, 5, "PartyNew::players")?;

        Some(Message::PartyNew(PartyNew { source, players }))
    }
}

#[derive(Debug)]
pub struct PartyJoin {
    pub source: usize,
    pub target_name: String,
}

impl PartyJoin {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("PartyJoin parameters: {:?}", val);
        let source = decode_number!(val, 0, "PartyJoin::source")?;
        let target_name = decode_string!(val, 2, "PartyJoin::target_name")?;

        Some(Message::PartyJoin(PartyJoin {
            source,
            target_name,
        }))
    }
}

#[derive(Debug)]
pub struct PartyDisbanded {
    pub source: usize,
}

impl PartyDisbanded {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("PartyDisbanded parameters: {:?}", val);
        let source = decode_number!(val, 0, "PartyDisbanded::source")?;

        Some(Message::PartyDisbanded(PartyDisbanded { source }))
    }
}

#[derive(Debug)]
pub struct FameUpdate {
    pub source: usize,
    pub fame: f32,
}

impl FameUpdate {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("FameUpdate parameters: {:?}", val);
        let source = decode_number!(val, 0, "FameUpdate::source")?;
        let raw_fame = decode_number!(val, 2, "FameUpdate::fame")?;
        let fame = raw_fame as f32 / 10000.0;

        Some(Message::FameUpdate(FameUpdate { source, fame }))
    }
}

#[derive(Debug)]
pub struct PlayerItems {
    pub source: usize,
    pub items: Items,
}

impl PlayerItems {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("PlayerItems parameters: {:?}", val);
        let source = decode_number!(val, 0, "PlayerItems::source")?;
        let item_array = decode_number_vec!(val, 3, "PlayerItems::items")?;
        let items = Items::from(&item_array);

        Some(Message::PlayerItems(PlayerItems { source, items }))
    }
}

#[derive(Debug)]
pub enum Message {
    FameUpdate(FameUpdate),
    PlayerItems(PlayerItems),
    Leave(Leave),
    ChatSay(ChatSay),
    NewCharacter(NewCharacter),
    HealthUpdate(HealthUpdate),
    RegenerationHealthChanged(RegenerationHealthChanged),
    CharacterStats(CharacterStats),
    Died(Died),
    PartyNew(PartyNew),
    PartyJoin(PartyJoin),
    PartyDisbanded(PartyDisbanded),
}