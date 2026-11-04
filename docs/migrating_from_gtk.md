# Migrating from GTK

`efltk` maps familiar GTK-style interfaces to Elementary widgets, but the
libraries have different ownership and event-loop models. Use the README
[transition table](../README.md#transition-table) to find approximate widget
counterparts; it is a mapping aid, not a promise of identical behavior.

| GTK concept | efltk equivalent |
| --- | --- |
| `GtkWindow` | `Window` with `WindowExt` |
| `GtkBox`, `GtkGrid` | `Box`, `Grid`, or `Table` with container traits |
| `GtkLabel` | `Label` with `TextExt` |
| `GtkButton` | `Button` with `ButtonExt` and `with_callback` |
| `GtkEntry` | `Entry` with `InputExt<String>` |
| `GtkScale`, `GtkSpinButton` | `Slider` or `Spinner` with `RangerExt` |
| `GtkComboBox`, `GtkListBox` | `Hoversel`, `List`, `Menu`, or `SegmentControl` |
| `GtkScrolledWindow` | `Scroller` |
| `GtkNotebook` | `Naviframe` or `Flip`, depending on interaction |

## Event loops

Use `run` to initialize EFL, show a window, and run the main loop. Widget
callbacks execute on the EFL thread. Unlike designs that share widgets through
reference-counted handles, `efltk` wrappers are `!Send` and `!Sync`; marshal
background work with `spawn_main`.

## Ownership

EFL owns native widgets. A container owns its children, and deleting the parent
invalidates child handles. The Rust wrappers do not encode that relationship in
lifetimes. Review the [memory guide](memory.md) before retaining widget handles
across deletion or asynchronous callbacks.

## API style

Most APIs use extension traits imported from `efltk::prelude::*`. Builders such
as `with_text` configure a widget and return it; `set_*` methods mutate it after
creation. Use `try_*` methods for operations that can fail because of empty
handles or invalid strings. `EflResult` and `EflError` carry those failures.
