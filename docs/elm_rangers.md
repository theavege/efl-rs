# [Slider](https://www.enlightenment.org/develop/legacy/program_guide/widgets/slider)

The slider widget is a draggable bar that is used to select a value within a certain range.

![Slider](https://www.enlightenment.org/_media/widgets_slider.png)

```rust
    efltk::Slider::new(parent)
        .with_tooltip("Slider")
        .with_format("%1.2f")
        .with_callback(move |wgt| println!("{} is Changed", wgt.value()));
```

# [Spinner](https://www.enlightenment.org/develop/legacy/program_guide/widgets/spinner)

The spinner widget increases or decreases a numeric value with the arrow buttons. This widget inherits from the layout widget, so all functions concerning the layout widget are used on the spinner widget.

![Spinner](https://www.enlightenment.org/_media/widgets_spinner.png)

```rust
    efltk::Slider::new(parent)
        .with_tooltip("Slider")
        .with_format("%1.2f")
        .with_callback(move |wgt| println!("{} is Changed", wgt.value()));
```
