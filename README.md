# RustDB

A lightweight key-value database built from scratch in Rust to understand how real storage engines work internally.

Instead of relying on existing databases, this project focuses on implementing the core pieces manually — persistence, indexing, recovery, and compaction.

## Features

* Persistent append-only storage engine
* B-Tree based indexing for efficient lookups
* Crash recovery through log replay
* Tombstone-based deletion
* Log compaction to reclaim disk space
* Custom binary serialization format
* Simple query parser and CLI
* Range query support

## Why I Built This

I wanted to understand what actually happens underneath databases like SQLite or RocksDB instead of only using them through APIs.

The goal of this project was not to build a production-ready database, but to learn:

* how storage engines persist data
* how indexing works internally
* why databases use B-Trees
* how crash recovery is implemented
* how append-only systems maintain durability
* how low-level systems programming feels in Rust

## Architecture

RustDB uses an append-only log file as the source of truth.

Every write operation is serialized and appended to disk. The database maintains an in-memory B-Tree that maps keys to offsets inside the log file.

When reading a key:

1. Search the B-Tree for the key
2. Retrieve the offset
3. Seek directly to that position in the log
4. Deserialize and return the value

Deletes are implemented using tombstones instead of modifying old data in-place.

## Storage Format

Each entry is stored in binary format:

### PUT Entry

[type][key_length][key][value_length][value]

### DELETE Entry

[type][key_length][key]

The log is append-only, which keeps writes simple and crash-safe.

## Recovery

On startup, the database rebuilds the B-Tree index by replaying the log from disk.

This allows the database to recover automatically after crashes without losing committed data.

## Compaction

Since updates and deletes keep old entries in the log, the file grows over time.

Compaction rewrites only the latest valid entries into a new log file and removes obsolete records.

## Example Commands

```bash
INSERT user Alice
GET user
DELETE user
```

## Tech Stack

* Rust
* std::fs for file handling
* Custom B-Tree implementation
* Binary serialization

## What I Learned

Building this project gave me hands-on experience with:

* storage engine design
* B-Tree indexing
* durability and recovery
* file I/O
* serialization
* memory management in Rust
* systems programming concepts

## Future Improvements

Planned improvements include:

* concurrent readers/writers
* disk-based B+Tree pages
* MVCC support
* background compaction
* WAL buffering
* snapshots/checkpoints
* TCP server support
* SQL-like query layer

## Running the Project

```bash
cargo build --release
cargo run --release
```

## Repository Goals

This project is mainly focused on learning database internals and systems programming by implementing core database concepts from scratch instead of relying on existing engines.
