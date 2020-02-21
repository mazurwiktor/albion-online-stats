use std::fs::File;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
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

pub fn initialize(subscribers: Subscribers) -> Result<Arc<Mutex<Meter>>, InitializationError> {
    initialize_logging();

    let meter = Meter::new();

    let meter = Arc::new(Mutex::new(meter));
    let cloned_meter = meter.clone();
    let mut world = World::new();
    
    if let Ok(interfaces) = packet_sniffer::network_interfaces() {
        thread::spawn(move || {
            let (tx, rx): (Sender<UdpPacket>, Receiver<UdpPacket>) = channel();

            let mut photon = Photon::new();

            let consume_by_meter = move |e| {
                if let Ok(ref mut meter) = meter.lock() {
                    meter.consume(e); 
                }
            };
            let mut publisher = Publisher::new(vec![
                Box::new(consume_by_meter)
            ]);

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

    Ok(cloned_meter)
}

fn initialize_logging() {
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Debug,
        Config::default(),
        File::create("damage-meter.log").unwrap(),
    )])
    .unwrap();
}