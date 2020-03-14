use derive_more::{From, Into};

#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into, Default, Hash)]
pub struct DynamicId(u32);

#[derive(Copy, Clone, Debug, Eq, PartialEq, From, Into, Hash)]
pub struct StaticId(u32);

#[derive(Clone, Debug, Eq, PartialEq, From, Into, Default, Hash)]
pub struct PlayerName(String);

impl StaticId {
    pub fn inner(&self) -> u32 {
        self.0
    }
}

impl PlayerName {
    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}