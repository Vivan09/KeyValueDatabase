use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path: Path,
}

use crate:: {
    command::Command,
    engine::{Engine, DbEngineError, Value}
}

pub struct FileEngine<E: Engine>{
    inner_engine: E,
    log_file: File,
}

impl <E:Engine> FileEngine<E> {
    pub fn open<P: AsRef<Path>>(path: P, mut inner: E) -> Result<Self, DbEngineError> {
        let log_file = OpenOptions::new().read(true).append(true).create(true).open(path)?;

        let reader = BufReader::new(&log_file);

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }

            let parse_error = || DbEngineError::ParseError(format!("line {}: {line}", n+1))

            match Command::parse(&line).ok_or_else(parse_error) {
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
                _ => return Err(DbEngineError::Parse(line)),
            }
        }

        Ok(FileEngine {
            inner_engine,
            log_file,
        })
    }

    fn append(&mut self, line: &str) -> Result<(), DbEngineError> {
        writeln!(self.log_file, "{line}")?;
        Ok(())
    }
}

