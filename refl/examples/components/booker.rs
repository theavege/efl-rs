mod models {
    #[derive(Default)]
    pub struct Model {}
}

use refl::prelude::*;

pub enum Msg {
    Flight(u32),
}

#[derive(Default)]
pub struct Booker {}

impl Component for Booker {
    type Event = Msg;
    type State = models::Model;
    fn update(&self, _model: &Self::State) {}
    fn handle(msg: Self::Event, _model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Flight(_value) => {}
        };
        true
    }
    fn view(&mut self, parent: &impl ContainerExt, sender: Sender<Self::Event>) {
        refl::Box::new(parent).insert(|vbox| {
            refl::Panes::new(vbox).insert(|pns| {
                refl::Label::new(pns).with_text("Flight");
                refl::FlipSelector::new(pns).with_items(&["One-way", "Return"], {
                    let sender = sender.clone();
                    move |wgt| {
                        if wgt.focus() {
                            sender.send(Msg::Flight(wgt.value())).unwrap();
                        }
                    }
                });
            });
            refl::Panes::new(vbox).insert(|hbox| {
                refl::Label::new(hbox).with_text("Departure data");
                refl::Entry::new(hbox)
                    .with_clicked(move |wgt| {
                        refl::Notify::new(&wgt).insert(|ntf| {
                            let _ = refl::Calendar::new(ntf);
                        });
                    })
                    .with_changed({
                        let _sender = sender.clone();
                        move |wgt| {
                            if wgt.focus() {
                                let _value = wgt.value().parse::<f64>().unwrap_or_default();
                            }
                        }
                    });
            });
            refl::Panes::new(vbox).insert(|hbox| {
                refl::Label::new(hbox).with_text("Return data");
                refl::Entry::new(hbox)
                    .with_clicked(move |wgt| {
                        refl::Notify::new(&wgt).insert(|ntf| {
                            let _ = &refl::Calendar::new(ntf);
                        });
                    })
                    .with_changed({
                        let _sender = sender.clone();
                        move |wgt| {
                            if wgt.focus() {
                                let _value = wgt.value().parse::<f64>().unwrap_or_default();
                            }
                        }
                    });
            });
            refl::Button::new(vbox).set_text("4");
        });
    }
}
