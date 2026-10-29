use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum Value{
    String(String),
    Integer(i64),
    List(VecDeque<String>),
}

#[derive(Debug)]
pub enum DbEngineError{
    WrongType,
}

pub struct DbEngine {
    storage: HashMap <String, Value>, 
}

impl DbEngine{
    pub fn new() -> Self{
        DbEngine {
            storage: HashMap::new(),
        }
    }

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

    pub fn flushdb (&mut self) -> usize {
        let size = self.storage.len();
        self.storage.clear();
        size
    }

    pub fn mget(&self, keys: &[String]) -> Vec<Option<&Value>> {
        keys.itre().map([key] self.get(key)).collect()
    }

    pub fn keys(&self, pattern: &str) -> Vec<&str> {
        // keys.itre().map([key] self.get(key)).collect()
    }
}