use std::collections::HashMap;

use cpython::PythonObject;
use cpython::ToPyObject;
use cpython::Python;
use cpython::PyObject;
use cpython::PyDict;
use cpython::PyList;

use super::traits::DamageStats;

#[derive(Debug, PartialEq, Clone)]
pub struct Party {
    pub id: usize,
    pub members: Vec<String>
}

impl Party {
    pub fn new(id: usize, members: &std::vec::Vec<std::string::String>) -> Self {
        Self {
            id,
            members: members.clone(),
        }
    }

    pub fn add_member(&mut self, member_name: &str) {
        self.members.push(member_name.to_string());
    }

    pub fn includes(&self, other: &str) -> bool {
        return self.members.iter().find(|m| **m == other).is_some();
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerStatistics {
    pub player: String,
    pub damage: f32,
    pub time_in_combat: f32,
    pub dps: f32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerStatisticsVec {
    _vec: Vec<PlayerStatistics>
}

impl PlayerStatisticsVec {
    pub fn new() -> Self {
        Self {
            _vec: vec![]
        }
    }

    pub fn from(player_statistics_vec: Vec<PlayerStatistics>) -> Self {
        Self {
            _vec: player_statistics_vec
        }
    }

    pub fn merged(a: &Self, b: &Self) -> Self {
        let merged = [&a._vec[..], &b._vec[..]].concat().iter().fold(
            HashMap::<String, PlayerStatistics>::new(),
            |mut acc, stat| {
                acc.entry(stat.player.clone())
                    .and_modify(|s| {
                        s.damage += stat.damage;
                        s.time_in_combat += stat.time_in_combat;
                        s.dps = s.dps();
                    })
                    .or_insert(stat.clone());
                acc
            },
        );

        Self {
            _vec: merged.iter().map(|(_, v)| v.clone()).collect()
        }
    }
}

impl DamageStats for PlayerStatistics {
    fn damage(&self) -> f32 {
        self.damage
    }
    fn time_in_combat(&self) -> f32 {
        self.time_in_combat
    }
}


impl ToPyObject for PlayerStatistics {
    type ObjectType = PyObject;
    fn to_py_object(&self, py: Python) -> Self::ObjectType {
        let stats = PyDict::new(py);

        stats
            .set_item(py, "player", self.player.to_py_object(py))
            .unwrap();
        stats
            .set_item(py, "damage", self.damage.to_py_object(py))
            .unwrap();
        stats
            .set_item(py, "time_in_combat", self.time_in_combat.to_py_object(py))
            .unwrap();
        stats
            .set_item(py, "dps", self.dps.to_py_object(py))
            .unwrap();

        stats.into_object()
    }
}

impl ToPyObject for PlayerStatisticsVec {
    type ObjectType = PyList;

    fn into_py_object(self, py: Python) -> Self::ObjectType {
        self._vec.into_py_object(py)
    }

    fn to_py_object(&self, py: Python) -> Self::ObjectType {
        self._vec.clone().into_py_object(py)
    }
}