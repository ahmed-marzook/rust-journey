# Two stages: build the program in a big image that has the Rust toolchain,
# then copy ONLY the finished binary into a small image. The final image
# contains no compiler and no source code, just the program.

# ── Stage 1: build ─────────────────────────────────────────
# Same Debian release (bookworm) as the runtime stage, so the binary is
# linked against the same system C library it will run with.
FROM rust:1-bookworm AS builder

WORKDIR /app

# Copy only what the build needs (.dockerignore keeps target/, todo.db etc. out).
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# --locked: use the exact dependency versions in Cargo.lock, never newer ones.
# rusqlite's "bundled" feature compiles SQLite itself here (the rust image
# has a C compiler), so the runtime image doesn't need SQLite installed.
RUN cargo build --release --locked

# ── Stage 2: run ───────────────────────────────────────────
FROM debian:bookworm-slim

# tzdata: lets the TZ setting below work. Without it, the container runs on
# UTC, and "today", "tmr" and "overdue" would be wrong for part of the day.
# Removing the apt lists afterwards keeps the image small.
RUN apt-get update \
    && apt-get install -y --no-install-recommends tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Europe/London

# Don't run as root: create an ordinary user, and a /data folder it owns.
# A named volume mounted at /data starts with this folder's owner, so the
# user can write todo.db there.
RUN useradd --create-home --uid 10001 todo \
    && mkdir /data \
    && chown todo:todo /data

COPY --from=builder /app/target/release/rust_todo_list /usr/local/bin/rust_todo_list

USER todo

# The app opens "todo.db" in the current folder, so making /data the current
# folder puts the database on the volume, with no code changes.
WORKDIR /data
VOLUME /data

# "todo" is baked in, so `docker run <image> list` runs `rust_todo_list todo list`.
# Anything after the image name is added on the end.
ENTRYPOINT ["rust_todo_list", "todo"]
