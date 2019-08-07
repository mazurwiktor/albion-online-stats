extern crate bytes;
extern crate packet_sniffer;
extern crate protocol16;

use std::fs::File;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use simplelog::*;
use log::*;


use packet_sniffer::UdpPacket;

mod game_protocol;
mod meter;

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Error, Config::default(), TerminalMode::Mixed).unwrap(),
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            File::create("damage-meter.log").unwrap(),
        ),
    ])
    .unwrap();

    let (tx, rx): (Sender<UdpPacket>, Receiver<UdpPacket>) = mpsc::channel();

    packet_sniffer::receive(tx);

    let mut meter = meter::Meter::new();

    loop {
        if let Ok(packet) = rx.recv() {
            if packet.destination_port != 5056 && packet.source_port != 5056 {
                continue;
            }
            let messages = game_protocol::decode(&packet.payload);

            for msg in messages {
                debug!("Found message {:?}", msg);

                match msg {
                    game_protocol::Message::NewCharacter(msg) => meter.register_player(&msg.character_name, msg.source),
                    game_protocol::Message::CharacterStats(msg) => meter.register_player(&msg.character_name, msg.source),
                    game_protocol::Message::HealthUpdate(msg) => meter.register_damage_dealt(msg.source, msg.value),
                    game_protocol::Message::RegenerationHealthChanged(msg) => {
                        match msg.regeneration_rate {
                            Some(_) => meter.register_combat_leave(msg.source),
                            None => meter.register_combat_enter(msg.source)
                        }
                    }
                    _ => {}
                }
            }
        }


    }
}