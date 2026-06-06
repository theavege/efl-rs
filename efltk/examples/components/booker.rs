mod models {
    #[derive(Default)]
    pub struct Model {}
}

use efltk::prelude::*;

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
    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        efltk::Box::new(prt).with_homogeneous(true).inside(|prt| {
            efltk::Panes::new(prt)
                .with_fixed_size(0.6, 0.4)
                .inside(|prt| {
                    efltk::Label::new(prt).with_text("Flight");
                    efltk::SegmentControl::new(prt)
                        .with_items(&["One-way", "Return"])
                        .with_value(0)
                        .with_changed({
                            let sender = sender.clone();
                            move |wgt| {
                                if wgt.focus() {
                                    sender.send(Msg::Flight(wgt.value())).unwrap();
                                }
                            }
                        });
                });
            efltk::Panes::new(prt)
                .with_fixed_size(0.6, 0.4)
                .inside(|prt| {
                    efltk::Label::new(prt).set_text("Departure data");
                    efltk::Entry::new(prt)
                        .with_editable(false)
                        .on_clicked(|wgt| {
                            efltk::Popup::new(&wgt)
                                .with_timeout(0.0)
                                .set_message("home", "TTT", "PPPP");
                        });
                });
            efltk::Panes::new(prt)
                .with_fixed_size(0.6, 0.4)
                .inside(|prt| {
                    efltk::Label::new(prt).set_text("Return data");
                    efltk::Entry::new(prt).set_editable(false);
                });
            efltk::Button::new(prt).set_text("Book");
        });
    }
}
