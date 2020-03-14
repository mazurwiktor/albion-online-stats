#![allow(dead_code)]
use std::collections::HashMap;

use super::player::{DynamicId, PlayerName, StaticId};

pub type DynIdToStaticId = HashMap<DynamicId, StaticId>;
pub type StaticIdToName = HashMap<StaticId, PlayerName>;

#[derive(Debug, PartialEq, Default)]
pub struct IdCache {
    dyn_id_to_static_id: DynIdToStaticId,
    static_id_to_name: StaticIdToName,
    last_id: u32,
}

impl IdCache {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn save(&mut self, dynamic_id: DynamicId, name: &str) {
        let player_name: PlayerName = name.to_owned().into();
        if let Some(static_id) = self.find_static_id(&player_name) {
            if let Some(dyn_id) = self.find_dynamic_id(&static_id) {
                self.dyn_id_to_static_id.remove(&dyn_id);
            }
            self.dyn_id_to_static_id.insert(dynamic_id, static_id);
        } else {
            let static_id: StaticId = self.last_id.into();
            self.last_id += 1;

            self.dyn_id_to_static_id.insert(dynamic_id, static_id);
            self.static_id_to_name.insert(static_id, player_name);
        }
    }

    fn find_static_id(&self, name: &PlayerName) -> Option<StaticId> {
        self.static_id_to_name
            .iter()
            .find(|(_k, v)| v == &name)
            .map(|(k, _v)| *k)
    }

    fn find_dynamic_id(&self, static_id: &StaticId) -> Option<DynamicId> {
        self.dyn_id_to_static_id
            .iter()
            .find(|(_k, v)| v == &static_id)
            .map(|(k, _v)| *k)
    }

    pub fn get_dyn_id_to_static_id_map(&self) -> DynIdToStaticId {
        self.dyn_id_to_static_id.clone()
    }

    pub fn get_static_id_to_name_map(&self) -> StaticIdToName {
        self.static_id_to_name.clone()
    }

    pub fn get_static_id(&self, dynamic_id: DynamicId) -> Option<StaticId> {
        self.dyn_id_to_static_id.get(&dynamic_id).map(|i| *i)
    }

    pub fn get_name(&self, static_id: StaticId) -> Option<PlayerName> {
        self.static_id_to_name.get(&static_id).map(|i| i.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_cache() {
        let mut cache = IdCache::new();

        assert!(cache.get_static_id(DynamicId::from(1)).is_none());

        cache.save(DynamicId::from(1), "test");
        assert!(cache.get_static_id(DynamicId::from(1)).is_some());
        assert_eq!(
            cache.get_static_id(DynamicId::from(1)),
            Some(StaticId::from(0))
        );

        cache.save(DynamicId::from(2), "test");
        assert!(cache.get_static_id(DynamicId::from(1)).is_none());
        assert_eq!(
            cache.get_static_id(DynamicId::from(2)),
            Some(StaticId::from(0))
        );
    }

    #[test]
    fn test_if_id_is_unique_for_a_player() {
        let mut cache = IdCache::new();

        cache.save(DynamicId::from(1), "test");
        assert_eq!(
            cache.get_static_id(DynamicId::from(1)),
            Some(StaticId::from(0))
        );

        cache.save(DynamicId::from(2), "test2");
        assert_eq!(
            cache.get_static_id(DynamicId::from(2)),
            Some(StaticId::from(1))
        );

        cache.save(DynamicId::from(12345), "test3");
        assert_eq!(
            cache.get_static_id(DynamicId::from(12345)),
            Some(StaticId::from(2))
        );
    }

    #[test]
    fn test_finding_player_name() {
        let mut cache = IdCache::new();

        assert!(cache.get_name(StaticId::from(1)).is_none());

        cache.save(DynamicId::from(1), "test");
        assert_eq!(
            cache.get_static_id(DynamicId::from(1)),
            Some(StaticId::from(0))
        );
        assert_eq!(
            cache.get_name(StaticId::from(0)),
            Some(PlayerName::from("test".to_owned()))
        );

        cache.save(DynamicId::from(2), "test2");
        assert_eq!(
            cache.get_static_id(DynamicId::from(2)),
            Some(StaticId::from(1))
        );
        assert_eq!(
            cache.get_name(StaticId::from(1)),
            Some(PlayerName::from("test2".to_owned()))
        );
        assert_eq!(
            cache.get_name(StaticId::from(0)),
            Some(PlayerName::from("test".to_owned()))
        );
    }
}
