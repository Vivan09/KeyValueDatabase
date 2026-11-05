use crate::{errors::DbEngineError, types::Value};

pub trait Engine{
    fn get(&self, key :&str) -> Option<&Value>;
    fn exist(&self, key :&str) -> bool;
    fn put(&mut self, key: String, value: Value) -> Result<Option<Value>, DbEngineError>;
    fn del(&mut self, key: &str) -> Result<Option<Value>, DbEngineError>;
    fn count(&self) -> i32;
    fn list(&self);
    fn lpush(&mut self, key: String, value: String) -> Result<(), DbEngineError>;
    fn lpop(&mut self, key: &str) -> Result<Option<String>, DbEngineError>;
    fn dbsize(&self) -> usize;
    fn flushdb (&mut self) -> Result<usize, DbEngineError>;
    fn mget(&self, keys: &[String]) -> Vec<Option<&Value>>;
    fn keys(&self, pattern: &str) -> Vec<&str>;
}

impl<E: Engine + ?Sized> Engine for Box<E> {
    fn get(&self, key: &str) -> Option<&Value> {
        (**self).get(key)
    }
    fn exist(&self, key: &str) -> bool {
        (**self).exist(key)
    }
    fn put(&mut self, key: String, value: Value) -> Result<Option<Value>, DbEngineError> {
        (**self).put(key, value)
    }
    fn del(&mut self, key: &str) -> Result<Option<Value>, DbEngineError> {
        (**self).del(key)
    }
    fn count(&self) -> i32 {
        (**self).count()
    }
    fn list(&self) {
        (**self).list()
    }
    fn lpush(&mut self, key: String, value: String) -> Result<(), DbEngineError> {
        (**self).lpush(key, value)
    }
    fn lpop(&mut self, key: &str) -> Result<Option<String>, DbEngineError> {
        (**self).lpop(key)
    }
    fn dbsize(&self) -> usize {
        (**self).dbsize()
    }
    fn flushdb(&mut self) -> Result<usize, DbEngineError> {
        (**self).flushdb()
    }
    fn mget(&self, keys: &[String]) -> Vec<Option<&Value>> {
        (**self).mget(keys)
    }
    fn keys(&self, pattern: &str) -> Vec<&str> {
        (**self).keys(pattern)
    }
}
