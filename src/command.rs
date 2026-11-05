use crate::types::Value;

#[derive(Debug, PartialEq)]
pub enum Command {
    PUT(String, Value),
    GET(String),
    MGET(Vec<String>),
    DEL(String),
    EXIST(String),
    LPUSH(String, String),
    LPOP(String),
    KEYS(String),
    DBSIZE,
    FLUSHDB,
    LIST,
    PING,
    HELP,
    EXIT,
}

pub const HELP: &str = "\
Commands:
  PUT <key> <value...>    store a value (numbers are stored as integers)
  GET <key>               read a value
  MGET <key> [key...]     read several values
  DEL <key>               delete a key            (alias: DELETE)
  EXIST <key>             check if a key exists   (aliases: EXISTS, CONTAINS)
  LPUSH <key> <value>     push to the front of a list
  LPOP <key>              pop from the front of a list
  KEYS [pattern]          list keys matching a glob (* and ?), default *
  DBSIZE                  number of keys          (alias: COUNT)
  FLUSHDB                 delete everything       (alias: DBFLUSH)
  LIST                    print all entries       (alias: PRINT)
  PING                    PONG
  HELP                    this help
  EXIT                    quit                    (alias: QUIT)";

impl Command {
    pub fn parse(input: &str) -> Result<Self, String> {
        let mut parts = input.split_whitespace();
        let command = parts
            .next()
            .ok_or_else(|| "Empty command".to_string())?
            .to_uppercase();
        let args: Vec<&str> = parts.collect();

        let wrong_args = || format!("Wrong number of arguments for '{command}'. Type HELP.");

        match (command.as_str(), args.as_slice()) {
            ("PUT", [key, rest @ ..]) if !rest.is_empty() => {
                Ok(Command::PUT(key.to_string(), Value::parse(&rest.join(" "))))
            }
            ("GET", [key]) => Ok(Command::GET(key.to_string())),
            ("MGET", keys) if !keys.is_empty() => {
                Ok(Command::MGET(keys.iter().map(|k| k.to_string()).collect()))
            }
            ("DEL" | "DELETE", [key]) => Ok(Command::DEL(key.to_string())),
            ("EXIST" | "EXISTS" | "CONTAINS", [key]) => Ok(Command::EXIST(key.to_string())),
            ("LPUSH", [key, value]) => Ok(Command::LPUSH(key.to_string(), value.to_string())),
            ("LPOP", [key]) => Ok(Command::LPOP(key.to_string())),
            ("KEYS", [pattern]) => Ok(Command::KEYS(pattern.to_string())),
            ("KEYS", []) => Ok(Command::KEYS("*".to_string())),
            ("DBSIZE" | "COUNT", []) => Ok(Command::DBSIZE),
            ("FLUSHDB" | "DBFLUSH", []) => Ok(Command::FLUSHDB),
            ("LIST" | "PRINT", []) => Ok(Command::LIST),
            ("PING", []) => Ok(Command::PING),
            ("HELP", _) => Ok(Command::HELP),
            ("EXIT" | "QUIT", _) => Ok(Command::EXIT),
            (
                "PUT" | "GET" | "MGET" | "DEL" | "DELETE" | "EXIST" | "EXISTS" | "CONTAINS"
                | "LPUSH" | "LPOP" | "KEYS" | "DBSIZE" | "COUNT" | "FLUSHDB" | "DBFLUSH"
                | "LIST" | "PRINT" | "PING",
                _,
            ) => Err(wrong_args()),
            _ => Err(format!("Unknown command '{command}'. Type HELP.")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic() {
        assert_eq!(
            Command::parse("put a 5"),
            Ok(Command::PUT("a".into(), Value::Integer(5)))
        );
        assert_eq!(
            Command::parse("PUT a hello   world"),
            Ok(Command::PUT("a".into(), Value::String("hello world".into())))
        );
        assert_eq!(Command::parse("get a"), Ok(Command::GET("a".into())));
        assert_eq!(
            Command::parse("MGET a b"),
            Ok(Command::MGET(vec!["a".into(), "b".into()]))
        );
        assert_eq!(Command::parse("keys"), Ok(Command::KEYS("*".into())));
        assert_eq!(Command::parse("count"), Ok(Command::DBSIZE));
        assert_eq!(Command::parse("contains x"), Ok(Command::EXIST("x".into())));
        assert_eq!(Command::parse("ping"), Ok(Command::PING));
    }

    #[test]
    fn parse_errors() {
        assert!(Command::parse("").is_err());
        assert!(Command::parse("GET").is_err());
        assert!(Command::parse("GET a b").is_err());
        assert!(Command::parse("PUT a").is_err());
        assert!(Command::parse("MGET").is_err());
        assert!(Command::parse("FOO").is_err());
    }
}
