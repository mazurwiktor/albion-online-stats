use photon_decode::Parameters;

#[derive(Debug)]
pub struct Packet {
    pub code: usize,
    pub parameters: Parameters
}