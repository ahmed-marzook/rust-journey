# rust_todo_list

A command-line todo list written in Rust, storing tasks in SQLite. Tasks can have a priority and a due date, and the list is sorted so what needs doing next is at the top.

```
$ todo list
  #  Done  Priority  Due         Title
--------------------------------------------------
  1  [ ]   Low       20-10-2026  Pay rent  (overdue)
  2  [ ]   High      28-10-2026  Go gym
  3  [ ]   -         -           Read a book
  4  [x]   Medium    25-10-2026  Call mum
```

## Quick start

Needs [Rust](https://www.rust-lang.org/tools/install) (edition 2024, so Rust 1.85 or newer).

```powershell
cargo run -- todo add Go gym -p high -d tmr
cargo run -- todo list
```

Everything after `--` is passed to the app. The database, `todo.db`, is created in the current folder on first use.

To run it in Docker instead, see [Docker](#docker).

## Commands

| Command | What it does |
|---|---|
| `todo add <title> [-p <priority>] [-d <date>]` | Add a task |
| `todo list [-f <filter>]` | Show tasks, sorted |
| `todo done <#>` | Mark a task as completed |
| `todo remove <#>` | Delete a task |
| `todo priority <priority> <#>` | Set or clear a task's priority |
| `todo due <date> <#>` | Set or clear a task's due date (also `by`, `deadline`) |
| `todo check` | Check the database is healthy |

Running `todo` with no command prints this list.

### Task numbers

`<#>` is the number shown by `todo list`, not a permanent id. Numbers follow the list's sort order, so **they can change after any edit**: run `todo list` again before the next command.

A filtered list keeps the full list's numbers, so `todo done 4` always means the task shown as 4, whichever view you were looking at:

```
$ todo list -f high
  #  Done  Priority  Due         Title
--------------------------------------------------
  2  [ ]   High      28-10-2026  Go gym
```

### Priorities

`high` (`h`), `medium` (`med`, `m`), `low` (`l`), or `none` to clear it. Not case-sensitive.

### Dates

| You type | Means |
|---|---|
| `30-11-2026` | that date (`dd-mm-yyyy`) |
| `30-11` | that date this year, or next year if it has already passed |
| `today`, `tmr` (`tomorrow`) | relative to today |
| `none` | clear the due date |

Dates in the past are rejected.

### Filters

Used with `todo list -f <filter>`:

| Filter | Shows |
|---|---|
| `pending`, `done` (`completed`), `skipped` | tasks with that status |
| `high`, `medium`, `low` | tasks with that priority |
| `due` | tasks that have a due date |
| `overdue` | pending tasks whose due date has passed |

### Sort order

1. Pending before completed
2. Tasks with a due date, earliest first (so overdue tasks come first)
3. Then by priority: high, medium, low
4. Then oldest first

### Flags

| Flag | Used with |
|---|---|
| `-p`, `--priority <priority>` | `add` |
| `-d`, `--due`, `--by`, `--deadline <date>` | `add` |
| `-f`, `--filter <filter>` | `list` |

Flags can go anywhere after the command: `todo add -p high Go gym` works too. Unknown flags, or flags used with the wrong command, are an error.

### Exit codes

`0` on success, `1` on any error (the message goes to stderr). `todo check` also exits with `1` if the database is unhealthy, so scripts can rely on it.

## Data

Tasks are stored in **`todo.db`**, an SQLite database in the current folder (in Docker, in the `/data` volume). The file isn't committed to git.

- **Created automatically.** Opening the database creates the file if needed and applies any schema changes that haven't been applied yet. Each change is a numbered *migration* in `src/db.rs`, and SQLite's `user_version` records how many have been applied.
- **Checked by the database too.** `CHECK` constraints reject invalid statuses, priorities and dates even if a bug in the app tried to write them.
- **Ids are never reused**, even after deleting the newest task (`AUTOINCREMENT`).
- **`todo check`** reports the schema version, task count, and whether SQLite's `integrity_check` found any corruption.

```
$ todo check
Database:       todo.db
Schema version: 1 (latest 1)
Integrity:      ok
Tasks:          2
All checks passed
```

To change the schema, add a new entry to the end of `MIGRATIONS` in `src/db.rs`. Never edit or reorder an entry that has already been applied to a real database.

## Docker

The image builds the app in a Rust image, then copies only the binary into a small Debian image. It runs as a non-root user and keeps the database in a `/data` volume.

Build it (the first build compiles SQLite, so it takes a few minutes):

```powershell
docker build -t rust-todo .
```

Run commands. `todo` is built into the image, so leave it out:

```powershell
docker run --rm -v todo-data:/data rust-todo add Go gym -p high
docker run --rm -v todo-data:/data rust-todo list
```

`-v todo-data:/data` attaches a named volume, so tasks survive between runs and rebuilds. `--rm` deletes the container afterwards, which is safe because the data lives in the volume.

### Use it like a normal `todo` command

PowerShell (add it to `$PROFILE` to keep it in new terminals):
```powershell
function todo { docker run --rm -v todo-data:/data rust-todo @args }
```

Bash:
```bash
alias todo='docker run --rm -v todo-data:/data rust-todo'
```

Then `todo list`, `todo done 1` and so on work as usual.

### Docker Compose

[`compose.yaml`](compose.yaml) holds the same settings (image, volume, timezone), so the commands are shorter:

```powershell
docker compose build                       # build the image (again after code changes)
docker compose run --rm todo list          # everything after "todo" goes to the app
docker compose run --rm todo add Go gym -p high
```

This is a command-line app, not a server, so there's nothing to `docker compose up`: each `run` starts a container, runs one command and exits. It uses the same `todo-data` volume as the `docker run` commands above, so both ways see the same tasks.

### Notes

- **Timezone:** the image uses `TZ=Europe/London`, which decides what "today" and "overdue" mean. Change it in the `Dockerfile` if needed.
- **Copy an existing local database into the volume:**
  ```powershell
  docker run --rm -v todo-data:/data -v ${PWD}:/src --entrypoint cp rust-todo /src/todo.db /data/todo.db
  ```
- **Delete all tasks:** `docker volume rm todo-data`.
- After changing the code, rebuild the image. The volume isn't affected.

## Project layout

```
src/
  main.rs         opens the database, runs the parsed command, prints messages
  cli.rs          turns arguments into a command (flags, numbers, usage text)
  dates.rs        parses "today", "tmr", "30-11" and "30-11-2026"
  db.rs           opens the database, migrations, health check
  task_repo.rs    all task SQL: add, read, update, remove
  todo_list.rs    sorting, numbering and filtering (no database access)
  todo_filter.rs  the list filters and how they match a task
  display.rs      prints the task table and the check report
  task.rs         the Task struct (one row of the tasks table)
  priority.rs     Priority: parsing, display, database text
  status.rs       Status: parsing, database text
Dockerfile        two-stage image build
compose.yaml      Docker Compose settings (image, volume, timezone)
```

Each command flows the same way: `cli.rs` parses the arguments, `main.rs` turns a task number into an id via `todo_list.rs`, `task_repo.rs` runs the SQL, and `main.rs` prints the result.

## Tests

```powershell
cargo test
```

Database tests use an in-memory SQLite database (`db::open(":memory:")`), so they leave no files behind and don't touch your real tasks.
