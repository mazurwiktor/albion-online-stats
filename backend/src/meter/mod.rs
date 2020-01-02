extern crate chrono;
extern crate timer;

use std::collections::HashMap;

use log::*;

mod player;
mod traits;
mod types;

use player::Player;

pub use super::game_protocol;
pub use traits::*;
pub use types::*;

#[derive(Debug)]
pub struct Session {
    players: HashMap<String, Player>,
}

impl Session {
    fn new() -> Self {
        Self {
            players: HashMap::new(),
        }
    }

    fn from(session: &Self) -> Self {
        let mut players = HashMap::new();
        for (player_name, player) in &session.players {
            let mut new_player = Player::from(player);
            if let CombatState::InCombat = player.combat_state() {
                new_player.enter_combat();
            }
            players.insert(player_name.clone(), new_player);
        }

        Self { players }
    }

    pub fn stats<F>(&self, filter: F) -> PlayerStatisticsVec
    where
        F: Fn(&(&String, &Player)) -> bool,
    {
        PlayerStatisticsVec::from(
            self.players
                .iter()
                .filter(filter)
                .map(|(name, player)| PlayerStatistics {
                    player: name.to_owned(),
                    damage: player.damage(),
                    time_in_combat: player.time_in_combat(),
                    dps: player.dps(),
                    seconds_in_game: player.time_in_game().as_secs() as f32,
                    fame: player.fame(),
                    fame_per_minute: player.fame_per_minute(),
                    fame_per_hour: player.fame_per_hour(),
                    items: player.items(),
                    idle: player.idle(),
                    main_player_stats: player.main(),
                })
                .collect(),
        )
    }

    pub fn cleanup_players(&mut self) {
        let without_dmg = self
            .players
            .iter()
            .filter(|(_, player)| !player.main() && player.damage() == 0.0 && player.fame() == 0.0)
            .map(|(name, _)| name.clone())
            .collect::<Vec<String>>();
        for w in without_dmg {
            self.players.remove_entry(&w);
        }
    }

    pub fn players(&self) -> Vec<&Player> {
        self.players.iter().map(|(_, p)| p).collect()
    }

    fn get_player_by_id(&mut self, player_id: usize) -> Option<&mut Player> {
        self.players.values_mut().find(|p| p.id() == player_id)
    }

    fn add_player(&mut self, player_name: &str, player_id: usize, main: bool) {
        self.players
            .insert(player_name.to_owned(), Player::new(player_id, main));
    }
}

#[derive(Default)]
pub struct MeterConfig {}

pub struct Meter {
    zone_history: PlayerStatisticsVec,
    zone_session: Option<Session>,
    last_fight_session: Session,
    main_player_id: Option<usize>,
    unconsumed_items: HashMap<usize, game_protocol::Items>,
    config: MeterConfig,
}

impl Meter {
    pub fn new() -> Self {
        Self {
            zone_history: PlayerStatisticsVec::new(),
            zone_session: None,
            last_fight_session: Session::new(),
            main_player_id: None,
            unconsumed_items: HashMap::new(),
            config: Default::default(),
        }
    }

    pub fn configure(&mut self, config: MeterConfig) {
        self.config = config;
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

    #[allow(unused)]
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

impl PlayerEvents for Meter {
    fn get_damage_dealers_in_zone(
        &mut self,
        player_id: usize,
    ) -> Option<Vec<&mut dyn DamageDealer>> {
        let (zone, last_fight) = self.sessions_mut()?;
        let las_fight_session_player = last_fight.get_player_by_id(player_id)?;
        let zone_player = zone.get_player_by_id(player_id)?;
        Some(vec![zone_player, las_fight_session_player])
    }

    fn get_item_carriers_in_zone(&mut self, player_id: usize) -> Option<Vec<&mut dyn ItemCarrier>> {
        let (zone, last_fight) = self.sessions_mut()?;
        let las_fight_session_player = last_fight.get_player_by_id(player_id)?;
        let zone_player = zone.get_player_by_id(player_id)?;
        Some(vec![zone_player, las_fight_session_player])
    }

    fn get_fame_gatherers_in_zone(
        &mut self,
        player_id: usize,
    ) -> Option<Vec<&mut dyn FameGatherer>> {
        let main_player_id = self.main_player_id?;
        let (zone, last_fight) = self.sessions_mut()?;
        if player_id != main_player_id {
            return None;
        }

        let las_fight_session_player = last_fight.get_player_by_id(player_id)?;
        let zone_player = zone.get_player_by_id(player_id)?;
        Some(vec![zone_player, las_fight_session_player])
    }

    fn register_item_update(
        &mut self,
        player_id: usize,
        items: &game_protocol::Items,
    ) -> Option<()> {
        let mut consumed = false;
        for player in self.get_item_carriers_in_zone(player_id).unwrap_or(vec![]) {
            player.items_update(items);
            consumed = true;
        }

        if !consumed {
            info!("Storing not consumed items for player id: {}", player_id);
            self.unconsumed_items.insert(player_id, items.clone());
        }

        Some(())
    }

    fn register_main_player(&mut self, name: &str, id: usize) {
        info!("Main player {} registered with id {}", name, id);
        self.main_player_id = Some(id);

        if self.zone_session.is_none() {
            self.new_session();
        }
        self.add_player(name, id, true);
        if let Some(items) = self.unconsumed_items.get(&id) {
            let i = items.clone();
            self.register_item_update(id, &i);
            self.unconsumed_items.remove(&id);
        }
    }

    fn register_leave(&mut self, id: usize) -> Option<()> {
        let main_player_id = self.main_player_id?;
        if id == main_player_id {
            info!(
                "Main player ({}) left the zone, Creating new zone session.",
                id
            );
            self.zone_session_mut()?.cleanup_players();
            self.new_session();
        }

        Some(())
    }

    fn register_player(&mut self, name: &str, id: usize) {
        if self.zone_session.is_none() {
            info!(
                "New player ({}) registered without session, creating new session",
                name
            );
            self.new_session();
        }

        if self
            .get_damage_dealers_in_zone(id)
            .unwrap_or(vec![])
            .is_empty()
        {
            self.add_player(name, id, false);
        }
    }

    fn register_combat_enter(&mut self, player_id: usize) -> Option<()> {
        if self.combat_state() == CombatState::OutOfCombat {
            self.last_fight_session = Session::from(&self.last_fight_session);
        }
        for player in self.get_damage_dealers_in_zone(player_id)? {
            player.enter_combat();
        }

        Some(())
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

#[test]
fn test_meter() {
    let mut meter = Meter::new();

    assert_eq!(meter.zone_stats(), None);

    meter.register_main_player("name", 0);
    assert!(meter.zone_stats().is_some());
}
