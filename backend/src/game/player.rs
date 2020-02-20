use derive_more::{From, Into};

#[derive(Copy, Clone, Debug, PartialEq, From, Into, Default)]
pub struct DynamicId(u32);

#[derive(Copy, Clone, Debug, PartialEq, From, Into)]
pub struct StaticId(u32);

#[derive(Debug, PartialEq, From, Into, Default)]
pub struct PlayerName(String);