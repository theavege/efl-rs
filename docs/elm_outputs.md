# [Label](https://www.enlightenment.org/develop/legacy/program_guide/widgets/label)

The label widget displays text with simple html-like markup.

![Label](https://www.enlightenment.org/_media/widgets_label.png)

```rust
    efltk::Label::new(parent).with_text("Home");
```

# [ProgressBar](https://www.enlightenment.org/develop/legacy/program_guide/widgets/progressbar)

The progress bar is a widget for visually representing the progress status of a given job/task.

![ProgressBar](https://www.enlightenment.org/_media/widgets_progressbar.png)

```rust
    efltk::ProgressBar::new(parent)
        .with_icon("home")
        .with_text("Home")
        .with_tooltip("HOME")
        .with_cursor(Cursor::Hand1)
        .with_size(90, 45)
        .with_callback(move |wgt| println!("{} is Changed", wgt.text()));
```
