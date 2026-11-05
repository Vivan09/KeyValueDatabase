use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use crate::{command::Command, engine::Engine, errors::DbEngineError, types::Value};

pub struct FileEngine<E: Engine> {
    inner_engine: E,
    log_file: File,
}

impl <E: Engine> FileEngine<E>{
    pub fn open<P: AsRef<Path>>(path: P, mut inner: E) -> Result<Self, DbEngineError> {
        let log_file = OpenOptions::new().read(true).append(true).create(true).open(path)?;

        let reader = BufReader::new(&log_file);
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            match Command::parse(&line) {
                Some(Command::PUT(key, value)) => { inner.put(key, value)?; }
                Some(Command::DEL(key)) => { inner.del(&key)?; }
                Some(Command::LPUSH(key, value)) => { inner.lpush(key, value)?; }
                Some(Command::LPOP(key)) => { inner.lpop(&key)?; }
                Some(Command::FLUSHDB) => { inner.flushdb()?; }
                _ => return Err(DbEngineError::Parse(line)),
            }
        }

        Ok(FileEngine {
            inner_engine: inner,
            log_file,
        })
    }

    fn append(&mut self, line: &str) -> Result<(), DbEngineError> {
        writeln!(self.log_file, "{line}")?;
        self.log_file.flush()?;
        Ok(())
    }
}

impl <E: Engine> Engine for FileEngine<E> {
    fn get(&self, key: &str) -> Option<&Value> {
        self.inner_engine.get(key)
    }
    fn exist(&self, key: &str) -> bool {
        self.inner_engine.exist(key)
    }
    fn put(&mut self, key: String, value: Value) -> Result<Option<Value>, DbEngineError> {
        let line = format!("PUT {key} {value}");
        let old = self.inner_engine.put(key, value)?;
        self.append(&line)?;
        Ok(old)
    }
    fn del(&mut self, key: &str) -> Result<Option<Value>, DbEngineError> {
        let old = self.inner_engine.del(key)?;
        if old.is_some() {
            self.append(&format!("DEL {key}"))?;
        }
        Ok(old)
    }
    fn count(&self) -> i32 {
        self.inner_engine.count()
    }
    fn list(&self) {
        self.inner_engine.list()
    }
    fn lpush(&mut self, key: String, value: String) -> Result<(), DbEngineError> {
        let line = format!("LPUSH {key} {value}");
        self.inner_engine.lpush(key, value)?;
        self.append(&line)
    }
    fn lpop(&mut self, key: &str) -> Result<Option<String>, DbEngineError> {
        let value = self.inner_engine.lpop(key)?;
        if value.is_some() {
            self.append(&format!("LPOP {key}"))?;
        }
        Ok(value)
    }
    fn dbsize(&self) -> usize {
        self.inner_engine.dbsize()
    }
    fn flushdb(&mut self) -> Result<usize, DbEngineError> {
        let size = self.inner_engine.flushdb()?;
        self.log_file.set_len(0)?;
        Ok(size)
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
    use crate::engine::memory_engine::MemoryEngine;

    #[test]
    fn file_engine_restores_data() {
        let path = std::env::temp_dir().join(format!("kvdb_test_{}.aof", std::process::id()));
        let _ = std::fs::remove_file(&path);
        {
            let mut db = FileEngine::open(&path, MemoryEngine::new()).unwrap();
            db.put("name".into(), Value::String("John Smith".into())).unwrap();
            db.put("gone".into(), Value::Integer(1)).unwrap();
            db.del("gone").unwrap();
            db.lpush("l".into(), "a".into()).unwrap();
            db.lpush("l".into(), "b".into()).unwrap();
        }
        let db = FileEngine::open(&path, MemoryEngine::new()).unwrap();
        assert_eq!(db.get("name"), Some(&Value::String("John Smith".into())));
        assert!(!db.exist("gone"));
        assert_eq!(db.get("l"), Some(&Value::List(["b".to_string(), "a".to_string()].into())));
        std::fs::remove_file(&path).unwrap();
    }
}
