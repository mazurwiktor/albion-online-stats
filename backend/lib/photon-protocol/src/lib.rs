mod decode;
mod layout;

pub use decode::*;
pub use layout::*;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::{Cursor, Read};

impl TryFrom<(&ReliableCommand, &mut Cursor<&[u8]>)> for Message {
    type Error = &'static str;

    fn try_from(message: (&ReliableCommand, &mut Cursor<&[u8]>)) -> Result<Self, Self::Error> {
        let (command, cursor) = message;
        let _: u8 = cursor.decode()?;
        let msg_type: u8 = cursor.decode()?;
        match msg_type {
            2 => {
                let v = cursor.decode()?;
                Ok(Message::Request(v))
            }
            3 => {
                let v = cursor.decode()?;
                Ok(Message::Response(v))
            }
            4 => {
                let v = cursor.decode()?;
                Ok(Message::Event(v))
            }
            _ => {
                cursor.set_position(cursor.position() + command.msg_len as u64 - 2);
                Err("Unknown message")
            }
        }
    }
}

pub struct Photon {
    fragments: HashMap<u32, Vec<ReliableFragment>>,
}

impl Photon {
    pub fn new() -> Self {
        Self {
            fragments: HashMap::new(),
        }
    }

    pub fn try_decode(&mut self, payload: &[u8]) -> PhotonDecodeResult<Vec<Message>> {
        let mut result: Vec<Message> = vec![];
        let mut cursor = Cursor::new(payload);
        for _ in 0..Decode::<PhotonHeader>::decode(&mut cursor)?.command_count {
            let command = Decode::<Command>::decode(&mut cursor)?;
            match command {
                Command::SendReliable(c) | Command::SendUnreliable(c) => {
                    if let Ok(message) = Message::try_from((&c, &mut cursor)) {
                        result.push(message);
                    }
                }
                Command::SendReliableFragment(fragment) => {
                    self.fragments
                        .entry(fragment.sequence_number)
                        .or_insert(vec![])
                        .push(fragment.clone());
                    if let Ok(message) = self.decode_reliable_fragment(&fragment) {
                        result.push(message);
                        self.fragments.remove(&fragment.sequence_number).unwrap();
                    }
                }
                _ => {}
            }
        }

        Ok(result)
    }

    pub fn decode(&mut self, payload: &[u8]) -> Vec<Message> {
        if let Ok(messages) = self.try_decode(payload) {
            return messages;
        }
        vec![]
    }

    fn decode_reliable_fragment(
        &mut self,
        fragment: &ReliableFragment,
    ) -> PhotonDecodeResult<Message> {
        if let Some(fragments) = self.fragments.get(&fragment.sequence_number) {
            if fragments.len() == fragment.fragment_count as usize {
                let mut buf = Vec::<u8>::new();
                for fragment in fragments {
                    buf.extend(fragment.payload.iter());
                }
                let mut c = Cursor::new(&buf[..]);
                return Message::try_from((&fragment.reliable_command, &mut c));
            }
        }
        Err("Not enough information to decode the fragment")
    }
}
