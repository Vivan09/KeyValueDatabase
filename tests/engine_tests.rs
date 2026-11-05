use kvdb::{
    command::Command,
    engine::Engine,
    memory_engine::MemoryEngine,
    errors::DbEngineError,
    file_engine::FileEngine,
    read_only_engine::ReadOnlyEngine,
    types::Value,
};

#[test]
fn put_get_del() {
    let mut db = MemoryEngine::new();
    assert_eq!(db.put("a".into(), Value::Integer(1)).unwrap(), None);
    assert_eq!(db.get("a"), Some(&Value::Integer(1)));
    assert!(db.exist("a"));
    assert_eq!(db.del("a").unwrap(), Some(Value::Integer(1)));
    assert!(!db.exist("a"));
}

#[test]
fn lists() {
    let mut db = MemoryEngine::new();
    db.lpush("l".into(), "x".into()).unwrap();
    db.lpush("l".into(), "y".into()).unwrap();
    assert_eq!(db.lpop("l").unwrap(), Some("y".into()));
    assert_eq!(db.lpop("l").unwrap(), Some("x".into()));
    assert!(!db.exist("l"));

    db.put("s".into(), Value::String("str".into())).unwrap();
    assert!(matches!(db.lpush("s".into(), "x".into()), Err(DbEngineError::WrongType)));
}

#[test]
fn keys_and_mget() {
    let mut db = MemoryEngine::new();
    db.put("user:1".into(), Value::Integer(1)).unwrap();
    db.put("user:2".into(), Value::Integer(2)).unwrap();
    db.put("admin".into(), Value::Integer(3)).unwrap();

    let mut keys = db.keys("user:*");
    keys.sort();
    assert_eq!(keys, vec!["user:1", "user:2"]);
    assert_eq!(db.keys("*:1"), vec!["user:1"]);
    assert_eq!(db.keys("*dmi*"), vec!["admin"]);
    assert_eq!(db.keys("admin"), vec!["admin"]);
    assert_eq!(db.keys("*").len(), 3);

    assert_eq!(db.mget(&["admin".into(), "nope".into()]), vec![Some(&Value::Integer(3)), None]);
    assert_eq!(db.count(), 3);
    assert_eq!(db.flushdb().unwrap(), 3);
    assert_eq!(db.dbsize(), 0);
}

#[test]
fn parse_commands() {
    assert!(matches!(Command::parse("put a 5"), Some(Command::PUT(_, Value::Integer(5)))));
    assert!(matches!(Command::parse("PUT a hello world"), Some(Command::PUT(_, Value::String(s))) if s == "hello world"));
    assert!(matches!(Command::parse("ping"), Some(Command::PING)));
    assert!(matches!(Command::parse("count"), Some(Command::COUNT)));
    assert!(matches!(Command::parse("list"), Some(Command::LIST)));
    assert!(matches!(Command::parse("foo"), Some(Command::Unknow(_))));
    assert!(Command::parse("GET").is_none());
    assert!(Command::parse("GET a b").is_none());
}

#[test]
fn read_only() {
    let mut mem = MemoryEngine::new();
    mem.put("a".into(), Value::Integer(1)).unwrap();
    let mut db = ReadOnlyEngine::new(mem);
    assert_eq!(db.get("a"), Some(&Value::Integer(1)));
    assert!(matches!(db.put("b".into(), Value::Integer(2)), Err(DbEngineError::ReadOnly)));
    assert!(matches!(db.flushdb(), Err(DbEngineError::ReadOnly)));
}

#[test]
fn file_engine_restores_data() {
    let path = std::env::temp_dir().join(format!("kvdb_test_{}.aof", std::process::id()));
    let _ = std::fs::remove_file(&path);
    {
        let mut db = FileEngine::open(&path, MemoryEngine::new()).unwrap();
        db.put("name".into(), Value::String("John Smith".into())).unwrap();
        db.put("gone".into(), Value::Integer(1)).unwrap();
        db.del("gone").unwrap();
        db.lpush("l".into(), "a".into()).unwrap();
        db.lpush("l".into(), "b".into()).unwrap();
    }
    let db = FileEngine::open(&path, MemoryEngine::new()).unwrap();
    assert_eq!(db.get("name"), Some(&Value::String("John Smith".into())));
    assert!(!db.exist("gone"));
    assert_eq!(db.get("l"), Some(&Value::List(["b".to_string(), "a".to_string()].into())));
    std::fs::remove_file(&path).unwrap();
}
