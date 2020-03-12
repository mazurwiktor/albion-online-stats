//! Holds state of the game gathered from photon decoded packets.
//! Note: This module is responsible for resolving all inconsistency between photon events and required game events.
//!
//! Inconsistency list:
//!     - player id is different in each zone

use crate::photon_messages;

use super::events;
use super::convert;
use super::convert::EventList;
use super::id_cache;
use super::party::{Party};
use super::player::{StaticId, DynamicId};
use super::unconsumed_messages::UnconsumedMessages;

#[derive(Debug, Default)]
pub struct World {
    cache: id_cache::IdCache,
    unconsumed_messages: UnconsumedMessages,
    main_player_id: Option<StaticId>,
    party: Party
}

impl World {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Transforms inconsistent game message into corresponding list of game events
    pub fn transform(
        &mut self,
        message: photon_messages::Message,
    ) -> Option<Vec<events::Event>> {
        match message {
            photon_messages::Message::NewCharacter(msg) => {
                let mut result = vec![];
                let dynamic_id = DynamicId::from(msg.source as u32);
                self.assign_dynamic_id(dynamic_id, &msg.character_name);

                let static_id = self.get_static_id(msg.source)?;

                result.append(&mut EventList::from(self.get_intermediate(static_id, msg)?).values());

                if let Some(messages) = self.unconsumed_messages.get_for_id(dynamic_id) {
                    for message in messages {
                        result.append(&mut self.transform(message).unwrap_or(vec![]));
                    }
                }

                Some(result)
            }
            photon_messages::Message::Join(msg) => {
                let mut result = vec![];
                let dynamic_id = DynamicId::from(msg.source as u32);

                self.assign_dynamic_id(dynamic_id, &msg.character_name);
                let static_id = self.get_static_id(msg.source)?;

                if self.main_player_id.is_none() {
                    result.push(events::Event::ZoneChange)
                }
                self.party.set_main_player_name(&msg.character_name);
                result.push(self.get_intermediate(static_id, msg)?.into());


                self.main_player_id = Some(static_id);

                if let Some(messages) = self.unconsumed_messages.get_for_id(dynamic_id) {
                    for message in messages {
                        result.append(&mut self.transform(message).unwrap_or(vec![]));
                    }
                }

                Some(result)
            }
            photon_messages::Message::Leave(msg) => {
                let static_id = self.get_static_id(msg.source)?;
                if let Some(main_player_id) = self.main_player_id {
                    if main_player_id == static_id {
                        return Some(vec![events::Event::ZoneChange]);
                    }
                }
                None
            }
            photon_messages::Message::HealthUpdate(msg) => {
                let static_id = self.get_static_id(msg.target)?;
                Some(vec![self.get_intermediate(static_id, msg)?.into()])
            }
            photon_messages::Message::RegenerationHealthChanged(msg) => {
                let static_id = self.get_static_id(msg.source)?;
                Some(vec![self.get_intermediate(static_id, msg)?.into()])
            }
            photon_messages::Message::KnockedDown(msg) => {
                let static_id = self.get_static_id(msg.source)?;
                Some(vec![self.get_intermediate(static_id, msg)?.into()])
            }
            photon_messages::Message::UpdateFame(msg) => {
                let static_id = self.get_static_id(msg.source)?;
                Some(vec![self.get_intermediate(static_id, msg)?.into()])
            }
            photon_messages::Message::CharacterEquipmentChanged(msg) => {
                if let Some(static_id) = self.get_static_id(msg.source) {
                    return Some(vec![self.get_intermediate(static_id, msg)?.into()]);
                }
                let id = DynamicId::from(msg.source as u32);
                self.unconsumed_messages.add(
                    photon_messages::messages::Message::CharacterEquipmentChanged(msg), id);
                None
            },
            photon_messages::Message::PartyInvitation(_) => {
                None
            },
            photon_messages::Message::PartyJoined(msg) => {
                let evt = self.party.joined(msg)?;
                Some(vec![evt])
            },
            photon_messages::Message::PartyDisbanded(_) => {
                let evt = self.party.disbanded()?;
                Some(vec![evt])
            },
            photon_messages::Message::PartyPlayerJoined(msg) => {
                let evt = self.party.single_player_joined(msg)?;
                Some(vec![evt])
            },
            photon_messages::Message::PartyChangedOrder(_) => {
                None
            },
            photon_messages::Message::PartyPlayerLeft(msg) => {
                let evt = self.party.player_left(msg)?;
                Some(vec![evt])
            },
            photon_messages::Message::PartyLeaderChanged(_) => {
                None
            },
            photon_messages::Message::PartyLootSettingChangedPlayer(_) => {
                None
            },
            photon_messages::Message::PartySilverGained(_) => {
                None
            },
            photon_messages::Message::PartyPlayerUpdated(_) => {
                None
            },
            photon_messages::Message::PartyInvitationPlayerBusy(_) => {
                None
            },
            photon_messages::Message::PartyMarkedObjectsUpdated(_) => {
                None
            },
            photon_messages::Message::PartyOnClusterPartyJoined(_) => {
                None
            },
            photon_messages::Message::PartySetRoleFlag(_) => {
                None
            },
        }
    }
    fn assign_dynamic_id(&mut self, id: DynamicId, name: &str) {
        self.cache.save(id, name);
    }

