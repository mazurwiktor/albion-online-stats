use std::collections::HashMap;

#[cfg(test)]
use fake_clock::FakeClock as Instant;
#[cfg(not(test))]
use std::time::Instant;

use super::game_protocol::Items;
use super::traits::DamageStats;
use super::traits::FameStats;

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerStatistics {
    pub player: String,
    pub damage: f32,
    pub time_in_combat: f32,
    pub dps: f32,
    pub seconds_in_game: f32,
    pub fame: f32,
    pub fame_per_minute: u32,
    pub fame_per_hour: u32,
    pub items: Items,
    pub idle: bool,
    pub main_player_stats: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerStatisticsVec {
    _vec: Vec<PlayerStatistics>
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum StatType {
    LastFight,
    Zone,
    Overall,
    Unknown
}

impl PlayerStatisticsVec {
    pub fn new() -> Self {
        Self {
            _vec: vec![]
        }
    }

    pub fn from(player_statistics_vec: Vec<PlayerStatistics>) -> Self {
        Self {
            _vec: player_statistics_vec
        }
    }

    pub fn value(&self) -> Vec<PlayerStatistics> {
        self._vec.clone()
    }

    pub fn merged(a: &Self, b: &Self) -> Self {
        let merged = [&a._vec[..], &b._vec[..]].concat().iter().fold(
            HashMap::<String, PlayerStatistics>::new(),
            |mut acc, stat| {
                acc.entry(stat.player.clone())
                    .and_modify(|s| {
                        s.damage += stat.damage;
                        s.time_in_combat += stat.time_in_combat;
                        s.dps = s.dps();
                        s.seconds_in_game += stat.seconds_in_game;
                        s.fame += stat.fame;
                        s.fame_per_minute = s.fame_per_minute();
                        s.fame_per_hour = s.fame_per_hour();
                        s.main_player_stats = stat.main_player_stats;
                    })
                    .or_insert(stat.clone());
                acc
            },
        );

        Self {
            _vec: merged.iter().map(|(_, v)| v.clone()).collect()
        }
    }
}

impl DamageStats for PlayerStatistics {
    fn damage(&self) -> f32 {
        self.damage
    }
    fn time_in_combat(&self) -> f32 {
        self.time_in_combat
    }
}

impl FameStats for PlayerStatistics {
    fn fame(&self) -> f32 {
        self.fame
    }

    fn time_started(&self) -> Instant {
        Instant::now()
    }

    fn time_in_game(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.seconds_in_game as u64)
    }
}

