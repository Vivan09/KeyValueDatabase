use std::io;

use kvdb::{
    command::Command,
    engine::Engine,
    memory_engine::MemoryEngine,
    log_engine::LoggingEngine,
    file_engine::FileEngine,
    read_only_engine::ReadOnlyEngine,
};

// fn run_main_loop(mut engine: impl Engine){
fn run_main_loop(engine: &mut dyn Engine){
    let mut buffer = String::new();
    loop {
        buffer.clear();

        match io::stdin().read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error {e}");
                break;
            }
        }

        let trimmed = buffer.trim();

        if trimmed.is_empty(){
            continue;
        }

        //prinln("Vstup -> '{buffer}'");

        // let command = match Command::parse(&buffer){
        let command = match Command::parse(trimmed){
            Some(cmd) => cmd,
            None => {
                println!("Invalid command");
                continue;
            }
        };

        //println!("Command => {command}");
        //println!("Args => {args:?}");

        match command {
            Command::EXIT => break,
            Command::PUT(key, value) =>{
                match engine.put(key, value){
                    Ok(_) => println!("OK"),
                    Err(e) => println!("(error {e})"),
                }
            }
            Command::GET(key) => {
                match engine.get(&key){
                    Some(value) => println!("{value:?}"),
                    None => println!("None")
                }
            }
            Command::DEL(key) => {
                match engine.del(&key) {
                    Ok(Some(value)) => println!("Deleted: {value:?}"),
                    Ok(None) => println!("None"),
                    Err(e) => println!("(error {e})"),
                }
            }
            Command::EXIST(key) => {
                if engine.exist(&key) {
                    println!("Key exist");
                }
                else{
                    println!("key not found");
                }
            }
            Command::LPUSH(key, value) => {
                match engine.lpush(key, value){
                    Ok(()) => println!("OK"),
                    Err(e) => println!("(error {e})"),
                }
            }
            Command::LPOP(key) => {
                match engine.lpop(&key){
                    Ok(Some(value)) => println!("{value}"),
                    Ok(None) => println!("None"),
                    Err(e) => println!("(error {e})"),
                }
            },
            Command::Unknow(key) => {
                println!("Command not found {key}")
            }
            Command::PING => println!("PONG"),
            Command::LIST => {
                println!("LIST = ");
                engine.list();
            }
            Command::COUNT => {
                println!("count = {}", engine.count());
            }
            Command::DBSIZE => {
                println!("Number of elements: {}", engine.dbsize())
            }
            Command::FLUSHDB => {
                match engine.flushdb(){
                    Ok(_) => println!("OK"),
                    Err(e) => println!("(error {e})"),
                }
            }
            Command::MGET(keys) => {
                for value in engine.mget(&keys) {
                    match value{
                        Some(value) => println!("{value:?}"),
                        None => println!("key not present.")
                    }
                }
            }
            Command::KEYS(pattern) => {
                let matched_keys = engine.keys(&pattern);
                if matched_keys.is_empty() {
                    println!("()")
                }
                else {
                    for (i, key) in matched_keys.iter().enumerate() {
                        println!("{}) {}", i + 1, key)
                    }
                }
            }
        };
    }
}

fn main(){
    let mut use_loging = false;
    let mut read_only = false;
    let mut path = Some("kvdb.aof".to_string());

    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--log" => use_loging = true,
            "--readonly" => read_only = true,
            "--memory" => path = None,
            _ => path = Some(arg),
        }
    }

    let memmory_engine = MemoryEngine::new();

    let mut engine: Box<dyn Engine> = match path {
        Some(path) => match FileEngine::open(&path, memmory_engine) {
            Ok(file_engine) => Box::new(file_engine),
            Err(e) => {
                eprintln!("Cannot open {path}: {e}");
                return;
            }
        },
        None => Box::new(memmory_engine),
    };

    if read_only {
        engine = Box::new(ReadOnlyEngine::new(engine));
    }

    if use_loging {
        engine = Box::new(LoggingEngine::new(engine));
    }

    run_main_loop(engine.as_mut());
}
