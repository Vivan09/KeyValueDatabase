use std::collections::HashMap;
use std::collections::VecDeque;

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

#[derive(Default)]
pub struct MemoryEngine{
    storage: HashMap <String, Value>,
}

impl MemoryEngine{
    pub fn new() -> Self{
        MemoryEngine {
            storage: HashMap::default(),
        }
    }
}

impl Engine for MemoryEngine{
    fn get(&self, key :&str) -> Option<&Value> {
        self.storage.get(key)
    }

    fn exist(&self, key :&str) -> bool {
        self.storage.contains_key(key)
    }

    fn put(&mut self, key: String, value: Value) -> Result<Option<Value>, DbEngineError> {
        Ok(self.storage.insert(key, value))
    }

    fn del(&mut self, key: &str) -> Result<Option<Value>, DbEngineError> {
        Ok(self.storage.remove(key))
    }

    fn count(&self) -> i32 {
        self.storage.len().try_into().unwrap_or(i32::MAX)
    }

    fn list(&self) {
        for (key, value) in self.storage.iter() {
            println!("{} -> {:?}", key, value);
        }
    }

    fn lpush(&mut self, key: String, value: String) -> Result<(), DbEngineError>{
        let entry = self.storage.entry(key).or_insert_with(|| Value::List(VecDeque::new()));
        if let Value::List(list) = entry {
            list.push_front(value);
            Ok(())
        }
        else
        {
            Err(DbEngineError::WrongType)
        }
    }

    fn lpop(&mut self, key: &str) -> Result<Option<String>, DbEngineError>{
        match self.storage.get_mut(key){
            None => Ok(None),
            Some(Value::List(list))=>{
                let value = list.pop_front();

                if list.is_empty(){
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

    fn flushdb (&mut self) -> Result<usize, DbEngineError>{
        let size = self.storage.len();
        self.storage.clear();
        Ok(size)
    }

    fn mget(&self, keys: &[String]) -> Vec<Option<&Value>> {
        keys.iter().map(|key| self.get(key)).collect()
    }

    fn keys(&self, pattern: &str) -> Vec<&str> {
        self.storage
        .keys()
        .map(|k| k.as_str())
        .filter(
            |key| match (pattern.starts_with("*"), pattern.ends_with("*")){
                _ if pattern == "*" => true,
                (true, true) => {
                    let sub = &pattern[1..pattern.len() - 1];
                    key.contains(sub)
                },
                (true, false) => {
                    let sufix = &pattern[1..];
                    key.ends_with(sufix)
                },
                (false, true) => {
                    let prefix = &pattern[..pattern.len()-1];
                    key.starts_with(prefix)
                }
                (false, false) => *key == pattern
            }
        ).collect()
    }
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
