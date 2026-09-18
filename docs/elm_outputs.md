# [Label](https://www.enlightenment.org/develop/legacy/program_guide/widgets/label)

The label widget displays text with simple html-like markup.

![Label](https://www.enlightenment.org/_media/widgets_label.png)

```rust
    efltk::Label::new(parent).with_text("Home");
```

# [Separator](https://www.enlightenment.org/develop/legacy/program_guide/widgets/separator)

The separator widget draws a horizontal or vertical dividing line between neighbouring widgets.

```rust
    efltk::Separator::new(parent).with_horizontal(true);
```

# [ProgressBar](https://www.enlightenment.org/develop/legacy/program_guide/widgets/progressbar)

The progress bar is a widget for visually representing the progress status of a given job/task.

![ProgressBar](https://www.enlightenment.org/_media/widgets_progressbar.png)

```rust
    let bar = efltk::ProgressBar::new(parent)
        .with_format("%1.0f%%")
        .with_size(-1, 20);
    bar.set_value(0.42);
```
