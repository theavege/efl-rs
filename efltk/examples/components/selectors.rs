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

use efltk::prelude::*;

pub enum Msg {
    Set(u32),
}

#[derive(Default)]
pub struct Selector {
    radio: efltk::Radio,
    radio1: efltk::Radio,
    sel: efltk::HoverSel,
    seg: efltk::SegmentControl,
    list: efltk::List,
    flip: efltk::FlipSelector,
}

impl Component for Selector {
    type Event = Msg;
    type State = models::Model;
    fn update(&self, model: &Self::State) {
        self.list.update(model.value());
        self.seg.update(model.value());
        self.flip.update(model.value());
        self.radio.update(model.value() as i32);
        self.radio1.update(model.value() as i32);
        let choice = ["home", "close", "folder"][model.value() as usize];
        self.sel.set_text(choice);
        self.sel.set_icon(choice);
    }
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Set(value) => model.set_value(value),
        };
        true
    }
    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        let items = ["home", "close", "folder"];
        efltk::Box::new(prt).inside(|prt| {
            self.seg = efltk::SegmentControl::new(prt)
                .with_items(&items)
                .with_callback(SelectorSignal::Changed, {
                    let sender = sender.clone();
                    move |wgt| {
                        sender.send(Msg::Set(wgt.value())).unwrap();
                    }
                });
            efltk::Box::new(prt)
                .with_homogeneous(true)
                .with_horizontal(true)
                .inside(|prt| {
                    self.sel = efltk::HoverSel::new(prt);
                    for (idx, item) in items.iter().enumerate() {
                        self.sel.add_item(item, item, {
                            let sender = sender.clone();
                            move |_| {
                                sender.send(Msg::Set(idx as u32)).unwrap();
                            }
                        });
                    }
                    self.flip = efltk::FlipSelector::new(prt)
                        .with_items(&items)
                        .with_callback(SelectorSignal::Selected, {
                            let sender = sender.clone();
                            move |wgt| {
                                if wgt.focus() {
                                    sender.send(Msg::Set(wgt.value())).unwrap();
                                }
                            }
                        });
                });
            efltk::Box::new(prt)
                .with_homogeneous(true)
                .with_horizontal(true)
                .inside(|prt| {
                    self.list = efltk::List::new(prt).with_items(&items).with_callback(
                        SelectorSignal::Selected,
                        {
                            let sender = sender.clone();
                            move |wgt| {
                                if wgt.focus() {
                                    sender.send(Msg::Set(wgt.value())).unwrap();
                                }
                            }
                        },
                    );
                    efltk::Box::new(prt).inside(|prt| {
                        self.radio = efltk::Radio::new(prt, &items, {
                            let sender = sender.clone();
                            move |wgt| {
                                if wgt.focus() {
                                    sender.send(Msg::Set(wgt.value() as u32)).unwrap();
                                }
                            }
                        })
                    });
                });
            efltk::Box::new(prt)
                .with_homogeneous(true)
                .with_horizontal(true)
                .inside(|prt| {
                    self.radio1 = efltk::Radio::new(prt, &items, {
                        let sender = sender.clone();
                        move |wgt| {
                            if wgt.focus() {
                                sender.send(Msg::Set(wgt.value() as u32)).unwrap();
                            }
                        }
                    })
                });
        });
    }
}
