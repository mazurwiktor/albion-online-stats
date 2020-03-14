use std::convert::From;

use crate::photon_messages::messages;

use super::events;
use super::id_cache::{DynIdToStaticId, StaticIdToName};
use super::player::DynamicId;

pub struct EventIntermediate<Msg> {
    dyn_id_to_static_id: DynIdToStaticId,
    static_id_to_name: StaticIdToName,
    message: Msg,
}

impl<Msg> EventIntermediate<Msg> {
    pub fn new(
        dyn_id_to_static_id: DynIdToStaticId,
        static_id_to_name: StaticIdToName,
        message: Msg,
    ) -> Self {
        Self {
            dyn_id_to_static_id,
            static_id_to_name,
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

    pub fn values(self) -> Option<Vec<events::Event>> {
        Some(self.value)
    }
}

macro_rules! try_from_dynamic {
    ($field_name:ident, $intermediate:expr) => {
        if let Some(static_id) = $intermediate
            .dyn_id_to_static_id
            .get(&DynamicId::from($intermediate.message.$field_name as u32))
        {
            *static_id
        } else {
            return Self::new(vec![]);
        }
    };
}

macro_rules! try_from_dynamic_with_name {
    ($field_name:ident, $intermediate:expr) => {
        if let Some(static_id) = $intermediate
            .dyn_id_to_static_id
            .get(&DynamicId::from($intermediate.message.$field_name as u32))
        {
            if let Some(player_name) = $intermediate.static_id_to_name.get(&static_id) {
                (*static_id, player_name.to_string())
            } else {
                return Self::new(vec![]);
            }
        } else {
            return Self::new(vec![]);
        }
    };
}

impl From<EventIntermediate<messages::NewCharacter>> for EventList {
    fn from(intermediate: EventIntermediate<messages::NewCharacter>) -> Self {
        let static_id = try_from_dynamic!(source, intermediate);
        Self::new(vec![
            events::Event::PlayerAppeared(events::Player {
                id: static_id,
                name: intermediate.message.character_name,
            }),
            events::Event::UpdateItems(events::Items {
                source: static_id,
                value: intermediate.message.items,
            }),
        ])
    }
}

impl From<EventIntermediate<messages::Join>> for EventList {
    fn from(intermediate: EventIntermediate<messages::Join>) -> Self {
        let static_id = try_from_dynamic!(source, intermediate);
        Self::new(vec![events::Event::MainPlayerAppeared(events::Player {
            id: static_id,
            name: intermediate.message.character_name,
        })])
    }
}

impl From<EventIntermediate<messages::HealthUpdate>> for EventList {
    fn from(intermediate: EventIntermediate<messages::HealthUpdate>) -> Self {
        let target_static_id = try_from_dynamic!(target, intermediate);

        if intermediate.message.value < 0.0 {
            return Self::new(vec![events::Event::DamageDone(events::Damage {
                source: target_static_id,
                target: intermediate
                    .dyn_id_to_static_id
                    .get(&DynamicId::from(intermediate.message.source as u32))
                    .map(|v| *v),
                value: f32::abs(intermediate.message.value),
            })]);
        }

        Self::new(vec![events::Event::HealthReceived(events::Damage {
            source: target_static_id,
            target: intermediate
                .dyn_id_to_static_id
                .get(&DynamicId::from(intermediate.message.source as u32))
                .map(|v| *v),
            value: intermediate.message.value,
        })])
    }
}

impl From<EventIntermediate<messages::RegenerationHealthChanged>> for EventList {
    fn from(intermediate: EventIntermediate<messages::RegenerationHealthChanged>) -> Self {
        let (static_id, name) = try_from_dynamic_with_name!(source, intermediate);

        match intermediate.message.regeneration_rate {
            Some(_) => Self::new(vec![events::Event::LeaveCombat(events::Player {
                id: static_id,
                name: name,
            })]),
            None => Self::new(vec![events::Event::EnterCombat(events::Player {
                id: static_id,
                name: name,
            })]),
        }
    }
}

impl From<EventIntermediate<messages::KnockedDown>> for EventList {
    fn from(intermediate: EventIntermediate<messages::KnockedDown>) -> Self {
        let (static_id, name) = try_from_dynamic_with_name!(source, intermediate);
        Self::new(vec![events::Event::LeaveCombat(events::Player {
            id: static_id,
            name: name,
        })])
    }
}

impl From<EventIntermediate<messages::UpdateFame>> for EventList {
    fn from(intermediate: EventIntermediate<messages::UpdateFame>) -> Self {
        let source_static_id = try_from_dynamic!(source, intermediate);
        Self::new(vec![events::Event::UpdateFame(events::Fame {
            source: source_static_id,
            value: intermediate.message.fame as f32 / 10000.0,
        })])
    }
}

impl From<EventIntermediate<messages::CharacterEquipmentChanged>> for EventList {
    fn from(intermediate: EventIntermediate<messages::CharacterEquipmentChanged>) -> Self {
        let source_static_id = try_from_dynamic!(source, intermediate);
        Self::new(vec![events::Event::UpdateItems(events::Items {
            source: source_static_id,
            value: intermediate.message.items,
        })])
    }
}
