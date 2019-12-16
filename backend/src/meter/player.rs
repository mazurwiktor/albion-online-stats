#[cfg(test)]
use fake_clock::FakeClock as Instant;
#[cfg(not(test))]
use std::time::Instant;

use super::game_protocol::Items;

use super::traits::CombatState;
use super::traits::DamageDealer;
use super::traits::DamageStats;
use super::traits::FameStats;
use super::traits::FameGatherer;
use super::traits::ItemCarrier;

#[derive(Debug)]
struct CombatTime
{
    entered_combat: Option<Instant>,
    time_in_combat: std::time::Duration
}

impl CombatTime {
    fn new() -> Self {
        Self {
            entered_combat: None,
            time_in_combat: std::time::Duration::from_millis(0)
        }
    }
}

#[derive(Debug)]
pub struct Player {
    pub id: usize,
    damage_dealt: f32,
    combat_time: CombatTime,
    combat_state: CombatState,
    time_started: Instant,
    fame: f32,
    items: Items,
    idle: bool,
    pub main: bool
}


impl Player {
    pub fn new(id: usize, main: bool) -> Self {
        Self {
            id,
            damage_dealt: 0.0,
            combat_time: CombatTime::new(),
            combat_state: CombatState::OutOfCombat,
            time_started: Instant::now(),
            fame: 0.0,
            items: Default::default(),
            idle: true,
            main
        }
    }

    pub fn idle(&self) -> bool {
        self.idle
    }
}

impl DamageDealer for Player {
    fn register_damage_dealt(&mut self, damage_dealt: f32) {
        if self.combat_state == CombatState::OutOfCombat {
            return
        }
        self.idle = false;
        self.damage_dealt += damage_dealt
    }

    fn enter_combat(&mut self) {
        self.idle = false;
        self.combat_time.entered_combat = Some(Instant::now());
        self.combat_state = CombatState::InCombat;
    }

    fn leave_combat(&mut self) {
        if let Some(entered_combat) = self.combat_time.entered_combat {
            self.combat_time.time_in_combat += Instant::now() - entered_combat;
        }
        self.combat_state = CombatState::OutOfCombat;
    }

    fn combat_state(&self) -> CombatState {
        match self.combat_state {
            CombatState::InCombat => CombatState::InCombat,
            CombatState::OutOfCombat => CombatState::OutOfCombat,
        }
    }
}

impl DamageStats for Player {
    fn damage(&self) -> f32 {
        self.damage_dealt
    }
    fn time_in_combat(&self) -> f32 {
        if self.combat_state == CombatState::InCombat {
            if let Some(entered_combat) = self.combat_time.entered_combat {
                let time_in_combat = self.combat_time.time_in_combat + (Instant::now() - entered_combat);
                return time_in_combat.as_millis() as f32;
            }
        } else {
            return self.combat_time.time_in_combat.as_millis() as f32;
        }
        return 0.0;
    }
}

impl FameGatherer for Player {
    fn register_fame_gain(&mut self, fame: f32) {
        self.fame += fame;
        self.idle = false;
    }
}

impl ItemCarrier for Player {
    fn items_update(&mut self, items: &Items) {
        self.items = items.clone();
        self.idle = false;
    }

    fn items(&self) -> Items {
        self.items.clone()
    }
}


impl FameStats for Player {
    fn fame(&self) -> f32 {
        self.fame
    }

    fn time_started(&self) -> Instant {
        self.time_started
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sleep(time: u64) {
        use fake_clock::FakeClock;
        FakeClock::advance_time(time);
    }

    #[test]
    fn test_player_fame_stats() {
        let mut player = Player::new(1, true);

        sleep(1000 * 60);
        player.register_fame_gain(100.0);
        assert_eq!(player.fame_per_minute(), 100);
        assert_eq!(player.fame_per_hour(), 5999);

        sleep(1000 * 60 * 60 - 1000 * 60);

        assert_eq!(player.fame_per_hour(), 100);
    }
}

