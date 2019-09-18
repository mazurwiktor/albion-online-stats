#[macro_use]
extern crate cpython;
#[macro_use]
extern crate lazy_static;
extern crate bytes;
extern crate packet_sniffer;
extern crate protocol16;

use std::fs::File;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;
use std::thread;

use cpython::PyClone;
use cpython::{PyDict, PyList, PyObject, PyResult, Python, PythonObject, ToPyObject};
use log::*;
use simplelog::*;

use packet_sniffer::UdpPacket;

mod game_protocol;
mod meter;

use meter::PlayerEvents;
use meter::ZoneStats;

lazy_static! {
    static ref METER: Mutex<meter::Meter> = Mutex::new(meter::Meter::new());
}

impl ToPyObject for meter::PlayerStatistics {
    type ObjectType = PyObject;
    fn to_py_object(&self, py: Python) -> Self::ObjectType {
        let stats = PyDict::new(py);

        stats
            .set_item(py, "player", self.player.to_py_object(py))
            .unwrap();
        stats
            .set_item(py, "damage", self.damage.to_py_object(py))
            .unwrap();
        stats
            .set_item(py, "time_in_combat", self.time_in_combat.to_py_object(py))
            .unwrap();
        stats
            .set_item(py, "dps", self.dps.to_py_object(py))
            .unwrap();

        stats.into_object()
    }
}

fn get_zone_session(py: Python) -> PyResult<PyList> {
    let meter = &mut METER.lock().unwrap();
    meter.get_zone_session().map_or_else(
    // meter.get_last_fight_session().map_or_else(
        || Ok(PyList::new(py, Vec::<PyObject>::new().as_slice())),
        |v| { Ok(v.into_py_object(py)) }
    )
}

fn get_overall_session(py: Python) -> PyResult<PyList> {
    let meter = &mut METER.lock().unwrap();
    meter.get_overall_session().map_or_else(
        || Ok(PyList::new(py, Vec::<PyObject>::new().as_slice())),
        |v| { Ok(v.into_py_object(py)) }
    )
}

fn get_last_fight_session(py: Python) -> PyResult<PyList> {
    let meter = &mut METER.lock().unwrap();
    meter.get_last_fight_session().map_or_else(
        || Ok(PyList::new(py, Vec::<PyObject>::new().as_slice())),
        |v| { Ok(v.into_py_object(py)) }
    )
}

fn new_zone_session(_py: Python) -> PyResult<u32> {
    let meter = &mut METER.lock().unwrap();

    meter.new_zone_session();

    Ok(0)
}

fn reset_sessions(_py: Python) -> PyResult<u32> {
    let meter = &mut METER.lock().unwrap();

    meter.reset();

    Ok(0)
}

fn new_last_fight_session(_py: Python) -> PyResult<u32> {
    let meter = &mut METER.lock().unwrap();

    meter.new_last_fight_session();

    Ok(0)
}

fn initialize(_py: Python) -> PyResult<u32> {
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Trace,
        Config::default(),
        File::create("damage-meter.log").unwrap(),
    )])
    .unwrap();

    thread::spawn(move || {
        let (tx, rx): (Sender<UdpPacket>, Receiver<UdpPacket>) = channel();

        packet_sniffer::receive(tx);
        info!("Listening to network packets...");
        loop {
            if let Ok(packet) = rx.recv() {
                if packet.destination_port != 5056 && packet.source_port != 5056 {
                    continue;
                }
                let meter = &mut METER.lock().unwrap();
                register_messages(meter, &game_protocol::decode(&packet.payload));
            }
        }
    });

    Ok(0)
}

fn register_messages(meter: &mut meter::Meter, messages: &Vec<game_protocol::Message>) {
    messages
        .iter()
        .for_each(|message| register_message(meter, &message));
}

fn register_message(events: &mut PlayerEvents, message: &game_protocol::Message) {
    debug!("Found message {:?}", message);
    match message {
        game_protocol::Message::Leave(msg) => events.register_leave(msg.source).unwrap_or(()),
        game_protocol::Message::NewCharacter(msg) => {
            events.register_player(&msg.character_name, msg.source)
        }
        game_protocol::Message::CharacterStats(msg) => {
            events.register_main_player(&msg.character_name, msg.source)
        }
        game_protocol::Message::HealthUpdate(msg) => events
            .register_damage_dealt(msg.target, msg.value)
            .unwrap_or(()),
        game_protocol::Message::RegenerationHealthChanged(msg) => match msg.regeneration_rate {
            Some(_) => events.register_combat_leave(msg.source).unwrap_or(()),
            None => events.register_combat_enter(msg.source).unwrap_or(()),
        },
        game_protocol::Message::Died(msg) => events.register_combat_leave(msg.source).unwrap_or(()),
        _ => {}
    }
}

