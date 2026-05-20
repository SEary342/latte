# Latte

Latte is a lightweight Rust CLI for tracking work logs, tasks, and time entries from the terminal.

It is designed to be:

* fast
* portable
* human-readable
* scriptable
* easy to extend

Entries are stored locally as JSON and organized with task keys, tags, projects, and optional time ranges.

---

## Features

* Create structured work log entries
* Interactive prompts for missing fields
* JSON-backed local storage
* Configurable database location
* Tabular terminal output
* Cross-platform config/data directories
* Modular architecture for future expansion

Planned features:

* Search and filtering
* Entry editing
* Weekly summaries
* Duration calculations
* Markdown/CSV export
* SQLite backend support

---

## Installation

### From Source

Requirements:

* Rust stable
* Cargo

Clone the repository:

```bash
git clone <repo-url>
cd latte
```

Build:

```bash
cargo build --release
```

Run:

```bash
cargo run -- --help
```

Or install locally:

```bash
cargo install --path .
```

---

## Usage

### Add a Log Entry

```bash
latte add JIRA-123 \
  -m "Fixed authentication middleware" \
  -t rust,backend \
  -p internal \
  -s 1300 \
  -e 1600
```

### Interactive Message Prompt

If no message is provided, Latte will prompt for one:

```bash
latte add JIRA-123
```

Example:

```text
Work description:
```

### List Entries

```bash
latte list
```

Example output:

```text
+----------+----------+-----------------------+-------------+-----------+-------------+
| id       | task     | message               | tags        | projects  | time        |
+----------+----------+-----------------------+-------------+-----------+-------------+
| 40f82c7f | JIRA-123 | Fixed auth middleware | rust,backend| internal  | 1300-1600  |
+----------+----------+-----------------------+-------------+-----------+-------------+
```

---

## Data Storage

Latte stores configuration and data in platform-appropriate directories.

### Config File

Linux:

```text
~/.config/worklog/config.json
```

macOS:

```text
~/Library/Application Support/worklog/config.json
```

### Default Database Location

Linux:

```text
~/.local/share/worklog/logs.json
```

macOS:

```text
~/Library/Application Support/worklog/logs.json
```

---

## Custom Database Location

The config file supports overriding the database location.

Example:

```json
{
  "database_path": "/Users/sam/Documents/worklogs/client-a.json"
}
```

---


### Architecture

| Module       | Responsibility                            |
| ------------ | ----------------------------------------- |
| `main.rs`    | Application bootstrap and command routing |
| `cli.rs`     | Clap CLI definitions                      |
| `commands/`  | Business logic                            |
| `storage.rs` | Persistence layer                         |
| `models.rs`  | Domain models                             |
| `ui/`        | Terminal prompts and rendering            |
| `errors.rs`  | Application error handling                |

---

## License

MIT

---

## Goals

Latte aims to stay:

* simple
* fast
* terminal-first
* hackable
* dependency-light

while remaining flexible enough to grow into a more powerful developer work logging tool.
