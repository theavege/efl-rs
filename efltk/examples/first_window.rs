#![forbid(unsafe_code)]

use efltk::prelude::*;
use efltk::{Box as EflBox, Button, Label, Window};

fn main() {
    run(|| {
        let window = Window::new("first-window", "My first efltk app").with_min_size(320, 160);

        EflBox::new(&window).with_padding(12).inside(|column| {
            let message = Label::new(column).with_text("Welcome to efltk!");
            Button::new(column)
                .with_text("Click me")
                .with_callback(move |_| message.set_text("The callback updated this label."));
        });

        window
    });
}
