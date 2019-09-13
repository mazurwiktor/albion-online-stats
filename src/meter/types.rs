#[derive(Debug, PartialEq)]
pub struct PlayerStatistics {
    pub player: String,
    pub damage: f32,
    pub time_in_combat: f32,
    pub dps: f32
}