extern crate bytes;

extern crate packet_sniffer;
extern crate protocol16;


use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use packet_sniffer::UdpPacket;

mod protocol;

fn main() {
    let (tx, rx): (Sender<UdpPacket>, Receiver<UdpPacket>) = mpsc::channel();

    packet_sniffer::receive(tx);

    loop {
        if let Ok(packet) = rx.recv() {
            if packet.destination_port != 5056 && packet.source_port != 5056 {
                continue;
            }
            protocol::decode(&packet.payload);
        }


    }
}