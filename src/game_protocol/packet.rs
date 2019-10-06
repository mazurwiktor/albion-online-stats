use photon_protocol::Parameters;

#[derive(Debug)]
pub struct Packet {
    pub code: usize,
    pub parameters: Parameters,
}
