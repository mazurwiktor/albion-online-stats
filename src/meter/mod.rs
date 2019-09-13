extern crate chrono;
extern crate timer;

use std::collections::HashMap;
use std::collections::VecDeque;

use log::*;

mod player;
mod traits;
mod types;

use player::Player;

pub use traits::*;
pub use types::*;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct PlayerName(String);

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

        Self {
            players
        }
    }

    pub fn stats(&self) -> Vec<PlayerStatistics> {
        self.players
            .iter()
            .map(|(name, player)| PlayerStatistics {
                player: name.0.to_owned(),
                damage: player.damage(),
                time_in_combat: player.time_in_combat(),
                dps: player.dps()
            })
            .collect()
    }

    fn get_player_by_id(&mut self, player_id: usize) -> Option<&mut Player> {
        self.players.values_mut().find(|p| p.id == player_id)
    }

    fn add_player(&mut self, player_name: &str, player_id: usize) {
        self.players.insert(
            PlayerName(player_name.to_owned()),
            Player::new(player_id),
        );
    }
}

pub struct Meter {
    zone_sessions: VecDeque<Session>,
    main_player_id: Option<usize>,
}

impl Meter {
    pub fn new() -> Self {
        Self {
            zone_sessions: VecDeque::new(),
            main_player_id: None,
        }
    }
}


impl PlayerEvents for Meter {
    fn get_player_in_zone(&mut self, player_id: usize) -> Option<&mut Player> {
        let session = self.zone_sessions.back_mut()?;
        session.get_player_by_id(player_id)
    }

    fn register_main_player(&mut self, name: &str, id: usize) {
        debug!("Main player {} registerd with id {}", name, id);
        self.main_player_id = Some(id);
        match self.zone_sessions.back_mut() {
            Some(session) => session.add_player(name, id),
            None => {
                let mut session = Session::new();
                session.add_player(name, id);
                self.zone_sessions.push_back(session);
            }
        }
    }

    fn register_leave(&mut self, id: usize) -> Option<()> {
        let main_player_id = self.main_player_id?;
        if id == main_player_id {
            debug!("New session, main player left the zone");
            self.zone_sessions.push_back(Session::new());
        }

        Some(())
    }

    fn register_player(&mut self, name: &str, id: usize) {
        if self.zone_sessions.is_empty() {
            debug!("New session");
            self.zone_sessions.push_back(Session::new());
        }

        let session = self.zone_sessions.back_mut().unwrap();
        session.add_player(name, id);
    }
}

impl ZoneStats for Meter {
    fn get_zone_session(&self) -> Option<Vec<PlayerStatistics>> {
        let last_session = self.zone_sessions.back()?;
        Some(last_session.stats())
    }

    fn new_zone_session(&mut self) -> Option<()> {
        let last_session = self.zone_sessions.back()?;
        let new_session = Session::from(&last_session);
        self.zone_sessions.push_back(new_session);

        Some(())
    }
}

#[test]
fn test_meter() {
    let mut meter = Meter::new();

    assert_eq!(meter.get_zone_session(), None);

    meter.register_main_player("name", 0);
    
    assert!(meter.get_zone_session().is_some());
    assert_eq!(meter.get_zone_session().unwrap()[0].player, "name".to_owned());
}