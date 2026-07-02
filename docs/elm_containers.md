# [Box](https://www.enlightenment.org/develop/legacy/program_guide/containers/box)

Most of the time, you want to display widgets on the screen in a specific order. In Form Tutorial, for example, the user information is arranged vertically. This basic container is called a box. There is no theme for a box layout. It is just a linear method of arranging widgets horizontally or vertically.

![Box](https://www.enlightenment.org/_media/container_box.png)

```rust
    efltk::Entry::new(parent)
        .inside(|parent| {
            efltk::Button::new(parent).with_icon("home");
            efltk::Button::new(parent).with_icon("close");
        });
```

# [NaviFrame](https://www.enlightenment.org/develop/legacy/program_guide/containers/naviframe)

A naviframe widget consists of a stack of views. New views are pushed on top of previous ones, and only the top-most view on the stack is displayed. The previous views are not deleted. A previous view is displayed when the view on top of it is popped. Transitions can be animated on a push or a pop, depending on the theme applied to the widget.

```rust
    efltk::NaviFrame::new(parent)
        .inside(|parent| {
            efltk::Label::new(parent).with_text("Slide One");
            efltk::Label::new(parent).with_text("Slide Two");
            parent.promote();
        });
```
