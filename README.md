# fearless_concurrency

Hands-on Rust examples exploring concurrency, following the "Fearless Concurrency"
chapter of [*The Rust Programming Language*](https://doc.rust-lang.org/book/ch16-00-concurrency.html).

Each example is a self-contained Cargo binary crate, grouped by topic. Together they
walk through the three pillars of safe concurrency in Rust: spawning threads, passing
messages between them, and sharing state.

## Layout

```
fearless_concurrency/
├── threads/                 # Spawning and joining OS threads
│   ├── threads/             # Basic thread::spawn + join
│   └── move_closures/       # Moving ownership into a thread with `move`
├── mpsc/                    # Message passing over channels
│   ├── message_passing/     # Single producer sending one value
│   └── mpsc/                # Multiple producers (cloned transmitters)
└── sharing_states/          # Shared-state concurrency
    ├── mutex/               # Mutex<T> basics
    └── shared_access/       # Arc<Mutex<T>> shared across threads
```

## Examples

| Topic | Crate | Concept |
| --- | --- | --- |
| Threads | `threads/threads` | Spawn a thread with `thread::spawn` and wait with `JoinHandle::join`. |
| Threads | `threads/move_closures` | Use a `move` closure to transfer ownership of data into a thread. |
| Channels | `mpsc/message_passing` | Send a single value across a channel and receive it with `recv`. |
| Channels | `mpsc/mpsc` | Multiple producers via `tx.clone()`, consumed as an iterator over `rx`. |
| Shared state | `sharing_states/mutex` | Lock a `Mutex<T>` to get safe interior mutability. |
| Shared state | `sharing_states/shared_access` | Share a counter across 10 threads with `Arc<Mutex<T>>`. |

## Running

Each example is its own crate. From the repo root, `cd` into a crate and run it:

```bash
cd threads/threads
cargo run
```

For example, to see multiple producers feeding one channel:

```bash
cd mpsc/mpsc
cargo run
```

Or to watch threads safely increment a shared counter to 10:

```bash
cd sharing_states/shared_access
cargo run
```

## Requirements

- A recent stable [Rust toolchain](https://www.rust-lang.org/tools/install) (`rustc` + `cargo`).

All examples use only the standard library (`std::thread`, `std::sync::mpsc`,
`std::sync::{Mutex, Arc}`) — no external dependencies.
