# [Entry](https://www.enlightenment.org/develop/legacy/program_guide/widgets/entry)

![Entry](https://www.enlightenment.org/_media/widgets_entry.png)

The entry widget is a box where the user can enter text. It supports the following features:

- text wrapping
- multiline
- scrolling
- formatted markup text
- password mode
- filtering text
- read/write from a file
- theme style overrides

```rust
    efltk::Entry::new(parent)
        .with_tooltip("HOME")
        .with_callback(EntrySignal::Changed, move |wgt| println!("{} is Changed", wgt.text()));
```
