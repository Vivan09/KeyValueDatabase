use std::io;

use kvdb::{
    command::Command,
    engine::{DbEngine, DbEngineError, Value}
};

fn main() {
    let mut buffer = String::new();
    let mut db_engine = DbEngine::new();
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
                db_engine.put(key, value);
                println!("OK");
            }
            Command::GET(key) => {
                match db_engine.get(&key){
                    Some(value) => println!("{value:?}"),
                    None => println!("None")
                }
            }
            Command::DEL(key) => {
                match db_engine.del(&key) {
                    Some(value) => println!("Deleted: {value:?}"),
                    None => println!("None"),
                }
            }
            Command::EXIST(key) => {
                if db_engine.exist(&key) {
                    println!("Key exist");
                }
                else{
                    println!("key not found");
                }
            }
            Command::LPUSH(key, value) => {
                match db_engine.lpush(key, value){
                    Ok(()) => println!("OK"),
                    Err(DbEngineError::WrongType) => println!("(error Wrong type operation!)"),
                }
            }
            Command::LPOP(key) => {
                match db_engine.lpop(&key){
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
                db_engine.list();
            }
            Command::COUNT => {
                println!("count = {}", db_engine.count());
            }
            Command::DBSIZE => {
                println!("Number of elements: {}", db_engine.dbsize())
            }
            Command::FLUSHDB => {
                db_engine.flushdb();
                println!("OK")
            }
            Command::MGET(keys) => {
                for key in db_engine.mget(&keys) {
                    match value{
                        Some(value) => println!("{value:?}");
                        None => println!("key mot present.")
                    }
                }
            }
        };

        println!("{t}");
    }
}
