use std::collections::HashMap;
use std::collections::VecDeque;

use crate::{engine::Engine, errors::DbEngineError, types::Value};

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