py_module_initializer!(libmeter, initliblibmeter, PyInit_libmeter, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust")?;
    m.add(py, "initialize", py_fn!(py, initialize()))?;
    m.add(py, "new_zone_session", py_fn!(py, new_zone_session()))?;
    m.add(py, "get_overall_session", py_fn!(py, get_overall_session()))?;
    m.add(
        py,
        "get_last_fight_session",
        py_fn!(py, get_last_fight_session()),
    )?;
    m.add(py, "reset_sessions", py_fn!(py, reset_sessions()))?;
    m.add(
        py,
        "new_last_fight_session",
        py_fn!(py, new_last_fight_session()),
    )?;
    m.add(py, "get_zone_session", py_fn!(py, get_zone_session()))?;
    Ok(())
});

#[cfg(test)]
mod tests {
    use super::*;
    use cpython::{PyFloat, PyUnicode};

    use game_protocol::message;
    use game_protocol::Message;

    mod helpers {
        use super::*;

        pub fn init() -> cpython::GILGuard {
            let meter = &mut METER.lock().unwrap();
            meter.reset();

            Python::acquire_gil()
        }

        pub fn register(message: Message) {
            let meter = &mut METER.lock().unwrap();
            r(meter, &message);
        }

        fn r(meter: &mut meter::Meter, message: &game_protocol::Message) {
            register_message(meter, &message);
        }

        pub fn get_damage_dealer_in_zone_by_index(py: Python, index: usize) -> PyDict {
            let zone_session = get_zone_session(py).unwrap();
            let stat = zone_session.get_item(py, index);
            let player = stat.cast_as::<PyDict>(py).unwrap().clone_ref(py);
            player
        }

        pub fn get_damage_dealer_in_zone_by_name(py: Python, name: &str) -> Option<PyDict> {
            let zone_session_len = get_zone_session(py).unwrap().len(py);

            for idx in 0..zone_session_len {
                let player = get_damage_dealer_in_zone_by_index(py, idx);
                if get_string(py, &player, "player") == name {
                    return Some(player);
                }
            }
            None
        }

        pub fn get_player_overall_index(py: Python, index: usize) -> PyDict {
            let zone_session = get_overall_session(py).unwrap();
            let stat = zone_session.get_item(py, index);
            let player = stat.cast_as::<PyDict>(py).unwrap().clone_ref(py);
            player
        }

        pub fn get_player_overall_by_name(py: Python, name: &str) -> Option<PyDict> {
            let zone_session_len = get_overall_session(py).unwrap().len(py);

            for idx in 0..zone_session_len {
                let player = get_player_overall_index(py, idx);
                if get_string(py, &player, "player") == name {
                    return Some(player);
                }
            }
            None
        }

        pub fn get_player_last_fight_index(py: Python, index: usize) -> PyDict {
            let zone_session = get_last_fight_session(py).unwrap();
            let stat = zone_session.get_item(py, index);
            let player = stat.cast_as::<PyDict>(py).unwrap().clone_ref(py);
            player
        }

        pub fn get_player_last_fight_by_name(py: Python, name: &str) -> Option<PyDict> {
            let zone_session_len = get_last_fight_session(py).unwrap().len(py);

            for idx in 0..zone_session_len {
                let player = get_player_last_fight_index(py, idx);
                if get_string(py, &player, "player") == name {
                    return Some(player);
                }
            }
            None
        }

        pub fn get_string(py: Python, stats: &PyDict, key: &str) -> String {
            stats
                .get_item(py, key)
                .unwrap()
                .cast_as::<PyUnicode>(py)
                .unwrap()
                .to_string_lossy(py)
                .to_string()
        }

        pub fn get_float(py: Python, stats: &PyDict, key: &str) -> f64 {
            stats
                .get_item(py, key)
                .unwrap()
                .cast_as::<PyFloat>(py)
                .unwrap()
                .value(py)
        }
    }

    trait Testing {
        fn new(source: usize) -> Self;
    }

    trait NamedTesting {
        fn new_named(name: &str, source: usize) -> Self;
    }

    trait SwitchableTesting {
        fn enabled(source: usize) -> Self;
        fn disabled(source: usize) -> Self;
    }

    impl NamedTesting for message::NewCharacter {
        fn new_named(name: &str, source: usize) -> Self {
            Self {
                source: source,
                character_name: name.to_owned(),
                health: 10.0,
                max_health: 10.0,
                energy: 1.0,
                max_energy: 1.0,
            }
        }
    }

