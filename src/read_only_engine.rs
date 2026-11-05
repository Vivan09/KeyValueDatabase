use crate::{engine::Engine, errors::DbEngineError, types::Value};

pub struct ReadOnlyEngine<E: Engine> {
    inner_engine: E,
}

impl <E: Engine> ReadOnlyEngine<E>{
    pub fn new(inner: E) -> Self {
        ReadOnlyEngine {
            inner_engine: inner,
        }
    }
}

impl <E: Engine> Engine for ReadOnlyEngine<E> {
    fn get(&self, key: &str) -> Option<&Value> {
        self.inner_engine.get(key)
    }
    fn exist(&self, key: &str) -> bool {
        self.inner_engine.exist(key)
    }
    fn put(&mut self, _key: String, _value: Value) -> Result<Option<Value>, DbEngineError> {
        Err(DbEngineError::ReadOnly)
    }
    fn del(&mut self, _key: &str) -> Result<Option<Value>, DbEngineError> {
        Err(DbEngineError::ReadOnly)
    }
    fn count(&self) -> i32 {
        self.inner_engine.count()
    }
    fn list(&self) {
        self.inner_engine.list()
    }
    fn lpush(&mut self, _key: String, _value: String) -> Result<(), DbEngineError> {
        Err(DbEngineError::ReadOnly)
    }
    fn lpop(&mut self, _key: &str) -> Result<Option<String>, DbEngineError> {
        Err(DbEngineError::ReadOnly)
    }
    fn dbsize(&self) -> usize {
        self.inner_engine.dbsize()
    }
    fn flushdb(&mut self) -> Result<usize, DbEngineError> {
        Err(DbEngineError::ReadOnly)
    }
    fn mget(&self, keys: &[String]) -> Vec<Option<&Value>> {
        self.inner_engine.mget(keys)
    }
    fn keys(&self, pattern: &str) -> Vec<&str> {
        self.inner_engine.keys(pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_engine::MemoryEngine;

    #[test]
    fn read_only() {
        let mut mem = MemoryEngine::new();
        mem.put("a".into(), Value::Integer(1)).unwrap();
        let mut db = ReadOnlyEngine::new(mem);
        assert_eq!(db.get("a"), Some(&Value::Integer(1)));
        assert!(matches!(db.put("b".into(), Value::Integer(2)), Err(DbEngineError::ReadOnly)));
        assert!(matches!(db.flushdb(), Err(DbEngineError::ReadOnly)));
    }
}
