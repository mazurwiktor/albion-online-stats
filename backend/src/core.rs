use std::fs::File;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use log::*;
use simplelog::*;

use packet_sniffer::UdpPacket;

use photon_decode;
use photon_decode::Photon;

use crate::game_protocol;
use crate::meter;
pub use meter::GameStats;
pub use meter::LastFightStats;
pub use meter::MeterConfig;
pub use meter::OverallStats;
pub use meter::PlayerEvents;
pub use meter::PlayerStatistics;
pub use meter::PlayerStatisticsVec;
pub use meter::ZoneStats;

pub use meter::StatType;
pub use crate::game_protocol::Items;

pub enum InitializationError {
    NetworkInterfaceListMissing,
}

lazy_static! {
    static ref METER: Mutex<meter::Meter> = Mutex::new(meter::Meter::new());
}

pub fn stats(meter: &meter::Meter, stat_type: StatType) -> Vec<meter::PlayerStatistics> {
    match stat_type {
        StatType::Zone => meter
            .zone_stats()
            .unwrap_or(meter::PlayerStatisticsVec::new())
            .value(),
        StatType::LastFight => meter
            .last_fight_stats()
            .unwrap_or(meter::PlayerStatisticsVec::new())
            .value(),
        StatType::Overall => meter
            .overall_stats()
            .unwrap_or(meter::PlayerStatisticsVec::new())
            .value(),
        _ => vec![],
    }
}

pub fn reset(meter: &mut meter::Meter, stat_type: StatType) {
    match stat_type {
        StatType::Zone => {
            meter.reset_zone_stats();
        }
        StatType::LastFight => {
            meter.reset_last_fight_stats();
        }
        StatType::Overall => {
            meter.reset_stats();
        }
        _ => error!("Unexpected stat to reset."),
    }
}

pub fn initialize() -> Result<Arc<Mutex<meter::Meter>>, InitializationError> {
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        File::create("damage-meter.log").unwrap(),
    )])
    .unwrap();

    let meter = meter::Meter::new();

    let meter = Arc::new(Mutex::new(meter));
    let cloned_meter = meter.clone();

    if let Ok(interfaces) = packet_sniffer::network_interfaces() {
        thread::spawn(move || {
            let (tx, rx): (Sender<UdpPacket>, Receiver<UdpPacket>) = channel();

            let mut photon = Photon::new();

            packet_sniffer::receive(interfaces, tx);
            info!("Listening to network packets...");
            loop {
                if let Ok(packet) = rx.recv() {
                    if packet.destination_port != 5056 && packet.source_port != 5056 {
                        continue;
                    }
                    if let Ok(ref mut meter) = cloned_meter.lock() {
                        let game_messages = photon
                            .decode(&packet.payload)
                            .into_iter()
                            .filter_map(game_protocol::into_game_message)
                            .collect();
                        register_messages(meter, &game_messages);
                    }
                }
            }
        });
    } else {
        return Err(InitializationError::NetworkInterfaceListMissing);
    }

    Ok(meter)
}

pub fn register_messages(meter: &mut meter::Meter, messages: &Vec<game_protocol::Message>) {
    messages
        .iter()
        .for_each(|message| register_message(meter, &message));
}