    impl NamedTesting for message::CharacterStats {
        fn new_named(name: &str, source: usize) -> Self {
            Self {
                source: source,
                character_name: name.to_owned(),
                health: 10.0,
                max_health: 10.0,
                energy: 1.0,
                max_energy: 1.0,
            }
        }
    }

    impl Testing for message::NewCharacter {
        fn new(source: usize) -> Self {
            message::NewCharacter::new_named("CH1", source)
        }
    }

    impl Testing for message::CharacterStats {
        fn new(source: usize) -> Self {
            message::CharacterStats::new_named("MAIN_CH1", source)
        }
    }

    impl Testing for message::Leave {
        fn new(source: usize) -> Self {
            Self { source: source }
        }
    }

    impl Testing for message::HealthUpdate {
        fn new(source: usize) -> Self {
            Self {
                source: 200,
                target: source,
                value: -10.0,
            }
        }
    }

    impl SwitchableTesting for message::RegenerationHealthChanged {
        fn enabled(source: usize) -> Self {
            Self {
                source: source,
                health: 10.0,
                max_health: 10.0,
                regeneration_rate: Some(1.0),
            }
        }

        fn disabled(source: usize) -> Self {
            Self {
                source: source,
                health: 10.0,
                max_health: 10.0,
                regeneration_rate: None,
            }
        }
    }

    #[test]
    fn test_empty_session() {
        let guard = helpers::init();
        let py = guard.python();

        assert_eq!(get_zone_session(py).unwrap().len(py), 0);
    }

    #[test]
    fn test_new_player_appears() {
        let guard = helpers::init();
        let py = guard.python();

        helpers::register(Message::NewCharacter(message::NewCharacter::new(1)));

        let zone_session = get_zone_session(py).unwrap();
        assert_eq!(zone_session.len(py), 1);
    }

    #[test]
    fn test_new_player_stats() {
        let guard = helpers::init();
        let py = guard.python();

        helpers::register(Message::NewCharacter(message::NewCharacter::new(1)));

        let stats = helpers::get_damage_dealer_in_zone_by_index(py, 0);
        assert_eq!(stats.len(py), 4);
        assert_eq!(helpers::get_string(py, &stats, "player"), "CH1");
        assert_eq!(helpers::get_float(py, &stats, "damage"), 0.0);
        assert_eq!(helpers::get_float(py, &stats, "time_in_combat"), 0.0);
        assert_eq!(helpers::get_float(py, &stats, "dps"), 0.0);
    }

    #[test]
    fn test_damage_aggregation() {
        let guard = helpers::init();
        let py = guard.python();

        helpers::register(Message::NewCharacter(message::NewCharacter::new(1)));
        let stats = helpers::get_damage_dealer_in_zone_by_index(py, 0);
        assert_eq!(helpers::get_float(py, &stats, "damage"), 0.0);

        helpers::register(Message::RegenerationHealthChanged(
            message::RegenerationHealthChanged::disabled(1),
        ));

        helpers::register(Message::HealthUpdate(message::HealthUpdate::new(1)));
        let stats = helpers::get_damage_dealer_in_zone_by_index(py, 0);
        assert_eq!(helpers::get_float(py, &stats, "damage"), 10.0);

        helpers::register(Message::HealthUpdate(message::HealthUpdate::new(1)));
        let stats = helpers::get_damage_dealer_in_zone_by_index(py, 0);
        assert_eq!(helpers::get_float(py, &stats, "damage"), 20.0);
    }

    #[test]
    fn test_new_player_damage() {
        let guard = helpers::init();
        let py = guard.python();

        helpers::register(Message::NewCharacter(message::NewCharacter::new(1)));

        let stats = helpers::get_damage_dealer_in_zone_by_index(py, 0);
        assert_eq!(helpers::get_string(py, &stats, "player"), "CH1");
        assert_eq!(helpers::get_float(py, &stats, "damage"), 0.0);

        helpers::register(Message::RegenerationHealthChanged(
            message::RegenerationHealthChanged::disabled(1),
        ));
        helpers::register(Message::HealthUpdate(message::HealthUpdate::new(1)));

        let stats = helpers::get_damage_dealer_in_zone_by_index(py, 0);
        assert_eq!(helpers::get_float(py, &stats, "damage"), 10.0);
    }

