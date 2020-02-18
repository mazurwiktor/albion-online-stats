use std::convert::From;

use crate::photon_messages::messages;

use super::events;
use super::player::{StaticId, PlayerName};

pub struct EventIntermediate<Msg> {
    id: StaticId,
    name: PlayerName,
    message: Msg,
}

impl<Msg> EventIntermediate<Msg> {
    pub fn new(
        id: StaticId,
        name: PlayerName,
        message: Msg,
    ) -> Self {
        Self {
            id,
            name,
            message,
        }
    }
}

pub struct EventList {
    value: Vec<events::Event>,
}

impl EventList {
    pub fn new(list: Vec<events::Event>) -> Self {
        Self { value: list }
    }

    pub fn values(self) -> Vec<events::Event> {
        self.value
    }
}

impl From<EventIntermediate<messages::NewCharacter>> for events::Event {
    fn from(intermediate: EventIntermediate<messages::NewCharacter>) -> Self {
        Self::PlayerAppeared(events::Player {
            id: intermediate.id,
            name: intermediate.message.character_name,
        })
    }
}

impl From<EventIntermediate<messages::NewCharacter>> for EventList {
    fn from(intermediate: EventIntermediate<messages::NewCharacter>) -> Self {
        Self::new(vec![
            events::Event::PlayerAppeared(events::Player {
                id: intermediate.id,
                name: intermediate.message.character_name,
            }),
            events::Event::UpdateItems(events::Items {
                source: intermediate.id,
                value: intermediate.message.items,
            }),
        ])
    }
}

impl From<EventIntermediate<messages::Join>> for events::Event {
    fn from(intermediate: EventIntermediate<messages::Join>) -> Self {
        Self::MainPlayerAppeared(events::Player {
            id: intermediate.id,
            name: intermediate.message.character_name,
        })
    }
}

impl From<EventIntermediate<messages::HealthUpdate>> for events::Event {
    fn from(intermediate: EventIntermediate<messages::HealthUpdate>) -> Self {
        if intermediate.message.value < 0.0 {
            return Self::DamageDone(events::Damage {
                source: intermediate.id,
                target: None,
                value: f32::abs(intermediate.message.value),
            });
        }
        Self::HealthReceived(events::Damage {
            source: intermediate.id,
            target: None,
            value: intermediate.message.value,
        })
    }
}


impl From<EventIntermediate<messages::RegenerationHealthChanged>> for events::Event {
    fn from(intermediate: EventIntermediate<messages::RegenerationHealthChanged>) -> Self {
        match intermediate.message.regeneration_rate {
            Some(_) => Self::LeaveCombat(events::Player {
                id: intermediate.id,
                name: intermediate.name.into(),
            }),
            None => Self::EnterCombat(events::Player {
                id: intermediate.id,
                name: intermediate.name.into(),
            }),
        }
    }
}

impl From<EventIntermediate<messages::KnockedDown>> for events::Event {
    fn from(intermediate: EventIntermediate<messages::KnockedDown>) -> Self {
        Self::LeaveCombat(events::Player {
            id: intermediate.id,
            name: intermediate.name.into(),
        })
    }
}

impl From<EventIntermediate<messages::UpdateFame>> for events::Event {
    fn from(intermediate: EventIntermediate<messages::UpdateFame>) -> Self {
        Self::UpdateFame(events::Fame {
            source: intermediate.id,
            value: intermediate.message.fame as f32 / 10000.0,
        })
    }
}

impl From<EventIntermediate<messages::CharacterEquipmentChanged>> for events::Event {
    fn from(intermediate: EventIntermediate<messages::CharacterEquipmentChanged>) -> Self {
        Self::UpdateItems(events::Items {
            source: intermediate.id,
            value: intermediate.message.items,
        })
    }
}
