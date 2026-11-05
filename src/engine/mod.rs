mod file;
mod logging;
mod memory;
mod read_only;

pub use file::FileEngine;
pub use logging::LoggingEngine;
pub use memory::MemoryEngine;
pub use read_only::ReadOnlyEngine;

use crate::errors::DbEngineError;
use crate::types::Value;

pub type DbResult<T> = Result<T, DbEngineError>;

pub trait Engine {
    fn get(&self, key: &str) -> Option<&Value>;
    fn exist(&self, key: &str) -> bool;
    fn put(&mut self, key: String, value: Value) -> DbResult<Option<Value>>;
    fn del(&mut self, key: &str) -> DbResult<Option<Value>>;
    fn lpush(&mut self, key: String, value: String) -> DbResult<usize>;
    fn lpop(&mut self, key: &str) -> DbResult<Option<String>>;
    fn dbsize(&self) -> usize;
    fn flushdb(&mut self) -> DbResult<usize>;
    fn entries(&self) -> Vec<(&str, &Value)>;
    fn keys(&self, pattern: &str) -> Vec<&str>;

    fn mget(&self, keys: &[String]) -> Vec<Option<&Value>> {
        keys.iter().map(|key| self.get(key)).collect()
    }
}

impl<E: Engine + ?Sized> Engine for Box<E> {
    fn get(&self, key: &str) -> Option<&Value> {
        (**self).get(key)
    }
    fn exist(&self, key: &str) -> bool {
        (**self).exist(key)
    }
    fn put(&mut self, key: String, value: Value) -> DbResult<Option<Value>> {
        (**self).put(key, value)
    }
    fn del(&mut self, key: &str) -> DbResult<Option<Value>> {
        (**self).del(key)
    }
    fn lpush(&mut self, key: String, value: String) -> DbResult<usize> {
        (**self).lpush(key, value)
    }
    fn lpop(&mut self, key: &str) -> DbResult<Option<String>> {
        (**self).lpop(key)
    }
    fn dbsize(&self) -> usize {
        (**self).dbsize()
    }
    fn flushdb(&mut self) -> DbResult<usize> {
        (**self).flushdb()
    }
    fn entries(&self) -> Vec<(&str, &Value)> {
        (**self).entries()
    }
    fn keys(&self, pattern: &str) -> Vec<&str> {
        (**self).keys(pattern)
    }
    fn mget(&self, keys: &[String]) -> Vec<Option<&Value>> {
        (**self).mget(keys)
    }
}

pub fn glob_match(pattern: &str, text: &str) -> bool {
    let p: Vec<char> = pattern.chars().collect();
    let t: Vec<char> = text.chars().collect();
    let (mut pi, mut ti) = (0, 0);
    let mut star: Option<(usize, usize)> = None;

    while ti < t.len() {
        if pi < p.len() && (p[pi] == '?' || p[pi] == t[ti]) {
            pi += 1;
            ti += 1;
        } else if pi < p.len() && p[pi] == '*' {
            star = Some((pi, ti));
            pi += 1;
        } else if let Some((sp, st)) = star {
            pi = sp + 1;
            ti = st + 1;
            star = Some((sp, st + 1));
        } else {
            return false;
        }
    }
    while pi < p.len() && p[pi] == '*' {
        pi += 1;
    }
    pi == p.len()
}

#[cfg(test)]
mod tests {
    use super::glob_match;

    #[test]
    fn glob() {
        assert!(glob_match("*", "anything"));
        assert!(glob_match("*", ""));
        assert!(glob_match("user:*", "user:1"));
        assert!(!glob_match("user:*", "admin:1"));
        assert!(glob_match("*:1", "user:1"));
        assert!(glob_match("*ser*", "user:1"));
        assert!(glob_match("u?er", "user"));
        assert!(glob_match("a*b*c", "aXXbYYc"));
        assert!(!glob_match("a*b*c", "aXXbYY"));
        assert!(glob_match("exact", "exact"));
        assert!(!glob_match("exact", "exactly"));
    }
}
