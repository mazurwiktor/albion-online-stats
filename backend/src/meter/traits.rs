#[cfg(test)]
use fake_clock::FakeClock as Instant;
#[cfg(not(test))]
use std::time::Instant;

use super::types::PlayerStatisticsVec;
use super::game_messages::Items;

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

pub trait PlayerEvents {
    fn get_damage_dealers_in_zone(&mut self, player_id: usize) -> Option<Vec<&mut dyn DamageDealer>>;

    fn get_fame_gatherers_in_zone(&mut self, player_id: usize) -> Option<Vec<&mut dyn FameGatherer>>;

    fn get_item_carriers_in_zone(&mut self, player_id: usize) -> Option<Vec<&mut dyn ItemCarrier>>;

    fn register_main_player(&mut self, name: &str, id: usize);

    fn register_leave(&mut self, id: usize) -> Option<()>;

    fn register_player(&mut self, name: &str, id: usize);

    fn register_item_update(&mut self, player_id: usize, items: &Items) -> Option<()>;

    fn register_fame_gain(&mut self, player_id: usize, fame: f32) -> Option<()> {
        for player in self.get_fame_gatherers_in_zone(player_id)? {
            player.register_fame_gain(fame);
        }

        Some(())
    }

    fn register_damage_dealt(&mut self, player_id: usize, damage: f32) -> Option<()> {
        for player in self.get_damage_dealers_in_zone(player_id)? {
            if damage < 0.0 {
                player.register_damage_dealt(f32::abs(damage));
            }
        }

        Some(())
    }

    fn register_combat_enter(&mut self, player_id: usize) -> Option<()> {
        for player in self.get_damage_dealers_in_zone(player_id)? {
            player.enter_combat();
        }

        Some(())
    }

    fn register_combat_leave(&mut self, player_id: usize) -> Option<()> {
        for player in self.get_damage_dealers_in_zone(player_id)? {
            player.leave_combat();
        }

        Some(())
    }
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
