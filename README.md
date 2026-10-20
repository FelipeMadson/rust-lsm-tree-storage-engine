# Rust LSM-Tree Storage Engine

[![CI](https://github.com/FelipeMadson/rust-lsm-tree-storage-engine/actions/workflows/ci.yml/badge.svg)](https://github.com/FelipeMadson/rust-lsm-tree-storage-engine/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Release](https://img.shields.io/badge/release-v1.0.0-green.svg)](https://github.com/FelipeMadson/rust-lsm-tree-storage-engine/releases)

High-performance embedded Log-Structured Merge-tree (LSM) key-value engine written in pure **Rust**. Author: [Felipe Madison](https://github.com/FelipeMadson). Features thread-safe in-memory MemTables with BTreeMap ordered index and sub-microsecond point lookups.

## Architecture

```text
┌────────────────────────────────────────────────────────────────────────┐
│                        CONCURRENT INGRESS (API)                        │
│   • Point Writes: put(key, value)                                      │
│   • Point Reads: get(key)                                              │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
┌───────────────────────────────────▼────────────────────────────────────┐
│                        MEMTABLE & CONCURRENCY                          │
│   • Thread-safe Arc<RwLock<BTreeMap<String, String>>>                  │
│   • In-memory sorted keys for O(log N) point lookup                    │
└────────────────────────────────────────────────────────────────────────┘
```

## Build & Test

```bash
cargo check
cargo test
```

## Author

Authored by **[Felipe Madison](https://github.com/FelipeMadson)**.
