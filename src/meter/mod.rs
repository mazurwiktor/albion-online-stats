extern crate chrono;
extern crate timer;

use std::collections::HashMap;

use log::*;

mod player;
mod traits;
mod types;

use player::Player;

pub use traits::*;
pub use types::*;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct PlayerName(String);

#[derive(Debug)]
pub struct Session {
    players: HashMap<PlayerName, Player>,
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
            players.insert(player_name.clone(), Player::new(player.id));
        }

        Self { players }
    }

    pub fn stats(&self) -> PlayerStatisticsVec {
        self.players
            .iter()
            .map(|(name, player)| PlayerStatistics {
                player: name.0.to_owned(),
                damage: player.damage(),
                time_in_combat: player.time_in_combat(),
                dps: player.dps(),
            })
            .collect()
    }

    pub fn cleanup_players(&mut self) {
        let without_dmg = self
            .players
            .iter()
            .filter(|(_, player)| player.damage() == 0.0)
            .map(|(name, _)| name.clone())
            .collect::<Vec<PlayerName>>();
        for w in without_dmg {
            self.players.remove_entry(&w);
        }
    }

    pub fn players(&self) -> Vec<&Player> {
        self.players.iter().map(|(_, p)| p).collect()
    }

    fn get_player_by_id(&mut self, player_id: usize) -> Option<&mut Player> {
        self.players.values_mut().find(|p| p.id == player_id)
    }

    fn add_player(&mut self, player_name: &str, player_id: usize) {
        self.players
            .insert(PlayerName(player_name.to_owned()), Player::new(player_id));
    }
}

fn merge_stats(a : &PlayerStatisticsVec, b : &PlayerStatisticsVec) -> PlayerStatisticsVec {
    let merged = [&a[..], &b[..]].concat().iter().fold(
        HashMap::<String, PlayerStatistics>::new(),
        |mut acc, stat| {
            acc.entry(stat.player.clone())
                .and_modify(|s| {
                    s.damage += stat.damage;
                    s.time_in_combat += stat.time_in_combat;
                    s.dps = s.dps();
                })
                .or_insert(stat.clone());
            acc
        },
    );


    return merged.iter().map(|(_, v)| v.clone()).collect()
}

pub struct Meter {
    zone_history: PlayerStatisticsVec,
    zone_session: Option<Session>,
    last_fight_session: Session,
    main_player_id: Option<usize>,
}

impl Meter {
    pub fn new() -> Self {
        Self {
            zone_history: Vec::new(),
            zone_session: None,
            last_fight_session: Session::new(),
            main_player_id: None,
        }
    }

    fn zone_session_mut(&mut self) -> Option<&mut Session> {
        match &mut self.zone_session {
            Some(s) => Some(s),
            None => None
        }
    }

    fn zone_session(&self) -> Option<&Session> {
        match &self.zone_session {
            Some(s) => Some(s),
            None => None
        }
    }

    fn sessions_mut(&mut self) -> Option<(&mut Session, &mut Session)> {
        match &mut self.zone_session {
            Some(s) => Some((s, &mut self.last_fight_session)),
            None => None
        }
    }

    fn add_player(&mut self, name: &str, id: usize) -> Option<()> {
        let session = self.zone_session_mut()?;
        session.add_player(name, id);
        self.last_fight_session.add_player(name, id);
        Some(())
    }

    fn new_session(&mut self) {
        if let Some(zone) = self.zone_session() {
            self.zone_history = merge_stats(&self.zone_history, &zone.stats());
        }
        
        self.zone_session = Some(Session::new());
        self.last_fight_session = Session::new();
    }

    #[allow(unused)]
    fn combat_state(&self) -> CombatState {
        if self.last_fight_session
            .players()
            .iter()
            .any(|p| p.combat_state() == CombatState::InCombat) {
                return CombatState::InCombat;
            }

            CombatState::OutOfCombat
    }
}

impl PlayerEvents for Meter {
    fn get_damage_dealers_in_zone(&mut self, player_id: usize) -> Option<Vec<&mut DamageDealer>> {
        let (zone, last_fight) = self.sessions_mut()?;
        let las_fight_session_player = last_fight.get_player_by_id(player_id)?;
        let zone_player = zone.get_player_by_id(player_id)?;
        Some(vec![
            zone_player,
            las_fight_session_player,
        ])
    }

    fn register_main_player(&mut self, name: &str, id: usize) {
        debug!("Main player {} registerd with id {}", name, id);
        self.main_player_id = Some(id);

        if self.zone_session.is_none() {
            self.new_session();
        }
        self.add_player(name, id);
    }

    fn register_leave(&mut self, id: usize) -> Option<()> {
        let main_player_id = self.main_player_id?;
        if id == main_player_id {
            debug!("New session, main player left the zone");
            self.zone_session_mut()?.cleanup_players();
            self.new_session();
        }

        Some(())
    }

    fn register_player(&mut self, name: &str, id: usize) {
        if self.zone_session.is_none() {
            debug!("New session");
            self.new_session();
        }

        self.add_player(name, id);
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

impl ZoneStats for Meter {
    fn get_zone_session(&self) -> Option<Vec<PlayerStatistics>> {
        let last_session = self.zone_session()?;
        Some(last_session.stats())
    }

    fn new_zone_session(&mut self) -> Option<()> {
        let last_session = self.zone_session_mut()?;
        self.zone_session = Some(Session::from(&last_session));

        Some(())
    }

    fn get_overall_session(&self) -> Option<PlayerStatisticsVec> {
        if let Some(zone) = self.zone_session() {
            return Some(merge_stats(&self.zone_history, &zone.stats()));
        }

        None
    }

    fn reset(&mut self) {
        self.zone_history = vec![];
        self.zone_session = None;
        self.last_fight_session = Session::new();
        self.main_player_id = None;
    }

    fn get_last_fight_session(&self) -> Option<PlayerStatisticsVec> {
        let last_session = &self.last_fight_session;
        Some(last_session.stats())
    }

    fn new_last_fight_session(&mut self) -> Option<()> {
        self.last_fight_session = Session::from(&self.last_fight_session);
        Some(())
    }
}

#[test]
fn test_meter() {
    let mut meter = Meter::new();

    assert_eq!(meter.get_zone_session(), None);

    meter.register_main_player("name", 0);
    assert!(meter.get_zone_session().is_some());
    assert_eq!(
        meter.get_zone_session().unwrap()[0].player,
        "name".to_owned()
    );
}
