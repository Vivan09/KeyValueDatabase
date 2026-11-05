use super::{DbResult, Engine};
use crate::errors::DbEngineError;
use crate::types::Value;

/// Wrapper, ktory povoli len citanie. Kazdy zapis vrati `DbEngineError::ReadOnly`.
pub struct ReadOnlyEngine<E: Engine> {
    inner: E,
}

impl<E: Engine> ReadOnlyEngine<E> {
    pub fn new(inner: E) -> Self {
        ReadOnlyEngine { inner }
    }
}

impl<E: Engine> Engine for ReadOnlyEngine<E> {
    fn get(&self, key: &str) -> Option<&Value> {
        self.inner.get(key)
    }

    fn exist(&self, key: &str) -> bool {
        self.inner.exist(key)
    }

    fn put(&mut self, _key: String, _value: Value) -> DbResult<Option<Value>> {
        Err(DbEngineError::ReadOnly)
    }

    fn del(&mut self, _key: &str) -> DbResult<Option<Value>> {
        Err(DbEngineError::ReadOnly)
    }

    fn lpush(&mut self, _key: String, _value: String) -> DbResult<usize> {
        Err(DbEngineError::ReadOnly)
    }

    fn lpop(&mut self, _key: &str) -> DbResult<Option<String>> {
        Err(DbEngineError::ReadOnly)
    }

    fn dbsize(&self) -> usize {
        self.inner.dbsize()
    }

    fn flushdb(&mut self) -> DbResult<usize> {
        Err(DbEngineError::ReadOnly)
    }

    fn entries(&self) -> Vec<(&str, &Value)> {
        self.inner.entries()
    }

    fn keys(&self, pattern: &str) -> Vec<&str> {
        self.inner.keys(pattern)
    }

    fn mget(&self, keys: &[String]) -> Vec<Option<&Value>> {
        self.inner.mget(keys)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::MemoryEngine;

    #[test]
    fn reads_allowed_writes_rejected() {
        let mut mem = MemoryEngine::new();
        mem.put("a".into(), Value::Integer(1)).unwrap();
        let mut db = ReadOnlyEngine::new(mem);

        assert_eq!(db.get("a"), Some(&Value::Integer(1)));
        assert_eq!(db.dbsize(), 1);
        assert!(matches!(db.put("b".into(), Value::Integer(2)), Err(DbEngineError::ReadOnly)));
        assert!(matches!(db.del("a"), Err(DbEngineError::ReadOnly)));
        assert!(matches!(db.lpush("l".into(), "x".into()), Err(DbEngineError::ReadOnly)));
        assert!(matches!(db.lpop("l"), Err(DbEngineError::ReadOnly)));
        assert!(matches!(db.flushdb(), Err(DbEngineError::ReadOnly)));
        assert_eq!(db.dbsize(), 1);
    }
}
