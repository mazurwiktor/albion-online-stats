use std::fs::File;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use log::*;
use simplelog::*;

use packet_sniffer::UdpPacket;
use photon_decode::Photon;

use crate::game::World;
use crate::meter::Meter;
use crate::publisher::Publisher;

pub use crate::publisher::Subscribers;


use crate::translate::udp_packet_to_game_events;


pub enum InitializationError {
    NetworkInterfaceListMissing,
}

pub fn initialize(subscribers: Subscribers) -> Result<(), InitializationError> {
    initialize_logging();
    
    if let Ok(interfaces) = packet_sniffer::network_interfaces() {
        thread::spawn(move || {
            let (tx, rx): (Sender<UdpPacket>, Receiver<UdpPacket>) = channel();

            let mut photon = Photon::new();
            let mut world = World::new();
            let mut publisher = Publisher::new(subscribers);

            packet_sniffer::receive(interfaces, tx);
            info!("Listening to network packets...");
            loop {
                if let Ok(packet) = rx.recv() {
                    udp_packet_to_game_events(&mut world, &mut photon, &packet)
                    .into_iter()
                    .for_each(|e| {
                        publisher.publish(&e);
                    });
                }
            }
        });
    } else {
        return Err(InitializationError::NetworkInterfaceListMissing);
    }

    Ok(())
}

fn initialize_logging() {
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Debug,
        Config::default(),
        File::create("damage-meter.log").unwrap(),
    )]).unwrap();
}