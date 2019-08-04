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

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed).unwrap(),
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            File::create("packet-sniffer.log").unwrap(),
        ),
    ])
    .unwrap();

    let (tx, rx): (Sender<UdpPacket>, Receiver<UdpPacket>) = mpsc::channel();

    packet_sniffer::receive(tx);

    loop {
        if let Ok(packet) = rx.recv() {
            if packet.destination_port != 5056 && packet.source_port != 5056 {
                continue;
            }
            let messages = game_protocol::decode(&packet.payload);

            for msg in messages {
                info!("Found message {:?}", msg);
            }
        }


    }
}