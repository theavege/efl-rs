# [SegmentControl](https://www.enlightenment.org/develop/legacy/program_guide/widgets/radio)

This widget consists of several segment items. A segment item is similar to a discrete two state button. Any time, only one segment item can be selected. A segment item is composed of a label (text) and an icon. This widget inherits from the layout widget, so all the layout widgets API can be used on segmentcontrol objects.

![segmentcontrol](https://www.enlightenment.org/_media/widgets_segmentcontrol.png)

```rust
    efltk::SegmentControl::new(parent)
        .with_items(&["home", "close"])
        .with_size(90, 45)
        .with_callback(SegmentControlSignal::Changed, move |wgt| println!("{} is Changed", wgt.value()));
        .with_callback(SegmentControlSignal::LanguageChanged, move |wgt| println!("{} is LanguageChanged", wgt.value()));
```

# [List](https://www.enlightenment.org/develop/legacy/program_guide/widgets/list)

This widget is a very simple type of a list widget. It is not to be used to manage a lot of items. For that, genlists are a better option. The list items can contain a text and two contents (“start”, and “end”).

![List](https://www.enlightenment.org/_media/widgets_list.png)

```rust
    efltk::List::new(parent)
        .with_items(&["home", "close"])
        .with_callback(ListSignal::Selected, move |wgt| println!("{} is Selected", wgt.value()));
```

