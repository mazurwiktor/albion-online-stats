use std::sync::{Arc, Mutex};

use timer;

#[derive(PartialEq)]
enum CombatState
{
    InCombat,
    OutOfCombat
}

struct Time
{
    _guard: timer::Guard,
    _timer: timer::Timer
}

pub struct Player
{
    pub id: usize,
    damage_dealt: f32,
    time_elapsed: Arc<Mutex<f32>>,
    combat_state: Arc<Mutex<CombatState>>,
    _time: Time
}

impl Player
{
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
            _time: Time{_timer, _guard}
        }
    }
    
    pub fn register_damage_dealt(&mut self, damage_dealt: f32) {
        if *self.combat_state.lock().unwrap() == CombatState::OutOfCombat {
            *self.time_elapsed.lock().unwrap() += 1000.0;
        }

        self.damage_dealt += damage_dealt
    }

    pub fn enter_combat(&mut self) { 
        *self.combat_state.lock().unwrap() = CombatState::InCombat;
    }

    pub fn leave_combat(&mut self) { 
        *self.combat_state.lock().unwrap() = CombatState::OutOfCombat;
    }

    pub fn get_damage_dealt(&self) -> f32 { self.damage_dealt }

    pub fn get_time_elapsed(&self) -> f32 {
        *self.time_elapsed.lock().unwrap()
    }
}


mod test
{
    use super::*;
    use std::thread;
    
    #[allow(unused)]
    fn get_test_player() -> Player {
        Player::new(1)
    }

    #[test]
    fn test_player_initial_stats() {
        let player = Player::new(1);

        assert_eq!(player.get_damage_dealt(), 0.0);
    }

    #[test]
    fn test_damage_when_in_combat() {
        let mut player = get_test_player();

        player.enter_combat();
        player.register_damage_dealt(1000.0);
        assert_eq!(player.get_damage_dealt(), 1000.0);
    }

    #[test]
    fn test_damage_after_tenth_second() {
        let mut player = get_test_player();

        player.register_damage_dealt(1000.0);
        player.enter_combat();

        thread::sleep(std::time::Duration::from_millis(100));
        assert_eq!(player.get_damage_dealt(), 1000.0);
    }
}