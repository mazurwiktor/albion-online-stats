use packet_sniffer::UdpPacket;
use photon_decode::Photon;

use crate::game::Event;
use crate::photon_messages::into_game_message;
use crate::photon_messages::Message;
use crate::game::World;

static GAME_PORT : u16 = 5056;

pub fn udp_packet_to_game_events(game_world: &mut World, photon: &mut Photon, packet: &UdpPacket) -> Vec<Event> {
    if ! is_packet_valid(packet) {
        return vec![]
    }

    raw_to_photon_messages(photon, &packet.payload)
        .into_iter()
        .map(|message| game_world.transform(message))
        .flatten()
        .flatten()
        .collect()
}

fn raw_to_photon_messages(photon: &mut Photon, packet_payload: &[u8]) -> Vec<Message> {
    return photon
        .decode(packet_payload)
        .into_iter()
        .filter_map(into_game_message)
        .collect()
}

fn is_packet_valid(packet: &UdpPacket) -> bool {
    return packet.destination_port == GAME_PORT || packet.source_port == GAME_PORT;
}

#[cfg(test)]
mod tests {
    use super::*;

    
}