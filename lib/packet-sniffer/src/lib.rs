extern crate pnet;

use std::net::IpAddr;
use std::sync::mpsc::Sender;
use std::thread;

use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::udp;
use pnet::packet::Packet;
use pnet::util::MacAddr;

#[derive(Clone, Debug)]
pub struct UdpPacket {
    pub interface_name: String,
    pub source_address: IpAddr,
    pub source_port: u16,
    pub destination_address: IpAddr,
    pub destination_port: u16,
    pub length: u16,
    pub payload: Vec<u8>
}

pub fn receive(tx: Sender<UdpPacket>) {
    use pnet::datalink::Channel::Ethernet;
    let interfaces = datalink::interfaces();
    let up_interface = datalink::interfaces()
        .into_iter()
        .filter(|i| !i.is_loopback() && !i.is_up())
        .next();

    let any_interface = datalink::interfaces()
        .into_iter()
        .filter(|i| !i.is_loopback())
        .next();
    
    let interface = if up_interface.is_some() {
        up_interface.unwrap()
    } else {
        any_interface.unwrap()
    };


    // Create a channel to receive on
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("packetdump: unhandled channel type: {}"),
        Err(e) => panic!("packetdump: unable to create channel: {}", e),
    };

    thread::spawn(move || {
        loop {
            let mut buf: [u8; 1600] = [0u8; 1600];
            let mut fake_ethernet_frame = MutableEthernetPacket::new(&mut buf[..]).unwrap();
            match rx.next() {
                Ok(packet) => {
                    if cfg!(target_os = "macos")
                        && interface.is_up()
                        && !interface.is_broadcast()
                        && !interface.is_loopback()
                        && interface.is_point_to_point()
                    {
                        // Maybe is TUN interface
                        let version = Ipv4Packet::new(&packet).unwrap().get_version();
                        if version == 4 {
                            fake_ethernet_frame.set_destination(MacAddr(0, 0, 0, 0, 0, 0));
                            fake_ethernet_frame.set_source(MacAddr(0, 0, 0, 0, 0, 0));
                            fake_ethernet_frame.set_ethertype(EtherTypes::Ipv4);
                            fake_ethernet_frame.set_payload(&packet);
                            handle_ethernet_frame(&interface, &fake_ethernet_frame.to_immutable(), &tx);
                            continue;
                        } else if version == 6 {
                            fake_ethernet_frame.set_destination(MacAddr(0, 0, 0, 0, 0, 0));
                            fake_ethernet_frame.set_source(MacAddr(0, 0, 0, 0, 0, 0));
                            fake_ethernet_frame.set_ethertype(EtherTypes::Ipv6);
                            fake_ethernet_frame.set_payload(&packet);
                            handle_ethernet_frame(&interface, &fake_ethernet_frame.to_immutable(), &tx);
                            continue;
                        }
                    }
                    handle_ethernet_frame(&interface, &EthernetPacket::new(packet).unwrap(), &tx);
                }
                Err(e) => panic!("packetdump: unable to receive packet: {}", e),
            }
        }
    });
}

fn handle_ethernet_frame(interface: &NetworkInterface, ethernet: &EthernetPacket, tx: &Sender<UdpPacket>) {
    let interface_name = &interface.name[..];
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => handle_ipv4_packet(interface_name, ethernet, &tx),
        EtherTypes::Ipv6 => handle_ipv6_packet(interface_name, ethernet, &tx),
        _ => {}
    }
}

fn handle_ipv4_packet(interface_name: &str, ethernet: &EthernetPacket, tx: &Sender<UdpPacket>) {
    let header = Ipv4Packet::new(ethernet.payload());
    if let Some(header) = header {
        handle_transport_protocol(
            interface_name,
            IpAddr::V4(header.get_source()),
            IpAddr::V4(header.get_destination()),
            header.get_next_level_protocol(),
            header.payload(),
            &tx
        );
    }
}

fn handle_ipv6_packet(interface_name: &str, ethernet: &EthernetPacket, tx: &Sender<UdpPacket>) {
    let header = Ipv6Packet::new(ethernet.payload());
    if let Some(header) = header {
        handle_transport_protocol(
            interface_name,
            IpAddr::V6(header.get_source()),
            IpAddr::V6(header.get_destination()),
            header.get_next_header(),
            header.payload(),
            &tx
        );
    }
}

fn handle_transport_protocol(
    interface_name: &str,
    source: IpAddr,
    destination: IpAddr,
    protocol: IpNextHeaderProtocol,
    packet: &[u8],
    tx: &Sender<UdpPacket>
) {
    match protocol {
        IpNextHeaderProtocols::Udp => {
            handle_udp_packet(interface_name, source, destination, packet, &tx)
        }
        _ => {}
    }
}

fn handle_udp_packet(interface_name: &str, source: IpAddr, destination: IpAddr, packet: &[u8], tx: &Sender<UdpPacket>) {
    let udp = udp::UdpPacket::new(packet);

    if let Some(udp) = udp {
        tx.send(UdpPacket{
            interface_name: String::from(interface_name),
            source_address: source,
            source_port: udp.get_source(),
            destination_address: destination,
            destination_port: udp.get_destination(),
            length: udp.get_length(),
            payload: Vec::from(udp.payload())
        }).unwrap();
    }
}
