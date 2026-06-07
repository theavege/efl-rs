# [ProgressBar](https://www.enlightenment.org/develop/legacy/program_guide/widgets/progressbar)

![ProgressBar](https://www.enlightenment.org/_media/widgets_progressbar.png)

```rust
    efltk::ProgressBar::new(parent)
        .with_icon("home")
        .with_text("Home")
        .with_tooltip("HOME")
        .with_cursor(Cursor::Hand1)
        .with_size(90, 45)
        .with_callback(ProgressBarSignal::Changed, move |wgt| println!("{} is Changed", wgt.text()));
```
