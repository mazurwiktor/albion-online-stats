use std::convert::From;

use crate::game_messages::messages;

use super::events;
use super::id_cache;

pub struct EventIntermediate<Msg> {
    id: id_cache::StaticId,
    name: id_cache::PlayerName,
    message: Msg,
}

impl<Msg> EventIntermediate<Msg> {
    pub fn new(
        id: id_cache::StaticId,
        name: id_cache::PlayerName,
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
    value: Vec<events::Events>,
}

impl EventList {
    pub fn new(list: Vec<events::Events>) -> Self {
        Self { value: list }
    }

    pub fn values(self) -> Vec<events::Events> {
        self.value
    }
}

impl From<EventIntermediate<messages::NewCharacter>> for events::Events {
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
            events::Events::PlayerAppeared(events::Player {
                id: intermediate.id,
                name: intermediate.message.character_name,
            }),
            events::Events::UpdateItems(events::Items {
                source: intermediate.id,
                value: intermediate.message.items,
            }),
        ])
    }
}

impl From<EventIntermediate<messages::Join>> for events::Events {
    fn from(intermediate: EventIntermediate<messages::Join>) -> Self {
        Self::PlayerAppeared(events::Player {
            id: intermediate.id,
            name: intermediate.message.character_name,
        })
    }
}

impl From<EventIntermediate<messages::HealthUpdate>> for events::Events {
    fn from(intermediate: EventIntermediate<messages::HealthUpdate>) -> Self {
        if intermediate.message.value < 0.0 {
            return Self::DamageDone(events::Damage {
                source: intermediate.id,
                target: None,
                value: intermediate.message.value,
            });
        }
        Self::HealthReceived(events::Damage {
            source: intermediate.id,
            target: None,
            value: intermediate.message.value,
        })
    }
}


impl From<EventIntermediate<messages::RegenerationHealthChanged>> for events::Events {
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

impl From<EventIntermediate<messages::KnockedDown>> for events::Events {
    fn from(intermediate: EventIntermediate<messages::KnockedDown>) -> Self {
        Self::LeaveCombat(events::Player {
            id: intermediate.id,
            name: intermediate.name.into(),
        })
    }
}

impl From<EventIntermediate<messages::UpdateFame>> for events::Events {
    fn from(intermediate: EventIntermediate<messages::UpdateFame>) -> Self {
        Self::UpdateFame(events::Fame {
            source: intermediate.id,
            value: intermediate.message.fame as f32 / 10000.0,
        })
    }
}

impl From<EventIntermediate<messages::CharacterEquipmentChanged>> for events::Events {
    fn from(intermediate: EventIntermediate<messages::CharacterEquipmentChanged>) -> Self {
        Self::UpdateItems(events::Items {
            source: intermediate.id,
            value: intermediate.message.items,
        })
    }
}
