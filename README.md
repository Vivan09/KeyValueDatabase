# kvdb

Jednoduchá key-value databáza v Ruste.

**Autor:** Ivan Voloshyn ([@Vivan09](https://github.com/Vivan09))

## Spustenie

```bash
cargo run                  # dáta sa ukladajú do kvdb.aof
cargo run -- moj.aof       # iný súbor
cargo run -- --memory      # len v pamäti
cargo run -- --log         # logovanie operácií
cargo run -- --readonly    # len na čítanie
```

## Príkazy

| Príkaz | Popis |
|---|---|
| `PUT key value` | uloží hodnotu (číslo sa uloží ako Integer) |
| `GET key` | vráti hodnotu |
| `MGET key1 key2 ...` | vráti viac hodnôt |
| `DEL key` / `DELETE key` | zmaže kľúč |
| `EXIST key` / `CONTAINS key` | existuje kľúč? |
| `LPUSH key value` | pridá na začiatok listu |
| `LPOP key` | vyberie zo začiatku listu |
| `KEYS [pattern]` | kľúče podľa vzoru (`user:*`, `*:1`, `*abc*`) |
| `COUNT`, `DBSIZE` | počet kľúčov |
| `LIST` / `PRINT` | vypíše všetko |
| `FLUSHDB` / `DBFLUSH` | zmaže všetko |
| `PING` | `PONG` |
| `EXIT` | koniec |

## Štruktúra

- `engine.rs` – trait `Engine`
- `memory_engine.rs` – `MemoryEngine` (HashMap)
- `log_engine.rs` – `LoggingEngine`
- `read_only_engine.rs` – `ReadOnlyEngine`
- `file_engine.rs` – `FileEngine` (ukladanie do súboru)
- `command.rs` – parsovanie príkazov
