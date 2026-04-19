mod models {
    #[derive(Default)]
    pub struct Model(u32);
    impl Model {
        pub fn value(&self) -> u32 {
            self.0
        }
        pub fn set_value(&mut self, value: u32) {
            self.0 = value;
        }
    }
}

use refl::prelude::*;

pub enum Msg {
    Set(u32),
}

#[derive(Default)]
pub struct Selector {
    segment: refl::SegmentControl,
    radio: refl::Radio,
    toolbar: refl::ToolBar,
    list: refl::List,
    flip: refl::FlipSelector,
}

impl Component for Selector {
    type Event = Msg;
    type State = models::Model;
    fn update(&self, model: &Self::State) {
        self.segment.set_value(model.value());
        self.toolbar.set_value(model.value());
        self.list.set_value(model.value());
        self.flip.set_value(model.value());
        self.radio.set_value(model.value() as i32);
    }
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Set(value) => model.set_value(value),
        };
        true
    }
    fn view(&mut self, parent: &impl ContainerExt, sender: Sender<Self::Event>) {
        refl::Box::new(parent)
            .with_homogeneous(false)
            .insert(|vbox| {
                let items = ["home", "close", "folder"];
                self.toolbar = refl::ToolBar::new(vbox).with_items(&items, {
                    let sender = sender.clone();
                    move |wgt| {
                        if wgt.focus() {
                            sender.send(Msg::Set(wgt.value())).unwrap();
                        }
                    }
                });
                self.list = refl::List::new(vbox).with_items(&items, {
                    let sender = sender.clone();
                    move |wgt| {
                        if wgt.focus() {
                            sender.send(Msg::Set(wgt.value())).unwrap();
                        }
                    }
                });
                self.flip = refl::FlipSelector::new(vbox)
                    .with_size(0, 45)
                    .with_items(&items, {
                        let sender = sender.clone();
                        move |wgt| {
                            if wgt.focus() {
                                sender.send(Msg::Set(wgt.value())).unwrap();
                            }
                        }
                    });
                self.segment = refl::SegmentControl::new(vbox)
                    .with_items(&items)
                    .with_changed({
                        let sender = sender.clone();
                        move |wgt| {
                            if wgt.focus() {
                                sender.send(Msg::Set(wgt.value())).unwrap();
                            }
                        }
                    });
                self.radio = refl::Radio::new(vbox, &items, {
                    let sender = sender.clone();
                    move |wgt| {
                        if wgt.focus() {
                            sender.send(Msg::Set(wgt.value() as u32)).unwrap();
                        }
                    }
                });
                let hs = refl::HoverSel::new(vbox);
                for (idx, item) in items.iter().enumerate() {
                    hs.add(item, item, {
                        let sender = sender.clone();
                        move |_| {
                            sender.send(Msg::Set(idx as u32)).unwrap();
                        }
                    });
                }
            });
    }
}
