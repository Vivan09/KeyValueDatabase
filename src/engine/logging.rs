use super::{DbResult, Engine};
use crate::types::Value;

pub struct LoggingEngine<E: Engine> {
    inner: E,
}

impl<E: Engine> LoggingEngine<E> {
    pub fn new(inner: E) -> Self {
        LoggingEngine { inner }
    }

    pub fn into_inner(self) -> E {
        self.inner
    }
}

impl<E: Engine> Engine for LoggingEngine<E> {
    fn get(&self, key: &str) -> Option<&Value> {
        eprintln!("[LOG] GET key = {key}");
        self.inner.get(key)
    }

    fn exist(&self, key: &str) -> bool {
        eprintln!("[LOG] EXIST key = {key}");
        self.inner.exist(key)
    }

    fn put(&mut self, key: String, value: Value) -> DbResult<Option<Value>> {
        eprintln!("[LOG] PUT key = {key}, value = {value}");
        self.inner.put(key, value)
    }

    fn del(&mut self, key: &str) -> DbResult<Option<Value>> {
        eprintln!("[LOG] DEL key = {key}");
        self.inner.del(key)
    }

    fn lpush(&mut self, key: String, value: String) -> DbResult<usize> {
        eprintln!("[LOG] LPUSH key = {key}, value = {value}");
        self.inner.lpush(key, value)
    }

    fn lpop(&mut self, key: &str) -> DbResult<Option<String>> {
        eprintln!("[LOG] LPOP key = {key}");
        self.inner.lpop(key)
    }

    fn dbsize(&self) -> usize {
        eprintln!("[LOG] DBSIZE");
        self.inner.dbsize()
    }

    fn flushdb(&mut self) -> DbResult<usize> {
        eprintln!("[LOG] FLUSHDB");
        self.inner.flushdb()
    }

    fn entries(&self) -> Vec<(&str, &Value)> {
        eprintln!("[LOG] LIST");
        self.inner.entries()
    }

    fn keys(&self, pattern: &str) -> Vec<&str> {
        eprintln!("[LOG] KEYS pattern = {pattern}");
        self.inner.keys(pattern)
    }

    fn mget(&self, keys: &[String]) -> Vec<Option<&Value>> {
        eprintln!("[LOG] MGET keys count = {}", keys.len());
        self.inner.mget(keys)
    }
}
