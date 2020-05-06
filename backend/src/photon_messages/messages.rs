use log::*;
use crate::photon_messages::Items;
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

#[allow(unused_macros)]
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


macro_rules! decode_vec_of_number_vec {
    ($val:expr, $index:expr, $name:expr) => {
        if let Some(p) = $val.get(&$index) {
            match p {
                Value::Array(v) => {
                    let mut ret = vec![];
                    for i in v {
                        let mut params = Parameters::new();
                        params.insert(0, i.clone());
                        let item = decode_number_vec!(params, 0, $name)?;
                        ret.push(item);
                    }

                    Some(ret)
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


#[derive(Debug, Clone, PartialEq, Default)]
pub struct Leave {
    pub source: usize,

}

impl Leave {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("Leave parameters: {:?}", val);
        let source = decode_number!(val, 0, "Leave::source")?;


        Some(Message::Leave(Leave { source,  }))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HealthUpdate {
    pub source: usize,
    pub target: usize,
    pub value: f32,

}

impl HealthUpdate {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("HealthUpdate parameters: {:?}", val);
        let source = decode_number!(val, 0, "HealthUpdate::source")?;
        let target = decode_number!(val, 6, "HealthUpdate::target")?;
        let value = decode_float!(val, 2, "HealthUpdate::value")?;


        Some(Message::HealthUpdate(HealthUpdate { source, target, value,  }))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
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
        let health = decode_float!(val, 2, "RegenerationHealthChanged::health")?;
        let max_health = decode_float!(val, 3, "RegenerationHealthChanged::max_health")?;
        let regeneration_rate = decode_float!(val, 4, "RegenerationHealthChanged::regeneration_rate");


        Some(Message::RegenerationHealthChanged(RegenerationHealthChanged { source, health, max_health, regeneration_rate,  }))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct KnockedDown {
    pub source: usize,
    pub target: usize,
    pub target_name: String,

}

impl KnockedDown {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("KnockedDown parameters: {:?}", val);
        let source = decode_number!(val, 0, "KnockedDown::source")?;
        let target = decode_number!(val, 3, "KnockedDown::target")?;
        let target_name = decode_string!(val, 4, "KnockedDown::target_name")?;


        Some(Message::KnockedDown(KnockedDown { source, target, target_name,  }))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
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
        let item_array = decode_number_vec!(val, 33, "NewCharacter::item_array")?;
        let items = item_array.into();


        Some(Message::NewCharacter(NewCharacter { source, character_name, health, max_health, energy, max_energy, items,  }))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UpdateFame {
    pub source: usize,
    pub fame: usize,

}

impl UpdateFame {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("UpdateFame parameters: {:?}", val);
        let source = decode_number!(val, 0, "UpdateFame::source")?;
        let fame = decode_number!(val, 2, "UpdateFame::fame")?;


        Some(Message::UpdateFame(UpdateFame { source, fame,  }))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CharacterEquipmentChanged {
    pub source: usize,
    pub items: Items,

}

impl CharacterEquipmentChanged {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("CharacterEquipmentChanged parameters: {:?}", val);
        let source = decode_number!(val, 0, "CharacterEquipmentChanged::source")?;
        let item_array = decode_number_vec!(val, 2, "CharacterEquipmentChanged::item_array")?;
        let items = item_array.into();


        Some(Message::CharacterEquipmentChanged(CharacterEquipmentChanged { source, items,  }))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PartyInvitation {

}

impl PartyInvitation {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("PartyInvitation parameters: {:?}", val);


        Some(Message::PartyInvitation(PartyInvitation {  }))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PartyJoined {
    pub party_id: usize,
    pub party_structures: Vec<Vec<u32>>,
    pub character_names: Vec<String>,

}

impl PartyJoined {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("PartyJoined parameters: {:?}", val);
        let party_id = decode_number!(val, 0, "PartyJoined::party_id")?;
        let party_structures = decode_vec_of_number_vec!(val, 4, "PartyJoined::party_structures")?;
        let character_names = decode_string_vec!(val, 5, "PartyJoined::character_names")?;


        Some(Message::PartyJoined(PartyJoined { party_id, party_structures, character_names,  }))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PartyDisbanded {

}

impl PartyDisbanded {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("PartyDisbanded parameters: {:?}", val);


        Some(Message::PartyDisbanded(PartyDisbanded {  }))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PartyPlayerJoined {
    pub party_id: usize,
    pub party_structure: Vec<u32>,
    pub name: String,

}

impl PartyPlayerJoined {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("PartyPlayerJoined parameters: {:?}", val);
        let party_id = decode_number!(val, 0, "PartyPlayerJoined::party_id")?;
        let party_structure = decode_number_vec!(val, 1, "PartyPlayerJoined::party_structure")?;
        let name = decode_string!(val, 2, "PartyPlayerJoined::name")?;


        Some(Message::PartyPlayerJoined(PartyPlayerJoined { party_id, party_structure, name,  }))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PartyChangedOrder {

}

impl PartyChangedOrder {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("PartyChangedOrder parameters: {:?}", val);


        Some(Message::PartyChangedOrder(PartyChangedOrder {  }))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PartyPlayerLeft {
    pub party_id: usize,
    pub party_structure: Vec<u32>,

}

impl PartyPlayerLeft {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("PartyPlayerLeft parameters: {:?}", val);
        let party_id = decode_number!(val, 0, "PartyPlayerLeft::party_id")?;
        let party_structure = decode_number_vec!(val, 1, "PartyPlayerLeft::party_structure")?;


        Some(Message::PartyPlayerLeft(PartyPlayerLeft { party_id, party_structure,  }))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PartyLeaderChanged {

}

impl PartyLeaderChanged {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("PartyLeaderChanged parameters: {:?}", val);


        Some(Message::PartyLeaderChanged(PartyLeaderChanged {  }))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PartyLootSettingChangedPlayer {

}

impl PartyLootSettingChangedPlayer {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("PartyLootSettingChangedPlayer parameters: {:?}", val);


        Some(Message::PartyLootSettingChangedPlayer(PartyLootSettingChangedPlayer {  }))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PartySilverGained {

}

impl PartySilverGained {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("PartySilverGained parameters: {:?}", val);


        Some(Message::PartySilverGained(PartySilverGained {  }))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PartyPlayerUpdated {

}

impl PartyPlayerUpdated {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("PartyPlayerUpdated parameters: {:?}", val);


        Some(Message::PartyPlayerUpdated(PartyPlayerUpdated {  }))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PartyInvitationPlayerBusy {

}

impl PartyInvitationPlayerBusy {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("PartyInvitationPlayerBusy parameters: {:?}", val);


        Some(Message::PartyInvitationPlayerBusy(PartyInvitationPlayerBusy {  }))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PartyMarkedObjectsUpdated {

}

impl PartyMarkedObjectsUpdated {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("PartyMarkedObjectsUpdated parameters: {:?}", val);


        Some(Message::PartyMarkedObjectsUpdated(PartyMarkedObjectsUpdated {  }))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PartyOnClusterPartyJoined {

}

impl PartyOnClusterPartyJoined {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("PartyOnClusterPartyJoined parameters: {:?}", val);


        Some(Message::PartyOnClusterPartyJoined(PartyOnClusterPartyJoined {  }))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PartySetRoleFlag {

}

impl PartySetRoleFlag {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("PartySetRoleFlag parameters: {:?}", val);


        Some(Message::PartySetRoleFlag(PartySetRoleFlag {  }))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Join {
    pub source: usize,
    pub character_name: String,
    pub health: f32,
    pub max_health: f32,
    pub energy: f32,
    pub max_energy: f32,

}

impl Join {
    pub fn parse(val: Parameters) -> Option<Message> {
        info!("Join parameters: {:?}", val);
        let source = decode_number!(val, 0, "Join::source")?;
        let character_name = decode_string!(val, 2, "Join::character_name")?;
        let health = decode_float!(val, 11, "Join::health")?;
        let max_health = decode_float!(val, 12, "Join::max_health")?;
        let energy = decode_float!(val, 15, "Join::energy")?;
        let max_energy = decode_float!(val, 16, "Join::max_energy")?;


        Some(Message::Join(Join { source, character_name, health, max_health, energy, max_energy,  }))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Leave(Leave),
    HealthUpdate(HealthUpdate),
    RegenerationHealthChanged(RegenerationHealthChanged),
    KnockedDown(KnockedDown),
    NewCharacter(NewCharacter),
    UpdateFame(UpdateFame),
    CharacterEquipmentChanged(CharacterEquipmentChanged),
    PartyInvitation(PartyInvitation),
    PartyJoined(PartyJoined),
    PartyDisbanded(PartyDisbanded),
    PartyPlayerJoined(PartyPlayerJoined),
    PartyChangedOrder(PartyChangedOrder),
    PartyPlayerLeft(PartyPlayerLeft),
    PartyLeaderChanged(PartyLeaderChanged),
    PartyLootSettingChangedPlayer(PartyLootSettingChangedPlayer),
    PartySilverGained(PartySilverGained),
    PartyPlayerUpdated(PartyPlayerUpdated),
    PartyInvitationPlayerBusy(PartyInvitationPlayerBusy),
    PartyMarkedObjectsUpdated(PartyMarkedObjectsUpdated),
    PartyOnClusterPartyJoined(PartyOnClusterPartyJoined),
    PartySetRoleFlag(PartySetRoleFlag),
    Join(Join),
}

pub fn into_game_message(photon_message: photon_decode::Message) -> Option<Message> {
    debug!("Raw photon : {:?}", photon_message);
    match photon_message {
        photon_decode::Message::Event(photon_decode::EventData{
            code: 1,
            parameters
        }) => {
            match parameters.get(&252u8) {
                Some(photon_decode::Value::Short(1)) => Leave::parse(parameters),
                Some(photon_decode::Value::Short(6)) => HealthUpdate::parse(parameters),
                Some(photon_decode::Value::Short(81)) => RegenerationHealthChanged::parse(parameters),
                Some(photon_decode::Value::Short(154)) => KnockedDown::parse(parameters),
                Some(photon_decode::Value::Short(25)) => NewCharacter::parse(parameters),
                Some(photon_decode::Value::Short(72)) => UpdateFame::parse(parameters),
                Some(photon_decode::Value::Short(80)) => CharacterEquipmentChanged::parse(parameters),
                Some(photon_decode::Value::Short(212)) => PartyInvitation::parse(parameters),
                Some(photon_decode::Value::Short(213)) => PartyJoined::parse(parameters),
                Some(photon_decode::Value::Short(214)) => PartyDisbanded::parse(parameters),
                Some(photon_decode::Value::Short(215)) => PartyPlayerJoined::parse(parameters),
                Some(photon_decode::Value::Short(216)) => PartyChangedOrder::parse(parameters),
                Some(photon_decode::Value::Short(217)) => PartyPlayerLeft::parse(parameters),
                Some(photon_decode::Value::Short(218)) => PartyLeaderChanged::parse(parameters),
                Some(photon_decode::Value::Short(219)) => PartyLootSettingChangedPlayer::parse(parameters),
                Some(photon_decode::Value::Short(220)) => PartySilverGained::parse(parameters),
                Some(photon_decode::Value::Short(221)) => PartyPlayerUpdated::parse(parameters),
                Some(photon_decode::Value::Short(222)) => PartyInvitationPlayerBusy::parse(parameters),
                Some(photon_decode::Value::Short(223)) => PartyMarkedObjectsUpdated::parse(parameters),
                Some(photon_decode::Value::Short(224)) => PartyOnClusterPartyJoined::parse(parameters),
                Some(photon_decode::Value::Short(225)) => PartySetRoleFlag::parse(parameters),

                _ => None
            }
        },
        photon_decode::Message::Request(photon_decode::OperationRequest{
            code: 1,
            parameters
        }) => {
            match parameters.get(&253u8) {
                _ => None
            }
        },
        photon_decode::Message::Response(photon_decode::OperationResponse{
            code: 1,
            parameters,
            return_code: _,
            debug_message: _
        }) => {
            match parameters.get(&253u8) {
                Some(photon_decode::Value::Short(2)) => Join::parse(parameters),

                _ => None
            }
        },
        _ => None
    }
}
