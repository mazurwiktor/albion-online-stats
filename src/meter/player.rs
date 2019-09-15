use std::fmt;
use std::sync::{Arc, Mutex};

use timer;

use super::traits::CombatState;
use super::traits::DamageDealer;
use super::traits::DamageStats;

struct Time {
    _guard: timer::Guard,
    _timer: timer::Timer,
}

impl fmt::Debug for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

#[derive(Debug)]
pub struct Player {
    pub id: usize,
    damage_dealt: f32,
    time_elapsed: Arc<Mutex<f32>>,
    combat_state: Arc<Mutex<CombatState>>,
    _time: Time,
}

impl Player {
    pub fn new(id: usize) -> Self {
        let _timer = timer::Timer::new();
        let time_elapsed = Arc::new(Mutex::new(0.0));
        let combat_state = Arc::new(Mutex::new(CombatState::OutOfCombat));
        let _guard = {
            let time_elapsed = time_elapsed.clone();
            let combat_state = combat_state.clone();
            _timer.schedule_repeating(chrono::Duration::milliseconds(10), move || {
                if *combat_state.lock().unwrap() == CombatState::InCombat {
                    *time_elapsed.lock().unwrap() += 10.0;
                }
            })
        };

        Self {
            id,
            damage_dealt: 0.0,
            time_elapsed,
            combat_state,
            _time: Time { _timer, _guard },
        }
    }
}

impl DamageDealer for Player {
    fn register_damage_dealt(&mut self, damage_dealt: f32) {
        if *self.combat_state.lock().unwrap() == CombatState::OutOfCombat {
            *self.time_elapsed.lock().unwrap() += 1000.0;
        }

        self.damage_dealt += damage_dealt
    }

    fn enter_combat(&mut self) {
        *self.combat_state.lock().unwrap() = CombatState::InCombat;
    }

    fn leave_combat(&mut self) {
        *self.combat_state.lock().unwrap() = CombatState::OutOfCombat;
    }

    fn combat_state(&self) -> CombatState {
        match *self.combat_state.lock().unwrap() {
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
        *self.time_elapsed.lock().unwrap()
    }
}
