use crate::{engine::Engine, errors::DbEngineError, types::Value};

pub struct LoggingEngine<E: Engine> {
    inner_engine: E,
}

impl <E: Engine> LoggingEngine<E>{
    pub fn new(inner: E) -> Self {
        LoggingEngine {
            inner_engine: inner,
        }
    }
}

impl <E: Engine> Engine for LoggingEngine<E> {
    fn get(&self, key: &str) -> Option<&Value> {
        println!("[LOG] Get key = {key}");
        self.inner_engine.get(key)
    }
    fn exist(&self, key: &str) -> bool {
        println!("[LOG] Exist key = {key}");
        self.inner_engine.exist(key)
    }
    fn put(&mut self, key: String, value: Value) -> Result<Option<Value>, DbEngineError> {
        println!("[LOG] Put key = {key}");
        self.inner_engine.put(key, value)
    }
    fn del(&mut self, key: &str) -> Result<Option<Value>, DbEngineError> {
        println!("[LOG] Del key = {key}");
        self.inner_engine.del(key)
    }
    fn count(&self) -> i32 {
        println!("[LOG] Count");
        self.inner_engine.count()
    }
    fn list(&self) {
        println!("[LOG] List");
        self.inner_engine.list()
    }
    fn lpush(&mut self, key: String, value: String) -> Result<(), DbEngineError> {
        println!("[LOG] Lpush key = {key}");
        self.inner_engine.lpush(key, value)
    }
    fn lpop(&mut self, key: &str) -> Result<Option<String>, DbEngineError> {
        println!("[LOG] Lpop key = {key}");
        self.inner_engine.lpop(key)
    }
    fn dbsize(&self) -> usize {
        println!("[LOG] Dbsize");
        self.inner_engine.dbsize()
    }
    fn flushdb(&mut self) -> Result<usize, DbEngineError> {
        println!("[LOG] Flushdb");
        self.inner_engine.flushdb()
    }
    fn mget(&self, keys: &[String]) -> Vec<Option<&Value>> {
        println!("[LOG] Mget keys count = {}", keys.len());
        self.inner_engine.mget(keys)
    }
    fn keys(&self, pattern: &str) -> Vec<&str> {
        println!("[LOG] Keys pattern = {pattern}");
        self.inner_engine.keys(pattern)
    }
}
