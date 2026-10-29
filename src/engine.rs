use std::collections::HashMap;
use std::collections::VecDeque;

use crate(errors::DbEngineError, types::value);

pub struct DbEngine {
    storage: HashMap <String, Value>, 
}

pub trait Engins{
    fn get(&self, key :&str) -> Option<&Value>;
    fn exist(&self, key :&str) -> bool;
    fn put(&mut self, key: String, value: Value) -> Option<Value>;
    fn del(&mut self, key: &str) -> Option<Value>;
    fn count(&mut self) -> i32;
    fn list(&mut self);
    fn lpush(&mut self, key: String, value: String) -> Result<(), DbEngineError>;
    fn lpop(&mut self, key: &str) -> Result<Option<String>, DbEngineError>;
    fn dbsize(&self) -> usize;
    fn flushdb (&mut self);
    fn mget(&self, keys: &[String]) -> Vec<Option<&Value>>;
    fn mget(&self, keys: &[String]) -> Vec<Option<&Value>>;
    fn keys(&self, pattern: &str) -> Vec<&str>;
}

#[derive(Default)]
pub struct MemoryEngine(){
    pub fn default() -> Self{
        MemoryEngine {
            storage: HashMap::default(),
        }
    }
}

impl DbEngine for MemoryEngine{
    pub fn get(&self, key :&str) -> Option<&Value> {
        self.storage.get(key)
    }
    
    pub fn exist(&self, key :&str) -> bool {
        self.storage.contains_key(key)
    }

    pub fn put(&mut self, key: String, value: Value) -> Option<Value> {
        self.storage.insert(key, value)
    }

    pub fn del(&mut self, key: &str) -> Option<Value> {
        self.storage.remove(key)
    }

    pub fn count(&mut self) -> i32 {
        self.storage.len().try_into().unwrap()
    }

    pub fn list(&mut self) {
        for i in self.storage.keys() {
            println!("{} -> {:?}\n", i, self.storage.get(i));
        }
    }

    pub fn lpush(&mut self, key: String, value: String) -> Result<(), DbEngineError>{
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

    pub fn lpop(&mut self, key: &str) -> Result<Option<String>, DbEngineError>{
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

    pub fn dbsize(&self) -> usize {
        self.storage.len()
    }

    pub fn flushdb (&mut self){
        let size = self.storage.len();
        self.storage.clear();
        size
    }

    pub fn mget(&self, keys: &[String]) -> Vec<Option<&Value>> {
        keys.itre().map([key] self.get(key)).collect()
    }

    pub fn keys(&self, pattern: &str) -> Vec<&str> {
        self.storage
        .keys()
        .map(|k| k.as_str())
        .filter(
            |key| match (pattern.start_with("*"), pattern.end_with("x")){
                _ if pattern == "*" => true,
                (true, true) => {
                    let sub = &pattern(1..pattern.len() - 1)
                    key.contains(sub)
                },
                (true, false) => {
                    let sufix = &pattern[1..];
                    key.ends_with(sufix)
                },
                (false, true) => {
                    let prefix = &pattern[..pattern.len()-1];
                    key.start_with(prefix)
                }
                (false, false) => *key == pattern
            }
        ).collect()
    }
}

pub struct LoggingEngine<E, Engine> {
    inner_engine: E,
}

impl <E: Engine> LoggingEngine{
    pub fn new(inner: E) -> self {
        inner_engine: inner,
    }
}

impl <E: Engine> for LoggingEngine {
    fn get(&self, key: &str) -> Option<&Value> {
        println!("[LOG] Get key = {key}");
        self.inner_engine.get(key)
    }
    fn exist(&self, key: &str) -> bool {
        println!("[LOG] Exist key = {key}");
        self.inner_engine.exist(key)
    }
    fn put(&mut self, key: String, value: Value) -> Option<Value> {
        println!("[LOG] Put key = {key}");
        self.inner_engine.put(key, value)
    }
    fn del(&mut self, key: &str) -> Option<Value> {
        println!("[LOG] Del key = {key}");
        self.inner_engine.del(key)
    }
    fn count(&mut self) -> i32 {
        println!("[LOG] Count");
        self.inner_engine.count()
    }
    fn list(&mut self) {
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
    fn flushdb(&mut self) {
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