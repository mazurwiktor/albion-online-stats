mod decode;
mod error;
mod layout;

pub use decode::*;
use error::*;
pub use layout::*;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::Cursor;

impl TryFrom<(&ReliableCommand, &mut Cursor<&[u8]>)> for Message {
    type Error = PhotonDecodeError;

    fn try_from(message: (&ReliableCommand, &mut Cursor<&[u8]>)) -> Result<Self, Self::Error> {
        let (command, cursor) = message;
        let _: u8 = cursor.decode()?;
        let msg_type: u8 = cursor.decode()?;
        match msg_type {
            2 => {
                let v = cursor.decode().map_err(|e| e.extend("Request".into()))?;
                Ok(Message::Request(v))
            }
            3 => {
                let v = cursor.decode().map_err(|e| e.extend("Response".into()))?;
                Ok(Message::Response(v))
            }
            4 => {
                let v = cursor.decode().map_err(|e| e.extend("Event".into()))?;
                Ok(Message::Event(v))
            }
            _ => {
                let msg_header_len = 2;
                cursor.set_position(cursor.position() + command.msg_len as u64 - msg_header_len);
                Err(PhotonDecodeError::from(format!(
                    "Unknown message ({:#X})",
                    msg_type
                )))
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

    pub fn try_decode(
        &mut self,
        payload: &[u8],
    ) -> PhotonDecodeResult<Vec<PhotonDecodeResult<Message>>> {
        let mut cursor = Cursor::new(payload);
        Ok(
            (0..Decode::<PhotonHeader>::decode(&mut cursor)?.command_count)
                .into_iter()
                .map(|_| {
                    Decode::<Command>::decode(&mut cursor).map_or_else(
                        |e| Some(Err(e)),
                        |command| match command {
                            Command::SendReliable(c) | Command::SendUnreliable(c) => Some(
                                Message::try_from((&c, &mut cursor))
                                    .map_err(|e| e.extend("SendReliable".into())),
                            ),
                            Command::SendReliableFragment(fragment) => {
                                self.fragments
                                    .entry(fragment.sequence_number)
                                    .or_insert(vec![])
                                    .push(fragment.clone());
                                self.decode_reliable_fragment(&fragment).map(|msg| {
                                    self.fragments.remove(&fragment.sequence_number).unwrap();
                                    msg.map_err(|e| e.extend("Fragment".into()))
                                })
                            }
                            _ => None,
                        },
                    )
                })
                .filter_map(|v| v)
                .collect(),
        )
    }

    pub fn decode(&mut self, payload: &[u8]) -> Vec<Message> {
        if let Ok(messages) = self.try_decode(payload) {
            return messages.into_iter().filter_map(Result::ok).collect();
        }
        vec![]
    }

    fn decode_reliable_fragment(
        &mut self,
        fragment: &ReliableFragment,
    ) -> Option<PhotonDecodeResult<Message>> {
        if let Some(fragments) = self.fragments.get(&fragment.sequence_number) {
            if fragments.len() == fragment.fragment_count as usize {
                let mut buf = Vec::<u8>::new();
                for fragment in fragments {
                    buf.extend(fragment.payload.iter());
                }
                let mut c = Cursor::new(&buf[..]);
                return Some(Message::try_from((&fragment.reliable_command, &mut c)));
            }
        }
        None
    }
}
