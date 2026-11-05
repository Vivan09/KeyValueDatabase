use std::fmt;

#[derive(Debug)]
pub enum DbEngineError{
    Io(std::io::Error),
    Parse(String),
    WrongType,
    ReadOnly,
}

impl fmt::Display for DbEngineError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        match self {
            DbEngineError::Io(error) => write!(f, "I/O error: {error}"),
            DbEngineError::Parse(msg) => write!(f, "Cannot parse line: {msg}"),
            DbEngineError::WrongType => write!(f, "Wrong type operation!"),
            DbEngineError::ReadOnly => write!(f, "Read only database!"),
        }
    }
}

impl std::error::Error for DbEngineError{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)>{
        match self {
            DbEngineError::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<std::io::Error> for DbEngineError{
    fn from(value: std::io::Error) -> Self{
        DbEngineError::Io(value)
    }
}
