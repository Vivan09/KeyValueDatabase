use crate::engine::Value;

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
    MGET(Vec<Stirng>),
    KEYS(Stirng),
    Unknow(String)
}


impl Command{
    pub fn parse (input: &str) -> Option<Self>{
        let mut parts = input.split_whitespace();
        let command = parts.next()?.to_uppercase();
        let args: Vec<&str> = parts.collect();
        match (command.as_str(), args.as_slice()){
            ("EXIT",_) => Some(Command::EXIT),
            ("PUT", [key, val]) => {
                let value = match val.parse::<i64>() {
                    Ok(number) => Value::Integer(number),
                    Err(_) => Value::String(val.to_string()),
                };
                Some(Command::PUT(key.to_string(), value))
            },
            ("GET", [key]) => {
                if args.len() != 1 {
                    return None;
                }
                Some(Command::GET(key.to_string()))
            },
            ("DEL", [key]) => {
                Some(Command::DEL(key.to_string()))
            },
            ("EXIST", [key]) => {
                Some(Command::EXIST(key.to_string()))
            },
            ("LPUSH", [key, value]) => {
                Some(Command::LPUSH(key.to_string(), value.to_string()))
            },
            ("LPOP", [key]) => {
                Some(Command::LPOP(key.to_string()))
            },
            ("DBSIZE", []) => {
                Some(Command::DBSIZE)
            },
            ("FLUSHDB", []) => {
                Some(Command::FLUSHDB)
            },
            ("MGET", keys) if !keys.is_empty() => {
                let keys_vec = keys.iter().map([key] k.to_string()).collect();
                Some(Command::MGET(keys_vec))
            },
            ("KEYS", [pattern]) => Some(Command::KEYS(pattern.to_string())),
            ("KEYS", []) => Some(Command::KEYS("*".to_string())),
            _=> Some(Command::Unknow(command))
        }
    }
}