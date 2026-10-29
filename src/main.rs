use std::io;

use kvdb::{
    command::Command,
    engine::MemoryEngine,
    errors::DbEngineError,
    errors::LoggingEngine
};

fn run_main_loop(mut engine: impl Engine){
// fn run_main_loop(engine: &mut dyn Engine){
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
        let mut split = trimmed.split_whitespace();
        let command = match split.next(){
            Some(c) => c.trim().to_uppercase(),
            None => {
                println!("Cannont");
                continue;
            }
        };

        // let command = match Command::parse(&buffer){
        let command = match Command::parse(trimmed){
            Some(cmd) => cmd,
            None => {
                println!("Invalid command");
                continue;
            }
        };

        let args: Vec<&str> = split.collect();
        //println!("Command => {command}");
        //println!("Args => {args:?}");

        let mut t = String::new();
        match command {
            Command::EXIT => break,
            Command::PUT(key, value) =>{
                engine.put(key, value);
                println!("OK");
            }
            Command::GET(key) => {
                match engine.get(&key){
                    Some(value) => println!("{value:?}"),
                    None => println!("None")
                }
            }
            Command::DEL(key) => {
                match engine.del(&key) {
                    Some(value) => println!("Deleted: {value:?}"),
                    None => println!("None"),
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
                    Err(DbEngineError::WrongType) => println!("(error Wrong type operation!)"),
                }
            }
            Command::LPOP(key) => {
                match engine.lpop(&key){
                    Ok(Some(value)) => println!("{value}"),
                    Ok(None) => println!("None"),
                    Err(DbEngineError::WrongType) => println!("(error Wrong type operation!)"),
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
                engine.flushdb();
                println!("OK")
            }
            Command::MGET(keys) => {
                for key in engine.mget(&keys) {
                    match value{
                        Some(value) => println!("{value:?}");
                        None => println!("key mot present.")
                    }
                }
            }
            Command::KEYS(pattern) => {
                let matched_keys = engine.keys(&pattern);
                if matched_keys.is_empty() {
                    println!("()")
                }
                else {
                    for (int i, key) in matched_keys.iter.enumerate() {
                        prinln!(i)
                    }
                }
            }
        };

        println!("{t}");
    }
}

fn main(){
/* 
    let use_loging=false;
    let mut engine: Box<dyn Engine> = if use_loging {
        Box::new(LoggingEngine::new(MemoryEngine::new()))
    } else {
        Box::new(MemoryEngine::new())
    }

    run_main_loop(engine.as_mut());
*/
    
    let memmory_engine = MemoryEngine::new();
    let logging_engine = LoggingEngine::new();
    run_main_loop(engine);
}

