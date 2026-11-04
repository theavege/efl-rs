# Getting Started

`efltk` provides safe Rust wrappers for EFL's Elementary widgets. Building it
requires the native EFL development headers, `pkg-config`, Clang, and libclang;
the platform-specific installation commands are in [CONTRIBUTING.md](../CONTRIBUTING.md).

## Add the crate

```toml
[dependencies]
efltk = "0.0.8"
```

The crate currently targets Rust 1.85 or newer. The first release compatibility
policy and tested OS matrix are described in the [README](../README.md#support).

## Create a window

```rust,no_run
use efltk::prelude::*;
use efltk::{Box as EflBox, Button, Label, Window};

fn main() -> EflResult<()> {
    run(|| {
        let window = Window::new("hello", "Hello EFL");
        EflBox::new(&window).inside(|column| {
            let message = Label::new(column).with_text("Ready");
            Button::new(column)
                .with_text("Update")
                .with_callback(move |_| message.set_text("Updated"));
        });
        window
    })
}
```

Run the repository example with `cargo run -p efltk --example first_window`.
The example needs a graphical display. `cargo test -p efltk --lib` only runs
library tests and does not open a window.

## Handle errors

Prefer fallible `try_*` methods when input may be invalid, especially for strings
that can contain a NUL byte:

```rust
use efltk::{EflError, Label};
use efltk::prelude::*;

fn update(label: &Label, text: &str) -> EflResult<()> {
    label.try_set_text(text)
}

# fn main() {
# let label = Label::default();
let result = update(&label, "hello");
assert!(matches!(result, Err(EflError::EmptyWidget(_))));
# }
```

See the [container](elm_containers.md), [input](elm_inputs.md),
[output](elm_outputs.md), [range](elm_rangers.md), and
[selector](elm_selectors.md) guides for widget-specific APIs.
