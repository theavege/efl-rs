# [Slider](https://www.enlightenment.org/develop/legacy/program_guide/widgets/slider)

The slider widget is a draggable bar that is used to select a value within a certain range.

![Slider](https://www.enlightenment.org/_media/widgets_slider.png)

```rust
    efltk::Slider::new(parent)
        .with_tooltip("Slider")
        .with_format("%1.2f")
        .with_callback(move |wgt| println!("{} is Changed", wgt.value()));
```
