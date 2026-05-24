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
    list: refl::List,
    flip: refl::FlipSelector,
    //~ combo: refl::Combobox,
}

impl Component for Selector {
    type Event = Msg;
    type State = models::Model;
    fn update(&self, model: &Self::State) {
        self.segment.set_value(model.value());
        self.list.set_value(model.value());
        self.flip.set_value(model.value());
        self.radio.set_value(model.value() as i32);
        //~ self.combo.set_value(model.value());
    }
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Set(value) => model.set_value(value),
        };
        true
    }
    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        let items = ["home", "close", "folder"];
        refl::Box::new(prt).with_homogeneous(false).inside(|prt| {
            self.segment = refl::SegmentControl::new(prt)
                .with_items(&items)
                .with_changed({
                    let sender = sender.clone();
                    move |wgt| {
                        sender.send(Msg::Set(wgt.value())).unwrap();
                    }
                });
            self.flip = refl::FlipSelector::new(prt)
                .with_size(0, 45)
                .with_items(&items, {
                    let sender = sender.clone();
                    move |wgt| {
                        if wgt.focus() {
                            sender.send(Msg::Set(wgt.value())).unwrap();
                        }
                    }
                });
            refl::Box::new(prt).with_horizontal(true).inside(|prt| {
                self.list = refl::List::new(prt).with_items(&items, {
                    let sender = sender.clone();
                    move |wgt| {
                        if wgt.focus() {
                            sender.send(Msg::Set(wgt.index())).unwrap();
                        }
                    }
                });
                refl::Box::new(prt).inside(|prt| {
                    self.radio = refl::Radio::new(prt, &items, {
                        let sender = sender.clone();
                        move |wgt| {
                            if wgt.focus() {
                                sender.send(Msg::Set(wgt.value() as u32)).unwrap();
                            }
                        }
                    });
                });
            });
        });
    }
}