fn register_message<Events>(events: &mut Events, message: &game_protocol::Message)
where
    Events: PlayerEvents,
{
    info!("Found message {:?}", message);
    match message {
        game_protocol::Message::Leave(msg) => events.register_leave(msg.source).unwrap_or(()),
        game_protocol::Message::NewCharacter(msg) => {
            events.register_player(&msg.character_name, msg.source);
            events.register_item_update(msg.source, &msg.items);
        },
        game_protocol::Message::PlayerItems(msg) => {
            events.register_item_update(msg.source, &msg.items);
        },
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
        game_protocol::Message::FameUpdate(msg) => events
            .register_fame_gain(msg.source, msg.fame as f32 / 10000.0)
            .unwrap_or(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use game_protocol::message;
    use game_protocol::Items;
    use game_protocol::Message;

    mod helpers {
        use super::*;

        pub fn init_() -> meter::Meter {
            meter::Meter::new()
        }

        pub fn sleep(time: u64) {
            use fake_clock::FakeClock;
            FakeClock::advance_time(time);
        }
    }

    trait Testing {
        fn new(source: usize) -> Self;
    }

    trait NamedTesting {
        fn new_named(name: &str, source: usize) -> Self;
    }

    trait ListNamedTesting {
        fn new_list_of_named(names: &[&str], source: usize) -> Self;
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
                items: Items::from(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
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

    impl Testing for message::FameUpdate {
        fn new(source: usize) -> Self {
            Self {
                source: source,
                fame: 1000000,
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
        let meter = helpers::init_();
        assert_eq!(stats(&meter, StatType::Zone).len(), 0);
    }

    #[test]
    fn test_new_player_appears() {
        let mut meter = helpers::init_();
        register_message(
            &mut meter,
            &Message::NewCharacter(message::NewCharacter::new(1)),
        );
        assert_eq!(stats(&meter, StatType::Zone).len(), 1);
    }

    #[test]
    fn test_new_player_stats() {
        let mut meter = helpers::init_();
        register_message(
            &mut meter,
            &Message::NewCharacter(message::NewCharacter::new(1)),
        );
        assert_eq!(stats(&meter, StatType::Zone).len(), 1);

        let zone_stats = stats(&meter, StatType::Zone);
        assert_eq!(zone_stats[0].player, "CH1");
        assert_eq!(zone_stats[0].damage, 0.0);
        assert_eq!(zone_stats[0].time_in_combat, 0.0);
        assert_eq!(zone_stats[0].dps, 0.0);
        assert_eq!(zone_stats[0].seconds_in_game, 0.0);
        assert_eq!(zone_stats[0].fame, 0.0);
        assert_eq!(zone_stats[0].fame_per_minute, 0);
        assert_eq!(zone_stats[0].fame_per_hour, 0);
    }

    #[test]
    fn test_damage_aggregation() {
        let mut meter = helpers::init_();
        register_message(
            &mut meter,
            &Message::NewCharacter(message::NewCharacter::new(1)),
        );

        let zone_stats = stats(&meter, StatType::Zone);
        assert_eq!(zone_stats[0].damage, 0.0);

        register_message(
            &mut meter,
            &Message::RegenerationHealthChanged(message::RegenerationHealthChanged::disabled(1)),
        );
        let zone_stats = stats(&meter, StatType::Zone);
        assert_eq!(zone_stats[0].damage, 0.0);

        register_message(
            &mut meter,
            &Message::HealthUpdate(message::HealthUpdate::new(1)),
        );
        let zone_stats = stats(&meter, StatType::Zone);
        assert_eq!(zone_stats[0].damage, 10.0);

        register_message(
            &mut meter,
            &Message::HealthUpdate(message::HealthUpdate::new(1)),
        );
        let zone_stats = stats(&meter, StatType::Zone);
        assert_eq!(zone_stats[0].damage, 20.0);
    }

    #[test]
    fn test_new_player_damage() {
        let mut meter = helpers::init_();
        register_message(
            &mut meter,
            &Message::NewCharacter(message::NewCharacter::new(1)),
        );

        let zone_stats = stats(&meter, StatType::Zone);
        assert_eq!(zone_stats[0].damage, 0.0);
        assert_eq!(zone_stats[0].player, "CH1");

        register_message(
            &mut meter,
            &Message::RegenerationHealthChanged(message::RegenerationHealthChanged::disabled(1)),
        );
        register_message(
            &mut meter,
            &Message::HealthUpdate(message::HealthUpdate::new(1)),
        );
        let zone_stats = stats(&meter, StatType::Zone);
        assert_eq!(zone_stats[0].damage, 10.0);
    }

    #[test]
    fn test_new_player_damage_reset() {
        let mut meter = helpers::init_();
        register_message(
            &mut meter,
            &Message::NewCharacter(message::NewCharacter::new(1)),
        );

        let zone_stats = stats(&meter, StatType::Zone);
        assert_eq!(zone_stats[0].damage, 0.0);
        assert_eq!(zone_stats[0].player, "CH1");

        register_message(
            &mut meter,
            &Message::RegenerationHealthChanged(message::RegenerationHealthChanged::disabled(1)),
        );
        register_message(
            &mut meter,
            &Message::HealthUpdate(message::HealthUpdate::new(1)),
        );
        let zone_stats = stats(&meter, StatType::Zone);
        assert_eq!(zone_stats[0].damage, 10.0);

        reset(&mut meter, StatType::Zone);
        let zone_stats = stats(&meter, StatType::Zone);
        assert_eq!(zone_stats[0].damage, 0.0);
    }

    #[test]
    fn test_zone_detection() {
        let mut meter = helpers::init_();
        register_message(
            &mut meter,
            &Message::CharacterStats(message::CharacterStats::new(1)),
        );

        let zone_stats = stats(&meter, StatType::Zone);
        assert_eq!(zone_stats[0].damage, 0.0);
        assert_eq!(zone_stats[0].player, "MAIN_CH1");

        register_message(
            &mut meter,
            &Message::RegenerationHealthChanged(message::RegenerationHealthChanged::disabled(1)),
        );
        register_message(
            &mut meter,
            &Message::HealthUpdate(message::HealthUpdate::new(1)),
        );
        let zone_stats = stats(&meter, StatType::Zone);
        assert_eq!(zone_stats[0].damage, 10.0);

        register_message(&mut meter, &Message::Leave(message::Leave::new(1)));
        register_message(
            &mut meter,
            &Message::CharacterStats(message::CharacterStats::new(2)),
        );

        let zone_stats = stats(&meter, StatType::Zone);
        assert_eq!(zone_stats[0].damage, 0.0);

        register_message(
            &mut meter,
            &Message::RegenerationHealthChanged(message::RegenerationHealthChanged::disabled(2)),
        );
        register_message(
            &mut meter,
            &Message::HealthUpdate(message::HealthUpdate::new(2)),
        );
        let zone_stats = stats(&meter, StatType::Zone);
        assert_eq!(zone_stats[0].damage, 10.0);
    }

    macro_rules! main_character_enters {
        ($meter:expr, $name:expr, $id:expr) => {
            register_message(
                &mut $meter,
                &Message::CharacterStats(message::CharacterStats::new_named($name, $id)),
            );
        };
    }

    macro_rules! character_enters {
        ($meter:expr, $name:expr, $id:expr) => {
            register_message(
                &mut $meter,
                &Message::NewCharacter(message::NewCharacter::new_named($name, $id)),
            );
        };
    }

    macro_rules! attack {
        ($meter:expr, $id:expr) => {
            register_message(
                &mut $meter,
                &Message::RegenerationHealthChanged(message::RegenerationHealthChanged::disabled(
                    $id,
                )),
            );
            register_message(
                &mut $meter,
                &Message::HealthUpdate(message::HealthUpdate::new($id)),
            );
        };
    }

    macro_rules! combat_leave {
        ($meter:expr, $id:expr) => {
            register_message(
                &mut $meter,
                &Message::RegenerationHealthChanged(message::RegenerationHealthChanged::enabled(
                    $id,
                )),
            );
        };
    }

    #[test]
    fn test_two_players_in_the_zone() {
        let mut meter = helpers::init_();
        main_character_enters!(meter, "MAIN_CH1", 1);

        let zone_stats = stats(&meter, StatType::Zone);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.damage, 0.0);

        character_enters!(meter, "CH1", 2);
        let zone_stats = stats(&meter, StatType::Zone);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH1").unwrap();
        assert_eq!(player_stats.damage, 0.0);

        register_message(&mut meter, &Message::Leave(message::Leave::new(1)));
        let zone_stats = stats(&meter, StatType::Zone);
        assert!(zone_stats.iter().find(|s| s.player == "CH1").is_none());
    }

    #[test]
    fn test_overall_damage() {
        let mut meter = helpers::init_();
        main_character_enters!(meter, "MAIN_CH1", 1);
        let zone_stats = stats(&meter, StatType::Zone);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.damage, 0.0);

        attack!(meter, 1);
        let zone_stats = stats(&meter, StatType::Zone);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.damage, 10.0);

        character_enters!(meter, "CH1", 2);
        let zone_stats = stats(&meter, StatType::Zone);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH1").unwrap();
        assert_eq!(player_stats.damage, 0.0);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.damage, 10.0);

        attack!(meter, 1);
        let zone_stats = stats(&meter, StatType::Zone);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH1").unwrap();
        assert_eq!(player_stats.damage, 0.0);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.damage, 20.0);
    }

    #[test]
    fn test_last_fight_damage() {
        let mut meter = helpers::init_();
        main_character_enters!(meter, "MAIN_CH1", 1);
        let zone_stats = stats(&meter, StatType::LastFight);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.damage, 0.0);

        attack!(meter, 1);
        let zone_stats = stats(&meter, StatType::LastFight);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.damage, 10.0);
    }

    #[test]
    fn test_last_fight_management() {
        // session should be started when first player attacks
        // damage should be 0 when all players were out of combat and some player attacks

        let mut meter = helpers::init_();
        main_character_enters!(meter, "MAIN_CH1", 1);
        let zone_stats = stats(&meter, StatType::LastFight);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.damage, 0.0);

        attack!(meter, 1);
        let zone_stats = stats(&meter, StatType::LastFight);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.damage, 10.0);

        character_enters!(meter, "CH1", 2);
        let zone_stats = stats(&meter, StatType::Zone);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH1").unwrap();
        assert_eq!(player_stats.damage, 0.0);

        attack!(meter, 2);
        let zone_stats = stats(&meter, StatType::LastFight);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH1").unwrap();
        assert_eq!(player_stats.damage, 10.0);

        character_enters!(meter, "CH2", 3);
        let zone_stats = stats(&meter, StatType::Zone);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH2").unwrap();
        assert_eq!(player_stats.damage, 0.0);

        attack!(meter, 3);
        let zone_stats = stats(&meter, StatType::LastFight);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH2").unwrap();
        assert_eq!(player_stats.damage, 10.0);

        combat_leave!(meter, 1);
        let zone_stats = stats(&meter, StatType::LastFight);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.damage, 10.0);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH1").unwrap();
        assert_eq!(player_stats.damage, 10.0);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH2").unwrap();
        assert_eq!(player_stats.damage, 10.0);

        combat_leave!(meter, 2);
        let zone_stats = stats(&meter, StatType::LastFight);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.damage, 10.0);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH1").unwrap();
        assert_eq!(player_stats.damage, 10.0);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH2").unwrap();
        assert_eq!(player_stats.damage, 10.0);

        combat_leave!(meter, 3);
        let zone_stats = stats(&meter, StatType::LastFight);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.damage, 10.0);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH1").unwrap();
        assert_eq!(player_stats.damage, 10.0);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH2").unwrap();
        assert_eq!(player_stats.damage, 10.0);

        attack!(meter, 1);

        combat_leave!(meter, 3);
        let zone_stats = stats(&meter, StatType::LastFight);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.damage, 10.0);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH1").unwrap();
        assert_eq!(player_stats.damage, 0.0);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH2").unwrap();
        assert_eq!(player_stats.damage, 0.0);
    }

    #[test]
    fn test_fame_statistics() {
        let mut meter = helpers::init_();
        main_character_enters!(meter, "MAIN_CH1", 1);

        let zone_stats = stats(&meter, StatType::Zone);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.fame_per_minute, 0);

        helpers::sleep(1000 * 60);
        let zone_stats = stats(&meter, StatType::Zone);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.fame_per_minute, 0);

        register_message(
            &mut meter,
            &Message::FameUpdate(message::FameUpdate::new(1)),
        );
        let zone_stats = stats(&meter, StatType::Zone);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.fame_per_minute, 100);
    }
}