    #[test]
    fn test_new_player_damage_reset() {
        let guard = helpers::init();
        let py = guard.python();

        helpers::register(Message::NewCharacter(message::NewCharacter::new(1)));

        let stats = helpers::get_damage_dealer_in_zone_by_index(py, 0);
        assert_eq!(helpers::get_string(py, &stats, "player"), "CH1");
        assert_eq!(helpers::get_float(py, &stats, "damage"), 0.0);

        helpers::register(Message::RegenerationHealthChanged(
            message::RegenerationHealthChanged::disabled(1),
        ));
        helpers::register(Message::HealthUpdate(message::HealthUpdate::new(1)));

        let stats = helpers::get_damage_dealer_in_zone_by_index(py, 0);
        assert_eq!(helpers::get_float(py, &stats, "damage"), 10.0);

        new_zone_session(py).unwrap();
        let stats = helpers::get_damage_dealer_in_zone_by_index(py, 0);
        assert_eq!(helpers::get_float(py, &stats, "damage"), 0.0);
    }

    #[test]
    fn test_zone_detection() {
        let guard = helpers::init();
        let py = guard.python();

        helpers::register(Message::CharacterStats(message::CharacterStats::new(1)));

        let stats = helpers::get_damage_dealer_in_zone_by_index(py, 0);
        assert_eq!(helpers::get_string(py, &stats, "player"), "MAIN_CH1");
        assert_eq!(helpers::get_float(py, &stats, "damage"), 0.0);

        helpers::register(Message::RegenerationHealthChanged(
            message::RegenerationHealthChanged::disabled(1),
        ));
        helpers::register(Message::HealthUpdate(message::HealthUpdate::new(1)));
        let stats = helpers::get_damage_dealer_in_zone_by_index(py, 0);
        assert_eq!(helpers::get_float(py, &stats, "damage"), 10.0);

        helpers::register(Message::Leave(message::Leave::new(1)));
        helpers::register(Message::CharacterStats(message::CharacterStats::new(2)));
        let stats = helpers::get_damage_dealer_in_zone_by_index(py, 0);
        assert_eq!(helpers::get_float(py, &stats, "damage"), 0.0);

        helpers::register(Message::RegenerationHealthChanged(
            message::RegenerationHealthChanged::disabled(2),
        ));
        helpers::register(Message::HealthUpdate(message::HealthUpdate::new(2)));
        let stats = helpers::get_damage_dealer_in_zone_by_index(py, 0);
        assert_eq!(helpers::get_float(py, &stats, "damage"), 10.0);
    }

    #[test]
    fn test_two_players_in_the_zone() {
        let guard = helpers::init();
        let py = guard.python();

        helpers::register(Message::CharacterStats(message::CharacterStats::new_named(
            "MAIN_CH1", 1,
        )));
        let stats = helpers::get_damage_dealer_in_zone_by_name(py, "MAIN_CH1").unwrap();
        assert_eq!(helpers::get_string(py, &stats, "player"), "MAIN_CH1");
        assert_eq!(helpers::get_float(py, &stats, "damage"), 0.0);

        helpers::register(Message::NewCharacter(message::NewCharacter::new_named(
            "CH1", 2,
        )));
        let stats = helpers::get_damage_dealer_in_zone_by_name(py, "CH1").unwrap();
        assert_eq!(helpers::get_string(py, &stats, "player"), "CH1");
        assert_eq!(helpers::get_float(py, &stats, "damage"), 0.0);

        helpers::register(Message::Leave(message::Leave::new(1)));
        let stats = helpers::get_damage_dealer_in_zone_by_name(py, "CH1");
        assert!(stats.is_none());
    }

    #[test]
    fn test_overall_damage() {
        let guard = helpers::init();
        let py = guard.python();

        helpers::register(Message::CharacterStats(message::CharacterStats::new_named(
            "MAIN_CH1", 1,
        )));
        let stats = helpers::get_player_overall_by_name(py, "MAIN_CH1").unwrap();
        assert_eq!(helpers::get_float(py, &stats, "damage"), 0.0);

        helpers::register(Message::RegenerationHealthChanged(
            message::RegenerationHealthChanged::disabled(1),
        ));
        helpers::register(Message::HealthUpdate(message::HealthUpdate::new(1)));
        let stats = helpers::get_player_overall_by_name(py, "MAIN_CH1").unwrap();
        assert_eq!(helpers::get_float(py, &stats, "damage"), 10.0);

        helpers::register(Message::Leave(message::Leave::new(1)));
        helpers::register(Message::CharacterStats(message::CharacterStats::new_named(
            "MAIN_CH1", 1,
        )));
        let stats = helpers::get_player_overall_by_name(py, "MAIN_CH1").unwrap();
        assert_eq!(helpers::get_float(py, &stats, "damage"), 10.0);

        helpers::register(Message::RegenerationHealthChanged(
            message::RegenerationHealthChanged::disabled(1),
        ));
        helpers::register(Message::HealthUpdate(message::HealthUpdate::new(1)));
        let stats = helpers::get_player_overall_by_name(py, "MAIN_CH1").unwrap();
        assert_eq!(helpers::get_float(py, &stats, "damage"), 20.0);
    }

