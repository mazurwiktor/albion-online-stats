use protocol16::Parameters;
use protocol16::Value;
use super::packet::Packet;

#[derive(Debug)]
pub struct ChatSay {
    pub source: usize,
    pub source_name: String,
    pub text: String
}

impl ChatSay {
    fn encode(val: Parameters) -> Option<Message> {
        let source =  if let Value::Short(v) = val.get(&0)? { *v as usize } else { return None };
        let source_name =  if let Value::String(v) = val.get(&1)? { v.clone() } else { return None };
        let text =  if let Value::String(v) = val.get(&2)? { v.clone() } else { return None };

        Some(Message::ChatSay(Self {source, source_name, text}))
    }
}

#[derive(Debug)]
pub enum Message {
    ChatSay(ChatSay)
}

impl Packet {
    pub fn decode(self) -> Option<Message> {
        match self.code {
            63 => ChatSay::encode(self.parameters),
             _ => None
        }
    }
}