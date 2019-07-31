extern crate packet_sniffer;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use packet_sniffer::UdpPacket;

fn main() {
    let (tx, rx): (Sender<UdpPacket>, Receiver<UdpPacket>) = mpsc::channel();

    packet_sniffer::receive(tx);

    loop {
        println!("{:?}", rx.recv());
    }
}
