use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum DbEngineError {
    WrongType,
    ReadOnly,
    Io(std::io::Error),
    Parse(String),
}

impl Display for DbEngineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DbEngineError::WrongType => {
                write!(f, "WRONGTYPE Operation against a key holding the wrong kind of value")
            }
            DbEngineError::ReadOnly => write!(f, "READONLY You can't write against a read only database"),
            DbEngineError::Io(err) => write!(f, "IO error: {err}"),
            DbEngineError::Parse(msg) => write!(f, "PARSE error: {msg}"),
        }
    }
}

impl std::error::Error for DbEngineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DbEngineError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for DbEngineError {
    fn from(err: std::io::Error) -> Self {
        DbEngineError::Io(err)
    }
}
