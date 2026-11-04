# Memory and Lifetimes

`efltk` widget values are lightweight handles around EFL-owned objects; the Rust
wrapper does not own the native widget allocation. `del()` deletes the native
object. EFL containers own their child objects, so deleting a parent also
invalidates handles to its children. The Rust type system does not tie a child
handle's lifetime to its parent. Do not use a child after its parent is deleted.

Use `is_set()` to detect an empty wrapper, such as a `Default`-constructed
widget. An empty handle is different from a live widget that has been deleted;
callers must respect the native parent/child lifetime.

## Callback ownership

Closures installed as widget callbacks are owned by the native object and freed
when it is deleted. Timer callbacks are owned by the `Timer`; `del()` stops the
timer and releases its closure. If a timer callback returns `false`, the closure
is released when the timer stops. Keep component mounts alive while their
callbacks should continue running; dropping a `ComponentMount` stops its update
timer.

`WidgetItem` values are handles into their parent widget's item collection. The
parent owns the underlying item; removing the item or deleting the parent
invalidates the handle.

## Raw pointers

Raw-pointer constructors are `unsafe` because the caller must guarantee that a
pointer is null or points to a live object of the expected EFL type on the
current EFL thread, and that it remains valid while the wrapper is used. Prefer
the safe widget constructors for normal application code.