    fn get_static_id(&self, id: usize) -> Option<StaticId> {
        let dynamic_id = DynamicId::from(id as u32);
        self.cache.get_static_id(dynamic_id)  // queue message if static id isn't known
    }

    fn get_intermediate<Msg>(
        &self,
        static_id: StaticId,
        msg: Msg,
    ) -> Option<convert::EventIntermediate<Msg>> {
        Some(convert::EventIntermediate::new(
            static_id,
            self.cache.get_name(static_id)?,
            msg,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_contains {
        ($container:expr, $value:expr) => {
            assert!(
                format!("{:?}", $container).contains($value),
                format!("{:?} does not contains {}", $container, $value)
            );
        };
    }

    macro_rules! simulate_new_player {
        ($id:expr, $name:expr, $msg:ident) => {
            photon_messages::Message::$msg(photon_messages::messages::$msg {
                source: $id,
                character_name: $name.to_string(),
                ..Default::default()
            });
        };
    }

    #[test]
    /// photon_messages::NewCharacter -> events::Event::PlayerAppeared
    fn test_player_appeared() {
        let mut world = World::new();

        let game_message = simulate_new_player!(1, "TestCharacter", NewCharacter);

        assert!(world.transform(game_message.clone()).is_some());
        assert_contains!(
            world.transform(game_message.clone()),
            "PlayerAppeared"
        );
    }

    #[test]
    /// photon_messages::Join -> Event::PlayerAppeared
    fn test_main_player_appeared() {
        let mut world = World::new();

        let game_message = simulate_new_player!(1, "TestCharacter", Join);

        assert!(world.transform(game_message.clone()).is_some());
        assert_contains!(
            world.transform(game_message.clone()),
            "PlayerAppeared"
        );
    }

    #[test]
    /// photon_messages::HealthUpdate -> Event::DamageDone | Event::HealthReceived
    fn test_damage_done() {
        let mut world = World::new();

        let game_message = simulate_new_player!(1, "TestCharacter", Join);
        assert!(world.transform(game_message.clone()).is_some());
        assert_contains!(
            world.transform(game_message.clone()),
            "PlayerAppeared"
        );

        let target = 1;
        let game_message =
            photon_messages::Message::HealthUpdate(photon_messages::messages::HealthUpdate {
                target,
                value: -666.0,
                ..Default::default()
            });

        assert!(world.transform(game_message.clone()).is_some());
        assert_contains!(world.transform(game_message.clone()), "DamageDone");
        assert_contains!(world.transform(game_message.clone()), "666");

        let target = 1;
        let game_message =
            photon_messages::Message::HealthUpdate(photon_messages::messages::HealthUpdate {
                target,
                value: 666.0,
                ..Default::default()
            });

        assert!(world.transform(game_message.clone()).is_some());
        assert_contains!(
            world.transform(game_message.clone()),
            "HealthReceived"
        );
        assert_contains!(world.transform(game_message.clone()), "666");
    }

    #[test]
    /// photon_messages::Leave -> Event::ZoneChange
    fn test_zone_change() {
        let mut world = World::new();

        let game_message = photon_messages::Message::Leave(photon_messages::messages::Leave {
            source: 1,
            ..Default::default()
        });

        assert!(world.transform(game_message.clone()).is_none());

        let game_message = simulate_new_player!(1, "TestCharacter", Join);

        assert!(world.transform(game_message.clone()).is_some());
        assert_contains!(
            world.transform(game_message.clone()),
            "PlayerAppeared"
        );

        let game_message = photon_messages::Message::Leave(photon_messages::messages::Leave {
            source: 1,
            ..Default::default()
        });

        assert_contains!(world.transform(game_message.clone()), "ZoneChange");

        let game_message = simulate_new_player!(2, "TestCharacter", NewCharacter);
        assert!(world.transform(game_message.clone()).is_some());
        let game_message = photon_messages::Message::Leave(photon_messages::messages::Leave {
            source: 1,
            ..Default::default()
        });
        assert!(world.transform(game_message.clone()).is_none());
    }

    #[test]
    /// photon_messages::RegenerationHealthChanged.regeneration_rate -> Event::LeaveCombat
    fn test_combat_leave_via_regeneration_change() {
        let mut world = World::new();

        let game_message = simulate_new_player!(1, "TestCharacter", Join);
        assert!(world.transform(game_message.clone()).is_some());

        let game_message = photon_messages::Message::RegenerationHealthChanged(
            photon_messages::messages::RegenerationHealthChanged {
                source: 1,
                regeneration_rate: Some(1.0),
                ..Default::default()
            },
        );

        assert_contains!(world.transform(game_message.clone()), "LeaveCombat");
    }

    #[test]
    /// photon_messages::RegenerationHealthChanged.regeneration_rate -> Event::EnterCombat
    fn test_combat_enter_via_regeneration_change() {
        let mut world = World::new();

        let game_message = simulate_new_player!(1, "TestCharacter", Join);
        assert!(world.transform(game_message.clone()).is_some());

        let game_message = photon_messages::Message::RegenerationHealthChanged(
            photon_messages::messages::RegenerationHealthChanged {
                source: 1,
                regeneration_rate: None,
                ..Default::default()
            },
        );

        assert_contains!(world.transform(game_message.clone()), "EnterCombat");
    }

    #[test]
    /// photon_messages::KnockedDown -> Event::EnterCombat
    fn test_combat_enter_via_knockout() {
        let mut world = World::new();

        let game_message = simulate_new_player!(1, "TestCharacter", Join);
        assert!(world.transform(game_message.clone()).is_some());

        let game_message =
            photon_messages::Message::KnockedDown(photon_messages::messages::KnockedDown {
                source: 1,
                ..Default::default()
            });

        assert_contains!(world.transform(game_message.clone()), "LeaveCombat");
    }

    #[test]
    /// photon_messages::UpdateFame -> Event::FameUpdate
    fn test_fame_update() {
        let mut world = World::new();

        let game_message = simulate_new_player!(1, "TestCharacter", Join);
        assert!(world.transform(game_message.clone()).is_some());

        let game_message =
            photon_messages::Message::UpdateFame(photon_messages::messages::UpdateFame {
                source: 1,
                ..Default::default()
            });

        assert_contains!(world.transform(game_message.clone()), "UpdateFame");
    }
}
