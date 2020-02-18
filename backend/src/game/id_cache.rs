#![allow(dead_code)]

use bimap::BiMap;

use std::collections::HashMap;

use super::player::{StaticId, DynamicId, PlayerName};

#[derive(Debug, PartialEq, Default)]
pub struct IdCache {
    name_to_static_id: HashMap<String, u32>,
    dyn_id_to_name: BiMap<u32, String>,
    last_id: u32,
}

impl IdCache {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn save(&mut self, dynamic_id: DynamicId, name: &str) {
        if self.name_to_static_id.get(name).is_none() {
            let id = self.last_id;
            self.last_id += 1;
            self.name_to_static_id.insert(name.to_owned(), id);
            self.dyn_id_to_name.insert(dynamic_id.into(), name.to_owned());
        }

        self.dyn_id_to_name.insert(dynamic_id.into(), name.to_owned());
    }

    pub fn get_static_id(&self, dynamic_id: DynamicId) -> Option<StaticId> {
        self.dyn_id_to_name
            .get_by_left(&dynamic_id.into())
            .map(|name| {
                self.name_to_static_id
                    .get(name)
                    .map(|v| *v)
                    .map(|v| v.into())
            })
            .flatten()
    }

    pub fn get_name(&self, static_id: StaticId) -> Option<PlayerName> {
        self.name_to_static_id
            .iter()
            .find(|(_k, v)| **v == StaticId::into(static_id))
            .map(|v| v.0)
            .map(|v| v.clone().into())
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
        assert_eq!(cache.get_static_id(DynamicId::from(1)), Some(StaticId::from(0)));

        cache.save(DynamicId::from(2), "test");
        assert!(cache.get_static_id(DynamicId::from(1)).is_none());
        assert_eq!(cache.get_static_id(DynamicId::from(2)), Some(StaticId::from(0)));
    }

    #[test]
    fn test_if_id_is_unique_for_a_player() {
        let mut cache = IdCache::new();

        cache.save(DynamicId::from(1), "test");
        assert_eq!(cache.get_static_id(DynamicId::from(1)), Some(StaticId::from(0)));

        cache.save(DynamicId::from(2), "test2");
        assert_eq!(cache.get_static_id(DynamicId::from(2)), Some(StaticId::from(1)));

        cache.save(DynamicId::from(12345), "test3");
        assert_eq!(cache.get_static_id(DynamicId::from(12345)), Some(StaticId::from(2)));
    }

    #[test]
    fn test_finding_player_name() {
        let mut cache = IdCache::new();

        assert!(cache.get_name(StaticId::from(1)).is_none());

        cache.save(DynamicId::from(1), "test");
        assert_eq!(cache.get_static_id(DynamicId::from(1)), Some(StaticId::from(0)));
        assert_eq!(
            cache.get_name(StaticId::from(0)),
            Some(PlayerName::from("test".to_owned()))
        );

        cache.save(DynamicId::from(2), "test2");
        assert_eq!(cache.get_static_id(DynamicId::from(2)), Some(StaticId::from(1)));
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
