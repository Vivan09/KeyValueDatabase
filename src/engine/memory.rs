use std::collections::{HashMap, VecDeque};

use super::{DbResult, Engine, glob_match};
use crate::errors::DbEngineError;
use crate::types::Value;

#[derive(Default)]
pub struct MemoryEngine {
    storage: HashMap<String, Value>,
}

impl MemoryEngine {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Engine for MemoryEngine {
    fn get(&self, key: &str) -> Option<&Value> {
        self.storage.get(key)
    }

    fn exist(&self, key: &str) -> bool {
        self.storage.contains_key(key)
    }

    fn put(&mut self, key: String, value: Value) -> DbResult<Option<Value>> {
        Ok(self.storage.insert(key, value))
    }

    fn del(&mut self, key: &str) -> DbResult<Option<Value>> {
        Ok(self.storage.remove(key))
    }

    fn lpush(&mut self, key: String, value: String) -> DbResult<usize> {
        let entry = self
            .storage
            .entry(key)
            .or_insert_with(|| Value::List(VecDeque::new()));
        match entry {
            Value::List(list) => {
                list.push_front(value);
                Ok(list.len())
            }
            _ => Err(DbEngineError::WrongType),
        }
    }

    fn lpop(&mut self, key: &str) -> DbResult<Option<String>> {
        match self.storage.get_mut(key) {
            None => Ok(None),
            Some(Value::List(list)) => {
                let value = list.pop_front();
                if list.is_empty() {
                    self.storage.remove(key);
                }
                Ok(value)
            }
            Some(_) => Err(DbEngineError::WrongType),
        }
    }

    fn dbsize(&self) -> usize {
        self.storage.len()
    }

    fn flushdb(&mut self) -> DbResult<usize> {
        let size = self.storage.len();
        self.storage.clear();
        Ok(size)
    }

    fn entries(&self) -> Vec<(&str, &Value)> {
        let mut entries: Vec<(&str, &Value)> =
            self.storage.iter().map(|(k, v)| (k.as_str(), v)).collect();
        entries.sort_by(|a, b| a.0.cmp(b.0));
        entries
    }

    fn keys(&self, pattern: &str) -> Vec<&str> {
        let mut keys: Vec<&str> = self
            .storage
            .keys()
            .map(String::as_str)
            .filter(|key| glob_match(pattern, key))
            .collect();
        keys.sort_unstable();
        keys
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_get_del() {
        let mut db = MemoryEngine::new();
        assert_eq!(db.put("a".into(), Value::Integer(1)).unwrap(), None);
        assert_eq!(
            db.put("a".into(), Value::Integer(2)).unwrap(),
            Some(Value::Integer(1))
        );
        assert_eq!(db.get("a"), Some(&Value::Integer(2)));
        assert!(db.exist("a"));
        assert_eq!(db.del("a").unwrap(), Some(Value::Integer(2)));
        assert!(!db.exist("a"));
        assert_eq!(db.del("a").unwrap(), None);
    }

    #[test]
    fn lists() {
        let mut db = MemoryEngine::new();
        assert_eq!(db.lpush("l".into(), "x".into()).unwrap(), 1);
        assert_eq!(db.lpush("l".into(), "y".into()).unwrap(), 2);
        assert_eq!(db.lpop("l").unwrap(), Some("y".into()));
        assert_eq!(db.lpop("l").unwrap(), Some("x".into()));
        assert!(!db.exist("l"));
        assert_eq!(db.lpop("l").unwrap(), None);

        db.put("s".into(), Value::String("str".into())).unwrap();
        assert!(matches!(db.lpush("s".into(), "x".into()), Err(DbEngineError::WrongType)));
        assert!(matches!(db.lpop("s"), Err(DbEngineError::WrongType)));
    }

    #[test]
    fn size_flush_mget_keys() {
        let mut db = MemoryEngine::new();
        db.put("user:1".into(), Value::Integer(1)).unwrap();
        db.put("user:2".into(), Value::Integer(2)).unwrap();
        db.put("admin".into(), Value::Integer(3)).unwrap();
        assert_eq!(db.dbsize(), 3);
        assert_eq!(db.keys("user:*"), vec!["user:1", "user:2"]);
        assert_eq!(db.keys("*"), vec!["admin", "user:1", "user:2"]);
        assert_eq!(
            db.mget(&["admin".into(), "nope".into()]),
            vec![Some(&Value::Integer(3)), None]
        );
        assert_eq!(db.flushdb().unwrap(), 3);
        assert_eq!(db.dbsize(), 0);
    }
}
