use protocol16::Parameters;

#[derive(Debug)]
pub struct Packet {
    pub code: usize,
    pub parameters: Parameters
}