use std::io::{self, BufRead, Write};

use kvdb::{
    command::{Command, HELP},
    engine::{Engine, FileEngine, LoggingEngine, MemoryEngine, ReadOnlyEngine},
};

const USAGE: &str = "\
Usage: kvdb [OPTIONS] [FILE]

  FILE            append-only file used for persistence (default: kvdb.aof)
  --memory        keep data only in memory, no file
  --log           log every operation to stderr
  --readonly      open the database read-only
  -h, --help      show this help";

struct Options {
    path: Option<String>,
    log: bool,
    readonly: bool,
}

fn parse_args() -> Result<Options, String> {
    let mut opts = Options {
        path: Some("kvdb.aof".to_string()),
        log: false,
        readonly: false,
    };
    let mut path_set = false;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--memory" => opts.path = None,
            "--log" => opts.log = true,
            "--readonly" => opts.readonly = true,
            "-h" | "--help" => return Err(USAGE.to_string()),
            a if a.starts_with('-') => return Err(format!("Unknown option '{a}'\n\n{USAGE}")),
            a => {
                if path_set {
                    return Err(format!("Only one FILE may be given\n\n{USAGE}"));
                }
                opts.path = Some(a.to_string());
                path_set = true;
            }
        }
    }
    Ok(opts)
}

fn build_engine(opts: &Options) -> Result<Box<dyn Engine>, String> {
    let mut engine: Box<dyn Engine> = match &opts.path {
        Some(path) => Box::new(
            FileEngine::open(path, MemoryEngine::new())
                .map_err(|e| format!("Cannot open '{path}': {e}"))?,
        ),
        None => Box::new(MemoryEngine::new()),
    };
    if opts.readonly {
        engine = Box::new(ReadOnlyEngine::new(engine));
    }
    if opts.log {
        engine = Box::new(LoggingEngine::new(engine));
    }
    Ok(engine)
}

/// Vykona jeden prikaz. Vrati `false`, ked sa ma program ukoncit.
fn execute(engine: &mut dyn Engine, command: Command, out: &mut impl Write) -> io::Result<bool> {
    match command {
        Command::EXIT => return Ok(false),
        Command::HELP => writeln!(out, "{HELP}")?,
        Command::PING => writeln!(out, "PONG")?,
        Command::PUT(key, value) => match engine.put(key, value) {
            Ok(Some(old)) => writeln!(out, "OK (old value: {old})")?,
            Ok(None) => writeln!(out, "OK")?,
            Err(e) => writeln!(out, "(error) {e}")?,
        },
        Command::GET(key) => match engine.get(&key) {
            Some(value) => writeln!(out, "{value}")?,
            None => writeln!(out, "(nil)")?,
        },
        Command::MGET(keys) => {
            for (i, (key, value)) in keys.iter().zip(engine.mget(&keys)).enumerate() {
                match value {
                    Some(value) => writeln!(out, "{}) {key} = {value}", i + 1)?,
                    None => writeln!(out, "{}) {key} = (nil)", i + 1)?,
                }
            }
        }
        Command::DEL(key) => match engine.del(&key) {
            Ok(Some(old)) => writeln!(out, "Deleted: {old}")?,
            Ok(None) => writeln!(out, "(nil)")?,
            Err(e) => writeln!(out, "(error) {e}")?,
        },
        Command::EXIST(key) => {
            let exists = engine.exist(&key);
            writeln!(out, "{}", if exists { "Key exists" } else { "Key not found" })?
        }
        Command::LPUSH(key, value) => match engine.lpush(key, value) {
            Ok(len) => writeln!(out, "(integer) {len}")?,
            Err(e) => writeln!(out, "(error) {e}")?,
        },
        Command::LPOP(key) => match engine.lpop(&key) {
            Ok(Some(value)) => writeln!(out, "{value}")?,
            Ok(None) => writeln!(out, "(nil)")?,
            Err(e) => writeln!(out, "(error) {e}")?,
        },
        Command::KEYS(pattern) => {
            let keys = engine.keys(&pattern);
            if keys.is_empty() {
                writeln!(out, "(empty)")?;
            }
            for (i, key) in keys.iter().enumerate() {
                writeln!(out, "{}) {key}", i + 1)?;
            }
        }
        Command::DBSIZE => writeln!(out, "(integer) {}", engine.dbsize())?,
        Command::FLUSHDB => match engine.flushdb() {
            Ok(n) => writeln!(out, "OK ({n} keys removed)")?,
            Err(e) => writeln!(out, "(error) {e}")?,
        },
        Command::LIST => {
            let entries = engine.entries();
            if entries.is_empty() {
                writeln!(out, "(empty)")?;
            }
            for (key, value) in entries {
                writeln!(out, "{key} -> {value} ({})", value.type_name())?;
            }
        }
    }
    Ok(true)
}

fn run_main_loop(engine: &mut dyn Engine) -> io::Result<()> {
    let stdin = io::stdin();
    let mut out = io::stdout();
    let mut buffer = String::new();

    loop {
        write!(out, "kvdb> ")?;
        out.flush()?;

        buffer.clear();
        if stdin.lock().read_line(&mut buffer)? == 0 {
            writeln!(out)?;
            break;
        }

        let trimmed = buffer.trim();
        if trimmed.is_empty() {
            continue;
        }

        match Command::parse(trimmed) {
            Ok(command) => {
                if !execute(engine, command, &mut out)? {
                    break;
                }
            }
            Err(msg) => writeln!(out, "(error) {msg}")?,
        }
    }
    Ok(())
}

fn main() {
    let opts = match parse_args() {
        Ok(opts) => opts,
        Err(msg) => {
            eprintln!("{msg}");
            std::process::exit(2);
        }
    };

    let mut engine = match build_engine(&opts) {
        Ok(engine) => engine,
        Err(msg) => {
            eprintln!("{msg}");
            std::process::exit(1);
        }
    };

    if let Err(e) = run_main_loop(engine.as_mut()) {
        eprintln!("I/O error: {e}");
        std::process::exit(1);
    }
}
