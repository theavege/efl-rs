![bubble](https://www.enlightenment.org/_legacy_embed/img/widget/bubble/preview-00.png)

![frame](https://www.enlightenment.org/_legacy_embed/img/widget/frame/preview-00.png)

# Grid

Grid is a container that lays children out at explicit positions in a virtual coordinate space.

```rust
refl::Grid::new(parent)
    .with_virtual_size(100, 100)
    .inside(|grid| {
        let button = refl::Button::new(grid).with_text("Button");
        grid.set_pack(&button, 10, 10, 40, 20);
    });
```
