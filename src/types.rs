use std::collections::VecDeque;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Integer(i64),
    List(VecDeque<String>),
}

impl Value {
    /// Cislo sa ulozi ako `Integer`, vsetko ostatne ako `String`.
    pub fn parse(input: &str) -> Self {
        match input.parse::<i64>() {
            Ok(number) => Value::Integer(number),
            Err(_) => Value::String(input.to_string()),
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::String(_) => "string",
            Value::Integer(_) => "integer",
            Value::List(_) => "list",
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "{s}"),
            Value::Integer(i) => write!(f, "{i}"),
            Value::List(list) => {
                let items: Vec<&str> = list.iter().map(String::as_str).collect();
                write!(f, "[{}]", items.join(", "))
            }
        }
    }
}
