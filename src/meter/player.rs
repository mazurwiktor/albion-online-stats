use std::fmt;
use std::sync::{Arc, Mutex};

#[cfg(test)]
use fake_clock::FakeClock as Instant;
#[cfg(not(test))]
use std::time::Instant;

use timer;

use super::traits::CombatState;
use super::traits::DamageDealer;
use super::traits::DamageStats;
use super::traits::FameGatherer;
use super::traits::FameStats;

struct Time {
    _guard: timer::Guard,
    _timer: timer::Timer,
}

impl Time {
    fn with(time_elapsed: Arc<Mutex<f32>>, combat_state: Arc<Mutex<CombatState>>) -> Time {
        let _timer = timer::Timer::new();

        let _guard = {
            _timer.schedule_repeating(chrono::Duration::milliseconds(10), move || {
                if *combat_state.lock().unwrap() == CombatState::InCombat {
                    *time_elapsed.lock().unwrap() += 10.0;
                }
            })
        };

        Self { _timer, _guard }
    }
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
    _time: Option<Time>,
    time_started: Instant,
    fame: f32,
}

impl Player {
    pub fn new(id: usize) -> Self {
        let time_elapsed = Arc::new(Mutex::new(0.0));
        let combat_state = Arc::new(Mutex::new(CombatState::OutOfCombat));

        Self {
            id,
            damage_dealt: 0.0,
            time_elapsed,
            combat_state,
            _time: None,
            time_started: Instant::now(),
            fame: 0.0,
        }
    }
}

impl DamageDealer for Player {
    fn register_damage_dealt(&mut self, damage_dealt: f32) {
        if self._time.is_none() {
            let elapsed = self.time_elapsed.clone();
            let state = self.combat_state.clone();
            self._time = Some(Time::with(elapsed, state));
        }
        if *self.combat_state.lock().unwrap() == CombatState::OutOfCombat {
            return;
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

impl FameGatherer for Player {
    fn register_fame_gain(&mut self, fame: f32) {
        self.fame += fame;
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
        let mut player = Player::new(1);

        sleep(1000 * 60);
        player.register_fame_gain(100.0);
        assert_eq!(player.fame_per_minute(), 100);
        assert_eq!(player.fame_per_hour(), 5999);

        sleep(1000 * 60 * 60 - 1000 * 60);

        assert_eq!(player.fame_per_hour(), 100);
    }
}
