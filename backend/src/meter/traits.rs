#[cfg(test)]
use fake_clock::FakeClock as Instant;
#[cfg(not(test))]
use std::time::Instant;

use super::types::PlayerStatisticsVec;
use super::photon_messages::Items;

#[derive(Debug, PartialEq)]
pub enum CombatState {
    InCombat,
    OutOfCombat,
}

pub trait DamageStats {
    fn damage(&self) -> f32;

    fn time_in_combat(&self) -> f32;

    fn dps(&self) -> f32 {
        if self.time_in_combat() == 0.0 {
            0.0
        } else {
            self.damage() / self.time_in_combat() * 1000.0
        }
    }
}

pub trait FameStats {
    fn fame(&self) -> f32;
    fn time_started(&self) -> Instant;
    fn time_in_game(&self) -> std::time::Duration {
        Instant::now() - self.time_started()
    }
    fn fame_per_minute(&self) -> u32 {
        let minutes_in_game = self.time_in_game().as_secs() as f32 / 60.0;
        (self.fame() / minutes_in_game) as u32
    }
    fn fame_per_hour(&self) -> u32 {
        let hours_in_game = self.time_in_game().as_secs() as f32 / 60.0 / 60.0;
        (self.fame() / hours_in_game) as u32
    }
}

pub trait FameGatherer {
    fn register_fame_gain(&mut self, fame: f32);
}

pub trait DamageDealer {
    fn register_damage_dealt(&mut self, damage_dealt: f32);

    fn enter_combat(&mut self);

    fn leave_combat(&mut self);

    fn combat_state(&self) -> CombatState;
}

pub trait ItemCarrier {
    fn items_update(&mut self, items: &Items);
    fn items(&self) -> Items;
}

pub trait LastFightStats {
    fn last_fight_stats(&self) -> Option<PlayerStatisticsVec>;
    fn reset_last_fight_stats(&mut self) -> Option<()>;
}

pub trait ZoneStats {
    fn zone_stats(&self) -> Option<PlayerStatisticsVec>;
    fn reset_zone_stats(&mut self) -> Option<()>;
}

pub trait OverallStats {
    fn overall_stats(&self) -> Option<PlayerStatisticsVec>;
}

pub trait GameStats {
    fn reset_stats(&mut self) -> Option<()>;
}
