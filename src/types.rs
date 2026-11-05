use std::collections::VecDeque;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value{
    String(String),
    Integer(i64),
    List(VecDeque<String>),
}

impl fmt::Display for Value{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        match self {
            Value::String(s) => write!(f, "{s}"),
            Value::Integer(i) => write!(f, "{i}"),
            Value::List(list) => {
                let items: Vec<&str> = list.iter().map(|s| s.as_str()).collect();
                write!(f, "[{}]", items.join(", "))
            }
        }
    }
}
