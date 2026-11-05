use crate::types::Value;

#[derive(Debug)]
pub enum Command{
    PUT(String, Value),
    GET(String),
    DEL(String),
    EXIST(String),
    LPUSH(String, String),
    LPOP(String),
    PING,
    EXIT,
    LIST,
    COUNT,
    DBSIZE,
    FLUSHDB,
    MGET(Vec<String>),
    KEYS(String),
    Unknow(String)
}


impl Command{
    pub fn parse (input: &str) -> Option<Self>{
        let mut parts = input.split_whitespace();
        let command = parts.next()?.to_uppercase();
        let args: Vec<&str> = parts.collect();
        match (command.as_str(), args.as_slice()){
            ("EXIT",_) => Some(Command::EXIT),
            ("PUT", [key, rest @ ..]) if !rest.is_empty() => {
                let val = rest.join(" ");
                let value = match val.parse::<i64>() {
                    Ok(number) => Value::Integer(number),
                    Err(_) => Value::String(val),
                };
                Some(Command::PUT(key.to_string(), value))
            },
            ("GET", [key]) => {
                Some(Command::GET(key.to_string()))
            },
            ("DEL" | "DELETE", [key]) => {
                Some(Command::DEL(key.to_string()))
            },
            ("EXIST" | "CONTAINS", [key]) => {
                Some(Command::EXIST(key.to_string()))
            },
            ("LPUSH", [key, value]) => {
                Some(Command::LPUSH(key.to_string(), value.to_string()))
            },
            ("LPOP", [key]) => {
                Some(Command::LPOP(key.to_string()))
            },
            ("PING", []) => {
                Some(Command::PING)
            },
            ("LIST" | "PRINT", []) => {
                Some(Command::LIST)
            },
            ("COUNT", []) => {
                Some(Command::COUNT)
            },
            ("DBSIZE", []) => {
                Some(Command::DBSIZE)
            },
            ("FLUSHDB" | "DBFLUSH", []) => {
                Some(Command::FLUSHDB)
            },
            ("MGET", keys) if !keys.is_empty() => {
                let keys_vec = keys.iter().map(|k| k.to_string()).collect();
                Some(Command::MGET(keys_vec))
            },
            ("KEYS", [pattern]) => Some(Command::KEYS(pattern.to_string())),
            ("KEYS", []) => Some(Command::KEYS("*".to_string())),
            ("PUT" | "GET" | "DEL" | "DELETE" | "EXIST" | "CONTAINS" | "LPUSH" | "LPOP" | "PING"
            | "LIST" | "PRINT" | "COUNT" | "DBSIZE" | "FLUSHDB" | "DBFLUSH" | "MGET" | "KEYS", _) => None,
            _=> Some(Command::Unknow(command))
        }
    }
}
