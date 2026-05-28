use refl::prelude::*;

pub enum Msg {
    None,
}

#[derive(Default)]
pub struct ScrollerDemo {
    label_count: i32,
}

impl Component for ScrollerDemo {
    type Event = Msg;
    type State = ();

    fn handle(msg: Self::Event, _model: &mut Self::State, _sender: Sender<Self::Event>) -> bool {
        match msg {
            Msg::None => false,
        }
    }

    fn update(&self, _model: &Self::State) {}

    fn view(&mut self, prt: &impl ContainerExt, _sender: Sender<Self::Event>) {
        refl::Box::new(prt).inside(|prt| {
            // Create a label for the scroller
            refl::Label::new(prt)
                .with_text("Scrollable Content Area")
                .show();

            // Create a scroller with auto policy
            let scroller = refl::Scroller::new(prt)
                .with_policy(refl::ScrollerPolicy::Auto, refl::ScrollerPolicy::Auto)
                .with_bounce(true, true)
                .with_bar_mode(true, true);

            // Create a box inside the scroller with multiple labels
            refl::Box::new(&scroller)
                .with_horizontal(false)
                .inside(|prt| {
                    for i in 0..20 {
                        refl::Label::new(prt)
                            .with_text(&format!("Item {}: This is a scrollable label", i))
                            .with_size(300, 40)
                            .show();
                    }
                });

            scroller.show();

            // Additional info label
            refl::Label::new(prt)
                .with_text("Use scrollbars or mouse wheel to navigate")
                .show();
        });
    }
}
