use crate::photon_messages::messages;

use super::events;

use log::*;

#[derive(Debug, PartialEq, Default)]
struct Item {
    name: String,
    id: u32,
}

#[derive(Debug, PartialEq, Default)]
pub struct Party {
    main_player_name: Option<String>,
    items: Vec<Item>,
}

impl Party {
    pub fn set_main_player_name(&mut self, name: &str) {
        self.main_player_name = Some(name.to_owned());
    }

    pub fn player_left(&mut self, left: messages::PartyPlayerLeft) -> Option<events::Event> {
        let main_player_name = self.main_player_name.as_ref()?;
        let party_item = self.items.iter().find(|i| i.id == into_id(&left.party_structure))?;
        if &party_item.name == main_player_name {
            return self.disbanded();
        }

        let index = self.items.iter().position(|i| i.id == into_id(&left.party_structure))?;
        self.items.remove(index);

        self.game_event()
    }

    pub fn single_player_joined(&mut self, joined: messages::PartyPlayerJoined) -> Option<events::Event> {
        self.items.push(Item{name: joined.name.clone(), id: into_id(&joined.party_structure)});

        self.game_event()
    }

    pub fn joined(&mut self, joined: messages::PartyJoined) -> Option<events::Event> {
        let structures_and_names = joined
            .party_structures
            .iter()
            .zip(joined.character_names.iter());

        self.items = structures_and_names
            .map(|(structure, name)| Item {
                name: name.clone(),
                id: into_id(&structure),
            })
            .collect();

        self.game_event()
    }

    pub fn disbanded(&mut self) -> Option<events::Event> {
        self.items = vec![];
        self.game_event()
    }

    fn game_event(&self) -> Option<events::Event> {
        let names : Vec<String> = self.items.iter().map(|i| i.name.clone()).collect();
        let evt = Some(events::Event::UpdateParty(events::Party{player_names: names}));

        info!("Party changed: {:?}", evt);
        evt
    }
}

fn into_id(party_structure: &[u32]) -> u32 {
    party_structure.into_iter().fold(0, |acc, x| acc + x)
}
