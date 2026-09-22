# [Label](https://www.enlightenment.org/develop/legacy/program_guide/widgets/label)

The label widget displays text with simple html-like markup.

![Label](https://www.enlightenment.org/_media/widgets_label.png)

```rust
    efltk::Label::new(parent)
        .with_text("Home")
        .with_tooltip("HOME");
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

# [Image](https://www.enlightenment.org/develop/legacy/program_guide/widgets/image)

An image widget shows a file (or EET group). `Icon` is still the standard-icon helper; use `Image` when you have a path.

```rust
    efltk::Image::new(parent)
        .with_file("photo.png")
        .with_aspect_fixed(true);
```
