#![allow(dead_code)]

use crate::id_cache::StaticId;
use crate::game_messages;

#[derive(Debug, PartialEq)]
pub struct Player {
    pub id: StaticId,
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct Damage {
    pub source: StaticId,
    pub target: Option<StaticId>,
    pub value: f32
}

#[derive(Debug, PartialEq)]
pub struct Fame {
    pub source: StaticId,
    pub value: f32
}

#[derive(Debug, PartialEq)]
pub struct Items {
    pub source: StaticId,
    pub value: game_messages::Items
}

#[derive(Debug, PartialEq)]
pub enum Events {
    PlayerAppeared(Player),
    DamageDone(Damage),
    HealthReceived(Damage),
    ZoneChange,
    EnterCombat(Player),
    LeaveCombat(Player),
    UpdateFame(Fame),
    UpdateItems(Items)
}