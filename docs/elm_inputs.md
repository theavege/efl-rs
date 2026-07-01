# [Button](https://www.enlightenment.org/develop/legacy/program_guide/widgets/button)

The Elementary button widget is a simple push button. It is composed of a label icon and an icon object and has autorepeat feature.

![button](https://www.enlightenment.org/_media/widgets_button.png)

```rust
    efltk::Button::new(parent)
        .with_icon("home")
        .with_text("Home")
        .with_tooltip("HOME")
        .with_cursor(Cursor::Hand1)
        .with_size(45, 45)
        .with_disabled(true)
        .with_callback(move |wgt| println!("{} is Clicked", wgt.text()));
```

# [Check](https://www.enlightenment.org/develop/legacy/program_guide/widgets/check)

The check widget is similar to the radio widget, except that it does not work as a group. It toggles the value of a boolean between true and false. This widget inherits from the layout widget. All layout functions can be used on the check widget.

![Check](https://www.enlightenment.org/_media/widgets_check.png)

```rust
    efltk::Check::new(parent)
        .with_icon("home")
        .with_text("Home")
        .with_tooltip("HOME")
        .with_cursor(Cursor::Hand1)
        .with_size(90, 45)
        .with_callback(CheckSignal::Changed, move |wgt| println!("{} is Changed", wgt.text()));
```

# [Entry](https://www.enlightenment.org/develop/legacy/program_guide/widgets/entry)

The entry widget is a box where the user can enter text. It supports the following features:

- text wrapping
- multiline
- scrolling
- formatted markup text
- password mode
- filtering text
- read/write from a file
- theme style overrides

![Entry](https://www.enlightenment.org/_media/widgets_entry.png)

```rust
    efltk::Entry::new(parent)
        .with_tooltip("HOME")
        .with_callback(EntrySignal::Changed, move |wgt| println!("{} is Changed", wgt.text()));
```
