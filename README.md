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

## Notes — Fearless Concurrency

Notes distilled from the
[Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
chapter of *The Rust Programming Language*.

Rust's big idea here: by leaning on the **ownership and type system**, many
concurrency errors become **compile-time errors instead of runtime bugs**. You
get to fix mistakes while coding rather than debugging hard-to-reproduce crashes
in production — hence "fearless."

> A note on terminology: *concurrent* means different parts of a program execute
> independently; *parallel* means they execute at the same time. The book often
> says "concurrent" to mean "concurrent and/or parallel."

### 1. Using threads to run code simultaneously

- An executing program's code runs in a **process**, and within it you can have
  multiple **threads** running independently at the same time.
- Splitting work across threads can improve performance, but adds complexity:
  **race conditions**, **deadlocks**, and bugs that only appear in certain
  thread orderings.
- Rust's standard library uses a **1:1** model — one OS thread per spawned
  language thread.
- Create a thread with `thread::spawn`, passing a closure with the code to run:

  ```rust
  use std::thread;
  use std::time::Duration;

  let handle = thread::spawn(|| {
      for i in 1..10 {
          println!("hi number {i} from the spawned thread!");
          thread::sleep(Duration::from_millis(1));
      }
  });
  ```

- When the **main thread finishes, all spawned threads are shut down**, whether
  or not they have completed.
- `thread::sleep` forces a thread to pause, letting another thread make progress
  — but ordering is **not guaranteed**.

#### Waiting with `join` handles

- `thread::spawn` returns a `JoinHandle<T>`. Calling `.join()` on it **blocks**
  the current thread until the spawned thread finishes.

  ```rust
  handle.join().unwrap();
  ```

- *Where* you call `join` matters: calling it before a loop in main makes the
  threads run sequentially; calling it after lets them run concurrently.

#### `move` closures with threads

- Use the `move` keyword to **force a closure to take ownership** of the values
  it uses from the environment:

  ```rust
  let v = vec![1, 2, 3];
  let handle = thread::spawn(move || {
      println!("Here's a vector: {v:?}");
  });
  ```

- This is required because Rust can't know how long the thread will run, so it
  can't guarantee a borrowed reference stays valid. `move` transfers ownership
  into the thread, so the data lives as long as the thread needs it.

### 2. Message passing to transfer data between threads

> *"Do not communicate by sharing memory; instead, share memory by
> communicating."* — a popular Go slogan the book quotes.

- A **channel** has two halves: a **transmitter** (`tx`) and a **receiver**
  (`rx`). The channel is *closed* when either half is dropped.
- Create one with `mpsc::channel()` — **mpsc = multiple producer, single
  consumer**.

  ```rust
  use std::sync::mpsc;
  use std::thread;

  let (tx, rx) = mpsc::channel();
  thread::spawn(move || {
      let val = String::from("hi");
      tx.send(val).unwrap();
  });
  let received = rx.recv().unwrap();
  println!("Got: {received}");
  ```

- `tx.send(val)` returns `Result` — it errors if the receiver has been dropped.
- Receiving:
  - `rx.recv()` **blocks** the thread and returns `Result<T, _>` when a value
    arrives (or `Err` once the channel closes).
  - `rx.try_recv()` **doesn't block** — returns immediately with `Ok` if a value
    is ready, otherwise `Err`. Useful for doing other work while polling.

#### Channels and ownership

- `send` **takes ownership** of the value and moves it to the receiver. This
  prevents using a value after you've sent it — a use-after-send bug becomes a
  compile error.

#### Multiple values and multiple producers

- Treating `rx` as an **iterator** receives values until the channel closes:

  ```rust
  for received in rx {
      println!("Got: {received}");
  }
  ```

- **Clone the transmitter** (`tx.clone()`) to get multiple producers sending into
  the same receiver — the "multiple producer" part of mpsc.

### 3. Shared-state concurrency

- Channels are like single ownership (a value is moved away). **Shared-state**
  concurrency is like multiple ownership: several threads access the same memory.

#### Mutexes

- A **mutex** ("mutual exclusion") lets only **one thread access data at a time**.
  A thread must **acquire the lock** before accessing, and **release** it when done.
- Two rules to remember: (1) you must acquire the lock before using the data;
  (2) you must unlock when done so others can acquire it.

  ```rust
  use std::sync::Mutex;

  let m = Mutex::new(5);
  {
      let mut num = m.lock().unwrap();
      *num = 6;
  } // lock released automatically here
  println!("m = {m:?}");
  ```

- `lock()` blocks until the lock is available and returns a `Result` (it fails if
  a thread holding the lock panicked — a *poisoned* mutex).
- The lock returns a `MutexGuard` smart pointer. It implements `Deref` to reach
  the inner data and `Drop` to **release the lock automatically** when the guard
  goes out of scope — so you can't forget to unlock.
- `Mutex<T>` provides **interior mutability**, like `RefCell<T>` — it lets you
  mutate the contents even through a shared reference.

#### Sharing a `Mutex` across threads with `Arc<T>`

- You can't move a single `Mutex` into multiple threads, and `Rc<T>` is **not
  thread-safe** (its reference count isn't atomic — the compiler rejects it).
- Use **`Arc<T>`** ("atomically reference counted") — same API as `Rc<T>` but safe
  to share across threads. There's a small performance cost, so use it only when
  needed.

  ```rust
  use std::sync::{Arc, Mutex};
  use std::thread;

  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
      let counter = Arc::clone(&counter);
      let handle = thread::spawn(move || {
          let mut num = counter.lock().unwrap();
          *num += 1;
      });
      handles.push(handle);
  }
  for handle in handles {
      handle.join().unwrap();
  }
  println!("Result: {}", *counter.lock().unwrap());
  ```

- The combo `Arc<Mutex<T>>` is the idiomatic way to share mutable state across
  threads: `Arc` for shared ownership, `Mutex` for safe mutation.
- Mutexes still carry the risk of **deadlocks** (e.g. two threads each waiting for
  a lock the other holds). Rust doesn't prevent these for you.

### 4. `Send` and `Sync`: extensible concurrency

Most concurrency guarantees come from two marker traits in `std::marker`:

- **`Send`** — a type that is safe to **transfer ownership between threads**.
  Almost all Rust types are `Send` (notable exception: `Rc<T>`).
- **`Sync`** — a type that is safe to be **referenced from multiple threads**
  (`T` is `Sync` if `&T` is `Send`).
- Types composed entirely of `Send`/`Sync` types are **automatically** `Send`/
  `Sync` — you rarely implement them manually.
- Implementing them by hand is **unsafe**, because you're taking responsibility
  for upholding the concurrency guarantees the compiler normally checks.

### Key takeaways

- Rust pushes a large class of concurrency bugs to **compile time** via ownership,
  the borrow checker, and the `Send`/`Sync` traits.
- Prefer **message passing** (`mpsc`) to transfer data; reach for
  **`Arc<Mutex<T>>`** when threads genuinely need shared mutable state.
- The compiler enforces a lot, but **logic errors like deadlocks are still on
  you** — the guarantee is memory safety, not freedom from all concurrency bugs.
