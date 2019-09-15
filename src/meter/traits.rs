use super::types::PlayerStatistics;

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

pub trait DamageDealer {
    fn register_damage_dealt(&mut self, damage_dealt: f32);

    fn enter_combat(&mut self);

    fn leave_combat(&mut self);

    fn combat_state(&self) -> CombatState;
}

pub trait PlayerEvents {
    fn get_damage_dealers_in_zone(&mut self, player_id: usize) -> Option<Vec<&mut DamageDealer>>;

    fn register_main_player(&mut self, name: &str, id: usize);

    fn register_leave(&mut self, id: usize) -> Option<()>;

    fn register_player(&mut self, name: &str, id: usize);

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

pub trait ZoneStats {
    fn reset(&mut self);

    fn get_zone_session(&self) -> Option<Vec<PlayerStatistics>>;
    fn new_zone_session(&mut self) -> Option<()>;

    fn get_overall_session(&self) -> Option<Vec<PlayerStatistics>>;

    fn get_last_fight_session(&self) -> Option<Vec<PlayerStatistics>>;
    fn new_last_fight_session(&mut self) -> Option<()>;
}
