extern crate timer;
extern crate chrono;

use std::collections::HashMap;
use std::collections::VecDeque;

use log::*;

mod player;

use player::Player;


#[derive(Hash, Eq, PartialEq)]
struct PlayerName(String);

pub struct Session {
    players: HashMap<PlayerName, Player>
}

impl Session {
    fn new() -> Self {
        Self {
            players: HashMap::new()
        }
    }

    pub fn get_dps(&self) -> String {
        let mut f = String::new();
        f.push_str("\n");
        for player in self.players.values() {
            f.push_str(&format!("Name {} \t\t  Damage {} \t\t DPS {}\n", player.get_name(), player.get_damage_dealt(), player.get_dps()))
        }
        f.push_str("\n");
        f
    }

    fn get_player_by_name(&self, player_name: &str) -> Option<&Player> {
        self.players.get(&PlayerName(player_name.to_owned()))
    }

    fn get_player_by_id(&mut self, player_id: usize) -> Option<&mut Player> {
        self.players.values_mut().find(|p| p.id == player_id)
    }

    fn add_player(&mut self, player_name: &str, player_id: usize) {
        self.players.insert(PlayerName(player_name.to_owned()), Player::new(player_id, player_name));
    }
}

pub struct Meter
{
    instance_sessions: VecDeque<Session>
}

impl Meter {
    pub fn new() -> Self {
        Self {
            instance_sessions: VecDeque::new()
        }
    }

    pub fn register_player(&mut self, name: &str, id: usize) {
        if self.instance_sessions.is_empty() {
            debug!("New session {} is_empty", name);
            self.instance_sessions.push_back(Session::new());
        } else if let Some(player) = self.instance_sessions.back().unwrap().get_player_by_name(name) {
            if player.id != id {
                debug!("New session {} already defined", name);
                self.instance_sessions.push_back(Session::new());
            }
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

    pub fn get_instance_session(&self) -> Option<&Session> {
        self.instance_sessions.back()
    }
}