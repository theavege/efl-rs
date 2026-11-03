# Threading Model

EFL objects and Elementary widgets are bound to the thread that initializes
EFL and runs its event loop. `efltk` wrappers are `!Send` and `!Sync`, so Rust
will not allow a widget handle to be moved to another thread or shared across
threads. Create, read, and mutate widgets on the EFL main-loop thread.

## Schedule work on the UI thread

Use `spawn_main` to schedule a `Send + 'static` closure from a worker thread.
The closure runs asynchronously on the EFL thread:

```rust,no_run
use efltk::prelude::*;
use std::thread;

fn start_worker() {
    thread::spawn(|| {
        let result = fetch_data();
        spawn_main(move || update_ui(result));
    });
}

fn fetch_data() -> String { "ready".to_owned() }
fn update_ui(_value: String) {}
# fn main() {}
```

A `Timer` callback also runs on the EFL event loop. Keep the timer handle alive
for as long as it should run; dropping or deleting it stops the callback. Use
`run_with_guard` to retain resources until the event loop exits and drop them
before EFL shutdown.

Do not call `run` recursively from a callback. Use `exit` to stop the active
loop. Initialize and shut down EFL through the provided run functions rather
than mixing them with direct lifecycle calls.
