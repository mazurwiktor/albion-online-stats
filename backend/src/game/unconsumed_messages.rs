use super::player::{DynamicId};
use crate::photon_messages::Message;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Default)]
pub struct UnconsumedMessages{
    messages: HashMap<u32, Vec<Message>>
}

impl UnconsumedMessages {
    pub fn add(&mut self, msg: Message, id: DynamicId) {
        self.messages.entry(id.into()).or_insert(vec![]).push(msg);
    }

    pub fn get_for_id(&mut self, id: DynamicId) -> Option<Vec<Message>> {
        let messages = self.messages.get(&id.into())?;
        let result = messages.clone();
        self.messages.remove(&id.into());

        return Some(result);
    }
}