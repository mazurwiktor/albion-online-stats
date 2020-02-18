use std::collections::HashMap;


use crate::meter::traits::FameStats;
use crate::meter::traits::DamageStats;
use crate::meter::traits::ItemCarrier;
use crate::meter::traits::DamageDealer;
use crate::meter::types::PlayerStatistics;

use super::player::Player;

use super::traits::CombatState;
use super::types::PlayerStatisticsVec;

#[derive(Debug, Default)]
pub struct Session {
    players: HashMap<String, Player>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
        }
    }

    pub fn from(session: &Self) -> Self {
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

    pub fn players(&self) -> Vec<&Player> {
        self.players.iter().map(|(_, p)| p).collect()
    }

    pub fn get_player_by_id(&mut self, player_id: usize) -> Option<&mut Player> {
        self.players.values_mut().find(|p| p.id() == player_id)
    }

    pub fn add_player(&mut self, player_name: &str, player_id: usize, main: bool) {
        self.players
            .insert(player_name.to_owned(), Player::new(player_id, main));
    }
}