# [SegmentControl](https://www.enlightenment.org/develop/legacy/program_guide/widgets/segmentcontrol)

This widget consists of several segment items. A segment item is similar to a discrete two state button. Any time, only one segment item can be selected. A segment item is composed of a label (text) and an icon. This widget inherits from the layout widget, so all the layout widgets API can be used on segmentcontrol objects.

![segmentcontrol](https://www.enlightenment.org/_media/widgets_segmentcontrol.png)

```rust
    efltk::SegmentControl::new(parent)
        .with_items(&["home", "close"])
        .with_size(90, 45)
        .with_callback(move |wgt| println!("{} is Changed", wgt.value()));
```

# [List](https://www.enlightenment.org/develop/legacy/program_guide/widgets/list)

This widget is a very simple type of a list widget. It is not to be used to manage a lot of items. For that, genlists are a better option. The list items can contain a text and two contents (“start”, and “end”).

![List](https://www.enlightenment.org/_media/widgets_list.png)

```rust
    efltk::List::new(parent)
        .with_items(&["home", "close"])
        .with_callback(move |wgt| println!("{} is Selected", wgt.value()));
```

# [Radio](https://www.enlightenment.org/develop/legacy/program_guide/widgets/radio)

Radio buttons belong to a group. Only one button in the group can be selected at a time.

```rust
    efltk::Radio::from_items(parent, &["home", "close"], move |wgt| {
        println!("{} is Selected", wgt.value());
    });
```

# [Menu](https://www.enlightenment.org/develop/legacy/program_guide/widgets/menu)

A popup menu that can be opened from a button or other widget.

```rust
    efltk::Button::with_menu(
        parent,
        efltk::Menu::popup(parent)
            .with_items(&["home", "close"])
            .with_callback(move |wgt| println!("{} is Selected", wgt.value())),
    );
```

# [Hoversel](https://www.enlightenment.org/develop/legacy/program_guide/widgets/hoversel)

A button that pops a list of items.

```rust
    efltk::Hoversel::new(parent)
        .with_text("Choose")
        .with_items(&["home", "close"]);
```

# [Diskselector](https://www.enlightenment.org/develop/legacy/program_guide/widgets/diskselector)

A rotary selector. Use it like other `SelectorExt` widgets.

```rust
    efltk::Diskselector::new(parent)
        .with_items(&["Jan", "Feb", "Mar"])
        .with_callback(move |wgt| println!("{}", wgt.value()));
```

# [Toolbar](https://www.enlightenment.org/develop/legacy/program_guide/widgets/toolbar)

A horizontal bar of selectable items.

```rust
    efltk::Toolbar::new(parent).with_items(&["home", "close"]);
```

# [Genlist](https://www.enlightenment.org/develop/legacy/program_guide/widgets/genlist)

A virtualized list. This wrapper appends plain label items (`text_get` / `del`).

```rust
    efltk::Genlist::new(parent)
        .with_items(&["one", "two", "three"])
        .with_callback(move |wgt| println!("{}", wgt.value()));
```
