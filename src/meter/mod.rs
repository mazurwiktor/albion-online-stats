extern crate chrono;
extern crate timer;

use std::collections::HashMap;
use std::collections::VecDeque;

use log::*;

mod player;

use player::Player;

#[derive(Hash, Eq, PartialEq)]
struct PlayerName(String);

pub struct Session {
    players: HashMap<PlayerName, Player>,
}

#[derive(Debug)]
pub struct PlayerStatistics {
    player: String,
    damage: f32,
    time_in_combat: f32,
    dps: f32
}

impl Session {
    fn new() -> Self {
        Self {
            players: HashMap::new(),
        }
    }

    pub fn stats(&self) -> Vec<PlayerStatistics> {
        self.players
            .iter()
            .map(|(name, player)| PlayerStatistics {
                player: name.0.to_owned(),
                damage: player.get_damage_dealt(),
                time_in_combat: player.get_time_elapsed(),
                dps: player.get_damage_dealt() / player.get_time_elapsed() * 1000.0
            })
            .collect()
    }

    fn get_player_by_name(&self, player_name: &str) -> Option<&Player> {
        self.players.get(&PlayerName(player_name.to_owned()))
    }

    fn get_player_by_id(&mut self, player_id: usize) -> Option<&mut Player> {
        self.players.values_mut().find(|p| p.id == player_id)
    }

    fn add_player(&mut self, player_name: &str, player_id: usize) {
        self.players.insert(
            PlayerName(player_name.to_owned()),
            Player::new(player_id, player_name),
        );
    }
}

pub struct Meter {
    instance_sessions: VecDeque<Session>,
    main_player_id: Option<usize>,
}

impl Meter {
    pub fn new() -> Self {
        Self {
            instance_sessions: VecDeque::new(),
            main_player_id: None,
        }
    }

    pub fn register_main_player(&mut self, name: &str, id: usize) {
        error!("Main player {} registerd with id {}", name, id);
        self.main_player_id = Some(id);
        match self.instance_sessions.back_mut() {
            Some(session) => session.add_player(name, id),
            None => {
                let mut session = Session::new();
                session.add_player(name, id);
                self.instance_sessions.push_back(session);
            }
        }
    }

    pub fn register_leave(&mut self, id: usize) {
        if let Some(main_player_id) = self.main_player_id {
            if id == main_player_id {
                error!("New session, main player left the instance");
                self.instance_sessions.push_back(Session::new());
            }
        }
    }

    pub fn register_player(&mut self, name: &str, id: usize) {
        if self.instance_sessions.is_empty() {
            error!("New session");
            self.instance_sessions.push_back(Session::new());
        }

        let session = self.instance_sessions.back_mut().unwrap();
        session.add_player(name, id);
    }

    pub fn register_damage_dealt(&mut self, player_id: usize, damage: f32) {
        if let Some(session) = self.instance_sessions.back_mut() {
            if let Some(player) = session.get_player_by_id(player_id) {
                if damage < 0.0 {
                    player.register_damage_dealt(f32::abs(damage));
                }
            }
        }
    }

    pub fn register_combat_enter(&mut self, player_id: usize) {
        if let Some(session) = self.instance_sessions.back_mut() {
            if let Some(player) = session.get_player_by_id(player_id) {
                player.enter_combat();
            }
        }
    }

    pub fn register_combat_leave(&mut self, player_id: usize) {
        if let Some(session) = self.instance_sessions.back_mut() {
            if let Some(player) = session.get_player_by_id(player_id) {
                player.leave_combat();
            }
        }
    }

    pub fn get_instance_session(&self) -> Option<Vec<PlayerStatistics>> {
        let last_session = self.instance_sessions.back()?;
        Some(last_session.stats())
    }
}
