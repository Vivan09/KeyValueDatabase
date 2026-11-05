use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use super::{DbResult, Engine};
use crate::command::Command;
use crate::errors::DbEngineError;
use crate::types::Value;

pub struct FileEngine<E: Engine> {
    inner: E,
    log_file: File,
}

impl<E: Engine> FileEngine<E> {
    pub fn open<P: AsRef<Path>>(path: P, mut inner: E) -> DbResult<Self> {
        let log_file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(path)?;

        let reader = BufReader::new(&log_file);
        for (n, line) in reader.lines().enumerate() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            let parse_error = || DbEngineError::Parse(format!("invalid line {}: {line}", n + 1));
            match Command::parse(&line).map_err(|_| parse_error())? {
                Command::PUT(key, value) => {
                    inner.put(key, value)?;
                }
                Command::DEL(key) => {
                    inner.del(&key)?;
                }
                Command::LPUSH(key, value) => {
                    inner.lpush(key, value)?;
                }
                Command::LPOP(key) => {
                    inner.lpop(&key)?;
                }
                Command::FLUSHDB => {
                    inner.flushdb()?;
                }
                _ => return Err(parse_error()),
            }
        }

        Ok(FileEngine { inner, log_file })
    }

    fn append(&mut self, line: &str) -> DbResult<()> {
        writeln!(self.log_file, "{line}")?;
        self.log_file.flush()?;
        Ok(())
    }
}

impl<E: Engine> Engine for FileEngine<E> {
    fn get(&self, key: &str) -> Option<&Value> {
        self.inner.get(key)
    }

    fn exist(&self, key: &str) -> bool {
        self.inner.exist(key)
    }

    fn put(&mut self, key: String, value: Value) -> DbResult<Option<Value>> {
        let line = format!("PUT {key} {value}");
        let old = self.inner.put(key, value)?;
        self.append(&line)?;
        Ok(old)
    }

    fn del(&mut self, key: &str) -> DbResult<Option<Value>> {
        let old = self.inner.del(key)?;
        if old.is_some() {
            self.append(&format!("DEL {key}"))?;
        }
        Ok(old)
    }

    fn lpush(&mut self, key: String, value: String) -> DbResult<usize> {
        let line = format!("LPUSH {key} {value}");
        let len = self.inner.lpush(key, value)?;
        self.append(&line)?;
        Ok(len)
    }

    fn lpop(&mut self, key: &str) -> DbResult<Option<String>> {
        let popped = self.inner.lpop(key)?;
        if popped.is_some() {
            self.append(&format!("LPOP {key}"))?;
        }
        Ok(popped)
    }

    fn dbsize(&self) -> usize {
        self.inner.dbsize()
    }

    fn flushdb(&mut self) -> DbResult<usize> {
        let removed = self.inner.flushdb()?;
        self.log_file.set_len(0)?;
        Ok(removed)
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
    use std::path::PathBuf;

    fn temp_path(name: &str) -> PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!("kvdb_test_{}_{name}.aof", std::process::id()));
        let _ = std::fs::remove_file(&p);
        p
    }

    #[test]
    fn data_survives_reopen() {
        let path = temp_path("reopen");
        {
            let mut db = FileEngine::open(&path, MemoryEngine::new()).unwrap();
            db.put("name".into(), Value::String("hello world".into())).unwrap();
            db.put("n".into(), Value::Integer(42)).unwrap();
            db.put("gone".into(), Value::Integer(1)).unwrap();
            db.del("gone").unwrap();
            db.lpush("l".into(), "a".into()).unwrap();
            db.lpush("l".into(), "b".into()).unwrap();
            db.lpush("l".into(), "c".into()).unwrap();
            db.lpop("l").unwrap();
        }
        let db = FileEngine::open(&path, MemoryEngine::new()).unwrap();
        assert_eq!(db.get("name"), Some(&Value::String("hello world".into())));
        assert_eq!(db.get("n"), Some(&Value::Integer(42)));
        assert_eq!(db.get("gone"), None);
        assert_eq!(
            db.get("l"),
            Some(&Value::List(["b".to_string(), "a".to_string()].into()))
        );
        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    fn flush_truncates_file() {
        let path = temp_path("flush");
        {
            let mut db = FileEngine::open(&path, MemoryEngine::new()).unwrap();
            db.put("a".into(), Value::Integer(1)).unwrap();
            db.flushdb().unwrap();
            db.put("b".into(), Value::Integer(2)).unwrap();
        }
        let db = FileEngine::open(&path, MemoryEngine::new()).unwrap();
        assert_eq!(db.dbsize(), 1);
        assert_eq!(db.get("b"), Some(&Value::Integer(2)));
        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    fn corrupted_file_is_error() {
        let path = temp_path("corrupt");
        std::fs::write(&path, "PUT a 1\nGARBAGE\n").unwrap();
        assert!(matches!(
            FileEngine::open(&path, MemoryEngine::new()),
            Err(DbEngineError::Parse(_))
        ));
        std::fs::remove_file(&path).unwrap();
    }
}
