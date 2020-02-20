extern crate chrono;
extern crate timer;

use std::collections::HashMap;

use log::*;

mod player;
mod traits;
mod types;
mod session;

use crate::game::Events;
use player::Player;

pub use super::photon_messages;
pub use traits::*;
pub use types::*;

use session::Session;

#[derive(Default)]
pub struct MeterConfig {}

#[derive(Default)]
pub struct Meter {
    zone_history: PlayerStatisticsVec,
    zone_session: Option<Session>,
    last_fight_session: Session,
    unconsumed_items: HashMap<usize, photon_messages::Items>,
    config: MeterConfig,
}

impl Meter {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn configure(&mut self, config: MeterConfig) {
        self.config = config;
    }

    pub fn consume(&mut self, event: Events) -> Option<()> {
        match event {
            Events::MainPlayerAppeared(e) => {
                let id = u32::from(e.id) as usize;
                self.register_player(&e.name, id, true);

                if let Some(items) = self.unconsumed_items.get(&id) {
                    let i = items.clone();
                    self.register_item_update(id, &i);
                    self.unconsumed_items.remove(&id);
                }
            },
            Events::PlayerAppeared(e) => {
                let id = u32::from(e.id) as usize;
                self.register_player(&e.name, id, false);

                if let Some(items) = self.unconsumed_items.get(&id) {
                    let i = items.clone();
                    self.register_item_update(id, &i);
                    self.unconsumed_items.remove(&id);
                }
            },
            Events::ZoneChange => {
                self.new_session();
            },
            Events::UpdateItems(e) => {
                let player_id = u32::from(e.source) as usize;
                let items = e.value;
                let mut consumed = false;
                for player in self.get_players_in_the_zone(player_id).unwrap_or(vec![]) {
                    player.items_update(&items);
                    consumed = true;
                }
        
                if !consumed {
                    info!("Storing not consumed items for player id: {}", player_id);
                    self.unconsumed_items.insert(player_id, items.clone());
                }
            }
            Events::DamageDone(e) => {
                let player_id = u32::from(e.source) as usize;
                for player in self.get_players_in_the_zone(player_id)? {
                    player.register_damage_dealt(f32::abs(e.value));
                }
            }
            Events::EnterCombat(e) => {
                let player_id = u32::from(e.id) as usize;
                if self.combat_state() == CombatState::OutOfCombat {
                    self.last_fight_session = Session::from(&self.last_fight_session);
                }
                for player in self.get_players_in_the_zone(player_id)? {
                    player.enter_combat();
                }
            }
            Events::LeaveCombat(e) => {
                let player_id = u32::from(e.id) as usize;
                for player in self.get_players_in_the_zone(player_id)? {
                    player.leave_combat();
                }
            }
            Events::UpdateFame(e) => {
                let player_id = u32::from(e.source) as usize;
                for player in self.get_players_in_the_zone(player_id)? {
                    player.register_fame_gain(e.value);
                }
            }
            _ => {}
        }

        Some(())
    }

    fn get_players_in_the_zone(
        &mut self,
        player_id: usize,
    ) -> Option<Vec<&mut Player>> {
        let (zone, last_fight) = self.sessions_mut()?;
        let las_fight_session_player = last_fight.get_player_by_id(player_id)?;
        let zone_player = zone.get_player_by_id(player_id)?;
        Some(vec![zone_player, las_fight_session_player])
    }

    fn register_player(&mut self, name: &str, id: usize, main: bool) {
        if self.zone_session.is_none() {
            info!(
                "New player ({}) registered without session, creating new session",
                name
            );
            self.new_session();
        }

        if self
            .get_players_in_the_zone(id)
            .unwrap_or(vec![])
            .is_empty()
        {
            self.add_player(name, id, main);
        }
    }

    fn register_item_update(
        &mut self,
        player_id: usize,
        items: &photon_messages::Items,
    ) -> Option<()> {
        let mut consumed = false;
        for player in self.get_players_in_the_zone(player_id).unwrap_or(vec![]) {
            player.items_update(items);
            consumed = true;
        }

        if !consumed {
            info!("Storing not consumed items for player id: {}", player_id);
            self.unconsumed_items.insert(player_id, items.clone());
        }

        Some(())
    }

    fn stats_filter(&self, _player: &(&String, &Player)) -> bool {
        true
    }

    fn zone_session_mut(&mut self) -> Option<&mut Session> {
        match &mut self.zone_session {
            Some(s) => Some(s),
            None => None,
        }
    }

    fn zone_session(&self) -> Option<&Session> {
        match &self.zone_session {
            Some(s) => Some(s),
            None => None,
        }
    }

    fn sessions_mut(&mut self) -> Option<(&mut Session, &mut Session)> {
        match &mut self.zone_session {
            Some(s) => Some((s, &mut self.last_fight_session)),
            None => None,
        }
    }

    fn add_player(&mut self, name: &str, id: usize, main: bool) -> Option<()> {
        let session = self.zone_session_mut()?;
        session.add_player(name, id, main);
        self.last_fight_session.add_player(name, id, main);

        Some(())
    }

    fn new_session(&mut self) {
        if let Some(zone) = self.zone_session() {
            self.zone_history = PlayerStatisticsVec::merged(
                &self.zone_history,
                &zone.stats(|p| self.stats_filter(p)),
            );
        }
        self.zone_session = Some(Session::new());
        self.last_fight_session = Session::new();
    }

    fn combat_state(&self) -> CombatState {
        if self
            .last_fight_session
            .players()
            .iter()
            .any(|p| p.combat_state() == CombatState::InCombat)
        {
            return CombatState::InCombat;
        }

        CombatState::OutOfCombat
    }
}

impl LastFightStats for Meter {
    fn last_fight_stats(&self) -> Option<PlayerStatisticsVec> {
        let last_session = &self.last_fight_session;
        Some(last_session.stats(|p| self.stats_filter(p)))
    }

    fn reset_last_fight_stats(&mut self) -> Option<()> {
        self.last_fight_session = Session::from(&self.last_fight_session);
        info!("Reset: last fight");
        Some(())
    }
}

impl ZoneStats for Meter {
    fn zone_stats(&self) -> Option<PlayerStatisticsVec> {
        let last_session = self.zone_session()?;
        Some(last_session.stats(|p| self.stats_filter(p)))
    }

    fn reset_zone_stats(&mut self) -> Option<()> {
        let last_session = self.zone_session_mut()?;
        self.zone_session = Some(Session::from(&last_session));
        info!("Reset: zone");
        Some(())
    }
}

impl OverallStats for Meter {
    fn overall_stats(&self) -> Option<PlayerStatisticsVec> {
        if let Some(zone) = self.zone_session() {
            return Some(PlayerStatisticsVec::merged(
                &self.zone_history,
                &zone.stats(|p| self.stats_filter(p)),
            ));
        }

        None
    }
}

impl GameStats for Meter {
    fn reset_stats(&mut self) -> Option<()> {
        self.zone_history = PlayerStatisticsVec::new();
        let last_session = self.zone_session_mut()?;
        self.zone_session = Some(Session::from(&last_session));
        self.last_fight_session = Session::from(&self.last_fight_session);
        info!("Reset: overall");
        Some(())
    }
}
