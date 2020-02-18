//! Translates photon packets into game messages

//! Generated messages based on assets/messages.json
//! See build.rs 
pub mod messages;
/// generated from assets/item_ids.txt
/// See build.rs 
mod itemdb;

use itemdb::ITEMDB;

use std::convert::From;

pub use messages::Message;
pub use messages::into_game_message;

/// Player inventory items
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Items {
    pub weapon: Option<String>,
    pub offhand: Option<String>,
    pub helmet: Option<String>,
    pub armor: Option<String>,
    pub boots: Option<String>,
    pub bag: Option<String>,
    pub cape: Option<String>,
    pub mount: Option<String>,
    pub potion: Option<String>,
    pub food: Option<String>,
}

impl From<std::vec::Vec<u32>> for Items {
    fn from(item_array: std::vec::Vec<u32>) -> Self {
        macro_rules! extract {
            ($id:expr) => {
                item_array
                    .get($id)
                    .iter()
                    .filter(|id| **id != &0u32)
                    .map(|code| ITEMDB.get(code).map(|s| s.to_string()))
                    .filter_map(|o| o)
                    .next()
            };
        }

        Self{
            weapon: extract!(0),
            offhand: extract!(1),
            helmet: extract!(2),
            armor: extract!(3),
            boots: extract!(4),
            bag: extract!(5),
            cape: extract!(6),
            mount: extract!(7),
            potion: extract!(8),
            food: extract!(9),
        }
    }
}


#[test]
fn test_itemdb_generation() {
    assert_eq!(ITEMDB.get(&0), Some(&"UNIQUE_HIDEOUT"));
}