    #[test]
    fn test_last_fight_damage() {
        let guard = helpers::init();
        let py = guard.python();

        helpers::register(Message::CharacterStats(message::CharacterStats::new_named(
            "MAIN_CH1", 1,
        )));
        let stats = helpers::get_player_last_fight_by_name(py, "MAIN_CH1").unwrap();
        assert_eq!(helpers::get_string(py, &stats, "player"), "MAIN_CH1");
        assert_eq!(helpers::get_float(py, &stats, "damage"), 0.0);

        helpers::register(Message::RegenerationHealthChanged(
            message::RegenerationHealthChanged::disabled(1),
        ));
        helpers::register(Message::HealthUpdate(message::HealthUpdate::new(1)));
        let stats = helpers::get_player_last_fight_by_name(py, "MAIN_CH1").unwrap();
        assert_eq!(helpers::get_float(py, &stats, "damage"), 10.0);

        helpers::register(Message::RegenerationHealthChanged(
            message::RegenerationHealthChanged::enabled(1),
        ));
        helpers::register(Message::RegenerationHealthChanged(
            message::RegenerationHealthChanged::disabled(1),
        ));
    }

    #[test]
    fn test_last_fight_management() {
        // session should be started when first player attacks
        // damage should be 0 when all players were out of combat and some player attacks

        let guard = helpers::init();
        let py = guard.python();

        helpers::register(Message::CharacterStats(message::CharacterStats::new_named(
            "1", 1,
        )));
        helpers::register(Message::RegenerationHealthChanged(
            message::RegenerationHealthChanged::disabled(1),
        ));
        helpers::register(Message::HealthUpdate(message::HealthUpdate::new(1)));
        helpers::register(Message::RegenerationHealthChanged(
            message::RegenerationHealthChanged::disabled(1),
        ));

        assert!(helpers::get_player_last_fight_by_name(py, "1").is_some());

        helpers::register(Message::NewCharacter(message::NewCharacter::new_named(
            "2", 2,
        )));
        helpers::register(Message::RegenerationHealthChanged(
            message::RegenerationHealthChanged::disabled(2),
        ));
        helpers::register(Message::HealthUpdate(message::HealthUpdate::new(2)));
        helpers::register(Message::RegenerationHealthChanged(
            message::RegenerationHealthChanged::disabled(2),
        ));

        helpers::register(Message::NewCharacter(message::NewCharacter::new_named(
            "3", 3,
        )));
        helpers::register(Message::RegenerationHealthChanged(
            message::RegenerationHealthChanged::disabled(3),
        ));
        helpers::register(Message::HealthUpdate(message::HealthUpdate::new(3)));
        helpers::register(Message::RegenerationHealthChanged(
            message::RegenerationHealthChanged::disabled(3),
        ));

        helpers::register(Message::RegenerationHealthChanged(
            message::RegenerationHealthChanged::enabled(1),
        ));

        macro_rules! assert_named_player_dmg {
                ($id:expr, $dmg:expr) => {
                    assert_eq!(
                        helpers::get_float(
                            py,
                            &helpers::get_player_last_fight_by_name(py, $id).unwrap(),
                            "damage"
                        ),
                        $dmg
                    );
                }
        }

        assert_named_player_dmg!("1", 10.0);
        assert_named_player_dmg!("2", 10.0);
        assert_named_player_dmg!("3", 10.0);

        helpers::register(Message::RegenerationHealthChanged(
            message::RegenerationHealthChanged::enabled(2),
        ));

        assert_named_player_dmg!("1", 10.0);
        assert_named_player_dmg!("2", 10.0);
        assert_named_player_dmg!("3", 10.0);

        helpers::register(Message::RegenerationHealthChanged(
            message::RegenerationHealthChanged::enabled(3),
        ));
        assert_named_player_dmg!("1", 10.0);
        assert_named_player_dmg!("2", 10.0);
        assert_named_player_dmg!("3", 10.0);

        helpers::register(Message::RegenerationHealthChanged(
            message::RegenerationHealthChanged::disabled(1),
        ));
        helpers::register(Message::HealthUpdate(message::HealthUpdate::new(1)));
        assert_named_player_dmg!("1", 10.0);
        assert_named_player_dmg!("2", 0.0);
        assert_named_player_dmg!("3", 0.0);
    }
}
