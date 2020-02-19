use std::fs::File;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use log::*;
use simplelog::*;

use packet_sniffer::UdpPacket;

use photon_decode;
use photon_decode::Photon;
use crate::game;
use crate::game::{Events};
use crate::photon_messages;
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
pub use crate::photon_messages::Items;

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
    let mut world = game::World::new();
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
                        let photon_messages = photon
                            .decode(&packet.payload)
                            .into_iter()
                            .filter_map(photon_messages::into_game_message)
                            .collect();
                        register_messages(meter, &photon_messages, &mut world);
                    }
                }
            }
        });
    } else {
        return Err(InitializationError::NetworkInterfaceListMissing);
    }

    Ok(meter)
}

pub fn register_messages(meter: &mut meter::Meter, messages: &Vec<photon_messages::Message>, world: &mut game::World) {
    messages
        .iter()
        .for_each(|message| register_message(meter, &message, world));
}

fn register_message(events: &mut meter::Meter, message: &photon_messages::Message, world: &mut game::World)
{
    info!("Found message {:?}", message);
    // let events = world.transform(message);
    match message {
        photon_messages::Message::Leave(msg) => events.register_leave(msg.source).unwrap_or(()),
        photon_messages::Message::NewCharacter(msg) => {
            events.register_player(&msg.character_name, msg.source);
            events.register_item_update(msg.source, &msg.items);
        },
        photon_messages::Message::CharacterEquipmentChanged(msg) => {
            events.register_item_update(msg.source, &msg.items);
        },
        photon_messages::Message::Join(msg) => {
            events.register_main_player(&msg.character_name, msg.source)
        }
        photon_messages::Message::HealthUpdate(msg) => events
            .register_damage_dealt(msg.target, msg.value)
            .unwrap_or(()),
        photon_messages::Message::RegenerationHealthChanged(msg) => match msg.regeneration_rate {
            Some(_) => events.register_combat_leave(msg.source).unwrap_or(()),
            None => events.register_combat_enter(msg.source).unwrap_or(()),
        },
        photon_messages::Message::KnockedDown(msg) => events.register_combat_leave(msg.source).unwrap_or(()),
        photon_messages::Message::UpdateFame(msg) => events
            .register_fame_gain(msg.source, msg.fame as f32 / 10000.0)
            .unwrap_or(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use photon_messages::messages;
    use photon_messages::Items;
    use photon_messages::Message;

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

    impl NamedTesting for messages::NewCharacter {
        fn new_named(name: &str, source: usize) -> Self {
            Self {
                source: source,
                character_name: name.to_owned(),
                health: 10.0,
                max_health: 10.0,
                energy: 1.0,
                max_energy: 1.0,
                items: Items::from(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            }
        }
    }

    impl NamedTesting for messages::Join {
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

    impl Testing for messages::NewCharacter {
        fn new(source: usize) -> Self {
            messages::NewCharacter::new_named("CH1", source)
        }
    }

    impl Testing for messages::Join {
        fn new(source: usize) -> Self {
            messages::Join::new_named("MAIN_CH1", source)
        }
    }

    impl Testing for messages::Leave {
        fn new(source: usize) -> Self {
            Self { source: source }
        }
    }

    impl Testing for messages::HealthUpdate {
        fn new(source: usize) -> Self {
            Self {
                source: 200,
                target: source,
                value: -10.0,
            }
        }
    }

    impl Testing for messages::UpdateFame {
        fn new(source: usize) -> Self {
            Self {
                source: source,
                fame: 1000000,
            }
        }
    }

    impl SwitchableTesting for messages::RegenerationHealthChanged {
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
        let mut world = game::World::new();
        register_message(
            &mut meter,
            &Message::NewCharacter(messages::NewCharacter::new(1)),
            &mut world,
        );
        assert_eq!(stats(&meter, StatType::Zone).len(), 1);
    }

    #[test]
    fn test_new_player_stats() {
        let mut meter = helpers::init_();
        let mut world = game::World::new();
        register_message(
            &mut meter,
            &Message::NewCharacter(messages::NewCharacter::new(1)),
            &mut world,
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
        let mut world = game::World::new();
        register_message(
            &mut meter,
            &Message::NewCharacter(messages::NewCharacter::new(1)),
            &mut world,
        );

        let zone_stats = stats(&meter, StatType::Zone);
        assert_eq!(zone_stats[0].damage, 0.0);

        register_message(
            &mut meter,
            &Message::RegenerationHealthChanged(messages::RegenerationHealthChanged::disabled(1)),
            &mut world,
        );
        let zone_stats = stats(&meter, StatType::Zone);
        assert_eq!(zone_stats[0].damage, 0.0);

        register_message(
            &mut meter,
            &Message::HealthUpdate(messages::HealthUpdate::new(1)),
            &mut world,
        );
        let zone_stats = stats(&meter, StatType::Zone);
        assert_eq!(zone_stats[0].damage, 10.0);

        register_message(
            &mut meter,
            &Message::HealthUpdate(messages::HealthUpdate::new(1)),
            &mut world,
        );
        let zone_stats = stats(&meter, StatType::Zone);
        assert_eq!(zone_stats[0].damage, 20.0);
    }

    #[test]
    fn test_new_player_damage() {
        let mut meter = helpers::init_();
        let mut world = game::World::new();
        register_message(
            &mut meter,
            &Message::NewCharacter(messages::NewCharacter::new(1)),
            &mut world,
        );

        let zone_stats = stats(&meter, StatType::Zone);
        assert_eq!(zone_stats[0].damage, 0.0);
        assert_eq!(zone_stats[0].player, "CH1");

        register_message(
            &mut meter,
            &Message::RegenerationHealthChanged(messages::RegenerationHealthChanged::disabled(1)),
            &mut world,
        );
        register_message(
            &mut meter,
            &Message::HealthUpdate(messages::HealthUpdate::new(1)),
            &mut world,
        );
        let zone_stats = stats(&meter, StatType::Zone);
        assert_eq!(zone_stats[0].damage, 10.0);
    }

    #[test]
    fn test_new_player_damage_reset() {
        let mut meter = helpers::init_();
        let mut world = game::World::new();
        register_message(
            &mut meter,
            &Message::NewCharacter(messages::NewCharacter::new(1)),
            &mut world,
        );

        let zone_stats = stats(&meter, StatType::Zone);
        assert_eq!(zone_stats[0].damage, 0.0);
        assert_eq!(zone_stats[0].player, "CH1");

        register_message(
            &mut meter,
            &Message::RegenerationHealthChanged(messages::RegenerationHealthChanged::disabled(1)),
            &mut world
        );
        register_message(
            &mut meter,
            &Message::HealthUpdate(messages::HealthUpdate::new(1)),
            &mut world,
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
        let mut world = game::World::new();
        register_message(
            &mut meter,
            &Message::Join(messages::Join::new(1)),
            &mut world,
        );

        let zone_stats = stats(&meter, StatType::Zone);
        assert_eq!(zone_stats[0].damage, 0.0);
        assert_eq!(zone_stats[0].player, "MAIN_CH1");

        register_message(
            &mut meter,
            &Message::RegenerationHealthChanged(messages::RegenerationHealthChanged::disabled(1)),
            &mut world,
        );
        register_message(
            &mut meter,
            &Message::HealthUpdate(messages::HealthUpdate::new(1)),
            &mut world,
        );
        let zone_stats = stats(&meter, StatType::Zone);
        assert_eq!(zone_stats[0].damage, 10.0);

        register_message(&mut meter, &Message::Leave(messages::Leave::new(1)), &mut world);
        register_message(
            &mut meter,
            &Message::Join(messages::Join::new(2)),
            &mut world,
        );

        let zone_stats = stats(&meter, StatType::Zone);
        assert_eq!(zone_stats[0].damage, 0.0);

        register_message(
            &mut meter,
            &Message::RegenerationHealthChanged(messages::RegenerationHealthChanged::disabled(2)),
            &mut world,
        );
        register_message(
            &mut meter,
            &Message::HealthUpdate(messages::HealthUpdate::new(2)),
            &mut world,
        );
        let zone_stats = stats(&meter, StatType::Zone);
        assert_eq!(zone_stats[0].damage, 10.0);
    }

    macro_rules! main_character_enters {
        ($meter:expr, $world:expr, $name:expr, $id:expr) => {
            register_message(
                &mut $meter,
                &Message::Join(messages::Join::new_named($name, $id)),
                &mut $world,
            );
        };
    }

    macro_rules! character_enters {
        ($meter:expr, $world:expr, $name:expr, $id:expr) => {
            register_message(
                &mut $meter,
                &Message::NewCharacter(messages::NewCharacter::new_named($name, $id)),
                &mut $world,
            );
        };
    }

    macro_rules! attack {
        ($meter:expr, $world:expr, $id:expr) => {
            register_message(
                &mut $meter,
                &Message::RegenerationHealthChanged(messages::RegenerationHealthChanged::disabled(
                    $id,
                )),
                &mut $world,
            );
            register_message(
                &mut $meter,
                &Message::HealthUpdate(messages::HealthUpdate::new($id)),
                &mut $world,
            );
        };
    }

    macro_rules! combat_leave {
        ($meter:expr, $world:expr, $id:expr) => {
            register_message(
                &mut $meter,
                &Message::RegenerationHealthChanged(messages::RegenerationHealthChanged::enabled(
                    $id,
                )),
                &mut $world,
            );
        };
    }

    #[test]
    fn test_two_players_in_the_zone() {
        let mut meter = helpers::init_();
        let mut world = game::World::new();
        let mut world = game::World::new();
        main_character_enters!(meter, world, "MAIN_CH1", 1);

        let zone_stats = stats(&meter, StatType::Zone);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.damage, 0.0);

        character_enters!(meter, world, "CH1", 2);
        let zone_stats = stats(&meter, StatType::Zone);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH1").unwrap();
        assert_eq!(player_stats.damage, 0.0);

        register_message(&mut meter, &Message::Leave(messages::Leave::new(1)), &mut world);
        let zone_stats = stats(&meter, StatType::Zone);
        assert!(zone_stats.iter().find(|s| s.player == "CH1").is_none());
    }

    #[test]
    fn test_overall_damage() {
        let mut meter = helpers::init_();
        let mut world = game::World::new();
        main_character_enters!(meter, world, "MAIN_CH1", 1);
        let zone_stats = stats(&meter, StatType::Zone);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.damage, 0.0);

        attack!(meter, world, 1);
        let zone_stats = stats(&meter, StatType::Zone);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.damage, 10.0);

        character_enters!(meter, world, "CH1", 2);
        let zone_stats = stats(&meter, StatType::Zone);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH1").unwrap();
        assert_eq!(player_stats.damage, 0.0);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.damage, 10.0);

        attack!(meter, world, 1);
        let zone_stats = stats(&meter, StatType::Zone);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH1").unwrap();
        assert_eq!(player_stats.damage, 0.0);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.damage, 20.0);
    }

    #[test]
    fn test_last_fight_damage() {
        let mut meter = helpers::init_();
        let mut world = game::World::new();
        main_character_enters!(meter, world, "MAIN_CH1", 1);
        let zone_stats = stats(&meter, StatType::LastFight);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.damage, 0.0);

        attack!(meter, world, 1);
        let zone_stats = stats(&meter, StatType::LastFight);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.damage, 10.0);
    }

    #[test]
    fn test_last_fight_management() {
        // session should be started when first player attacks
        // damage should be 0 when all players were out of combat and some player attacks

        let mut meter = helpers::init_();
        let mut world = game::World::new();
        main_character_enters!(meter, world, "MAIN_CH1", 1);
        let zone_stats = stats(&meter, StatType::LastFight);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.damage, 0.0);

        attack!(meter, world, 1);
        let zone_stats = stats(&meter, StatType::LastFight);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.damage, 10.0);

        character_enters!(meter, world, "CH1", 2);
        let zone_stats = stats(&meter, StatType::Zone);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH1").unwrap();
        assert_eq!(player_stats.damage, 0.0);

        attack!(meter, world, 2);
        let zone_stats = stats(&meter, StatType::LastFight);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH1").unwrap();
        assert_eq!(player_stats.damage, 10.0);

        character_enters!(meter, world, "CH2", 3);
        let zone_stats = stats(&meter, StatType::Zone);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH2").unwrap();
        assert_eq!(player_stats.damage, 0.0);

        attack!(meter, world, 3);
        let zone_stats = stats(&meter, StatType::LastFight);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH2").unwrap();
        assert_eq!(player_stats.damage, 10.0);

        combat_leave!(meter, world, 1);
        let zone_stats = stats(&meter, StatType::LastFight);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.damage, 10.0);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH1").unwrap();
        assert_eq!(player_stats.damage, 10.0);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH2").unwrap();
        assert_eq!(player_stats.damage, 10.0);

        combat_leave!(meter, world, 2);
        let zone_stats = stats(&meter, StatType::LastFight);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.damage, 10.0);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH1").unwrap();
        assert_eq!(player_stats.damage, 10.0);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH2").unwrap();
        assert_eq!(player_stats.damage, 10.0);

        combat_leave!(meter, world, 3);
        let zone_stats = stats(&meter, StatType::LastFight);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.damage, 10.0);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH1").unwrap();
        assert_eq!(player_stats.damage, 10.0);
        let player_stats = zone_stats.iter().find(|s| s.player == "CH2").unwrap();
        assert_eq!(player_stats.damage, 10.0);

        attack!(meter, world, 1);

        combat_leave!(meter, world, 3);
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
        let mut world = game::World::new();
        main_character_enters!(meter, world, "MAIN_CH1", 1);

        let zone_stats = stats(&meter, StatType::Zone);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.fame_per_minute, 0);

        helpers::sleep(1000 * 60);
        let zone_stats = stats(&meter, StatType::Zone);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.fame_per_minute, 0);

        register_message(
            &mut meter,
            &Message::UpdateFame(messages::UpdateFame::new(1)),
            &mut world,
        );
        let zone_stats = stats(&meter, StatType::Zone);
        let player_stats = zone_stats.iter().find(|s| s.player == "MAIN_CH1").unwrap();
        assert_eq!(player_stats.fame_per_minute, 100);
    }
}
