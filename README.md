# rnit

A high-performance, modular system inspection and operational command-line utility built with Rust.

> **Active Development Warning**  
> `rnit` is an **ongoing project and currently under active development**. Features, command structures, and CLI arguments are subject to change.

---

## Overview

### Purpose & Motivation

Modern system operations often require juggling fragmented single-purpose utilities—from `ls`, `find`, and standard file manipulators to process inspectors like `ps` and `top`. Many of these tools output unstructured, ad-hoc text streams that are difficult to pipe programmatically without fragile string parsing (`awk`, `cut`, `sed`).

`rnit` was conceived to address this gap as a modern, modular, and developer-centric system inspection CLI. Its primary goals are:

- **Unify System Domains**: Consolidate disparate operating system subdomains—starting with filesystem management (`fs`) and Linux runtime telemetry (`proc`)—under a single, predictable command surface.
- **Predictable Data Interchange**: Eliminate arbitrary text scraping by giving every command first-class support for both human-readable tabular layouts and machine-parsable JSON streams.
- **Safety & Low-Level Rigor**: Bridge high-level ergonomics with zero-cost abstractions—enforcing strict pre-flight validation on filesystem modifications and executing zero-allocation kernel telemetry parsing directly over procfs and libc.

---

### Key Architectural Highlights

`rnit` is architected from the ground up for low overhead, modular extensibility, and crash resilience:

- **Decoupled Domain Architecture**: Subcommands are cleanly compartmentalized into isolated modules (`fs`, `proc`), decoupled from CLI parsing, and routed through generic core dispatchers.
- **High-Performance Memory Layout**: Traversal loops utilize reusable string buffers (`path_buf`, `content_buf`), dynamic runtime page size queries (`libc::_SC_PAGESIZE`), and in-place vector operations (`.retain()`, reverse sorting closures) to eliminate unnecessary heap churn.
- **Polymorphic Dual Output**: Core domain entities implement `TableRender` for dynamic, column-aligned terminal printing alongside `serde::Serialize` for structured JSON output pipelines.
- **Defensive Operation**: Destructive file operations feature pre-flight verification (path boundary assertions, naming validation, leaf checks) and non-destructive `--dry-run` simulation previews.

---

## Current Status & Implemented Modules

### 1. Process Monitoring (`rnit proc`)

Linux `/proc` filesystem telemetry parser and continuous monitor.

- **Non-allocating Telemetry Capture**: Reads `/proc/{pid}/stat` via pre-allocated buffers, safely parsing process commands with interior spaces and parenthetical boundaries.
- **Delta-Sampled CPU Profiling**: Calculates true normalized CPU load across an exact 200ms sleep gate using `/proc/stat` total ticks and hardware core counts (`libc::_SC_NPROCESSORS_ONLN`).
- **Dynamic Memory Metrics**: Queries system page size via `libc::_SC_PAGESIZE` to compute accurate Resident Set Size (RSS) byte values.
- **Interactive Watch Mode**: Continuously refreshes terminal output at 500ms intervals using ANSI terminal clear sequences.
- **Sorting & Truncation**: Multi-field sorting by `pid`, `name`, `cpu`, `memory` (RSS), or `state` with ascending/descending direction toggles and count limits.

#### Commands & Examples

```bash
# Display active processes sorted by CPU usage
rnit proc list --sort cpu --order desc

# Top 10 memory-consuming processes
rnit proc list --sort memory --order desc --limit 10

# Continuous top-like process monitoring
rnit proc list --watch --sort cpu --order desc

# Emit raw process data as JSON
rnit proc list --format json

```

---

### 2. Filesystem Operations (`rnit fs`)

Validated filesystem interactions with safe pre-flight checks and tabular/JSON output options.

- **Inspection & Search**: Directory listings with hidden file toggles (`-a` / `--all`) and glob pattern matching via `globset`.
- **Pre-flight Safety Layer**: Explicit validation against reserved names, null bytes, parent path existence, and directory boundaries before performing I/O.
- **Safe Modifications**: Atomic moves/renames, type-safe deletions (`--dir`), and non-destructive `--dry-run` simulation previews.
- **Structured Error Handling**: Rich domain failure reporting built with `thiserror`.

#### Commands & Examples

```bash
# List directory contents formatted as an aligned table
rnit fs list

# List all files including hidden entries, sorted by size
rnit fs list -a --sort size --order desc

# Search using glob patterns
rnit fs find "*.rs" ./src

# Inspect file or directory metadata
rnit fs info Cargo.toml

# Create a new directory or file
rnit fs create new_directory --dir
rnit fs create config.json

# Preview a deletion safely without making disk changes
rnit fs delete config.json --dry-run

# Atomic move or rename with pre-flight path validation
rnit fs rename old_name.txt new_name.txt

```

---

## Global Options & Query Flags

Most list and search subcommands support unified query and format arguments:

| Flag              | Values / Default                                | Description                                          |
| ----------------- | ----------------------------------------------- | ---------------------------------------------------- |
| `--sort <FIELD>`  | `name`, `size`, `pid`, `cpu`, `memory`, `state` | Field to sort the collection by                      |
| `--order <ORDER>` | `asc` (default), `desc`                         | Sorting direction                                    |
| `--limit <N>`     | Positive integer (`usize`)                      | Truncates output to the top `N` entries              |
| `--format <FMT>`  | `table` (default), `json`                       | Toggles human-readable terminal table or JSON stdout |

---

## Building from Source

### Prerequisites

- **Rust toolchain** (1.75+ recommended, MSRV compatible with 2024 edition features)
- **Linux** (target environment for `/proc` and `libc` telemetry)

### Compilation

```bash
# Clone the repository
git clone [https://github.com/your-username/rnit.git](https://github.com/your-username/rnit.git)
cd rnit

# Build release binary
cargo build --release

# Run unit and integration tests
cargo test

```

The compiled binary will be located at `target/release/rnit`.

---
