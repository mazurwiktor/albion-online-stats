use super::traits::DamageStats;

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerStatistics {
    pub player: String,
    pub damage: f32,
    pub time_in_combat: f32,
    pub dps: f32,
}

pub type PlayerStatisticsVec = Vec<PlayerStatistics>;

impl DamageStats for PlayerStatistics {
    fn damage(&self) -> f32 {
        self.damage
    }
    fn time_in_combat(&self) -> f32 {
        self.time_in_combat
    }
}
