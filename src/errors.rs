use std::fmt;

#[derive(Debug)]
pub enum DbEngineError{
    WrongType,
    ReadOnly,
    Io(std::io::Error),
    Parse(String),
}

impl fmt::Display for DbEngineError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        match self {
            DbEngineError::WrongType => write!(f, "Wrong type operation!"),
            DbEngineError::ReadOnly => write!(f, "Read only database!"),
            DbEngineError::Io(e) => write!(f, "IO error: {e}"),
            DbEngineError::Parse(line) => write!(f, "Cannot parse line: {line}"),
        }
    }
}

impl std::error::Error for DbEngineError{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)>{
        match self {
            DbEngineError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for DbEngineError{
    fn from(e: std::io::Error) -> Self{
        DbEngineError::Io(e)
    }
}
