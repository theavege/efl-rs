mod models {
    #[derive(Debug, Default)]
    pub struct Model {
        pub cel: Option<f64>,
        pub far: Option<f64>,
    }

    impl Model {
        pub fn set_cel(&mut self, value: f64) {
            self.far = Some((value * 9.0 / 5.0) + 32.0);
            self.cel = None;
        }

        pub fn set_far(&mut self, value: f64) {
            self.cel = Some((value - 32.0) * 5.0 / 9.0);
            self.far = None;
        }
    }
}

use efltk::prelude::*;

pub enum Msg {
    Cel(f64),
    Far(f64),
}

#[derive(Default)]
pub struct Converter {
    cel: efltk::Entry,
    far: efltk::Entry,
}

impl Component for Converter {
    type Event = Msg;
    type State = models::Model;
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Cel(value) => model.set_cel(value),
            Msg::Far(value) => model.set_far(value),
        };
        true
    }
    fn update(&self, model: &Self::State) {
        if let Some(value) = model.cel {
            self.cel.update(&value.to_string());
        }
        if let Some(value) = model.far {
            self.far.update(&value.to_string());
        }
    }
    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        efltk::Box::new(prt).inside(|prt| {
            efltk::Bubble::new(prt)
                .with_part("info", "Celsius")
                .inside(|prt| {
                    self.cel = efltk::Entry::new(prt)
                        .with_value("0")
                        .with_editable(true)
                        .with_changed({
                            let sender = sender.clone();
                            move |wgt| {
                                if wgt.focus() {
                                    let value = wgt.value().parse::<f64>().unwrap_or_default();
                                    sender.send(Msg::Cel(value)).unwrap();
                                }
                            }
                        });
                });
            efltk::Bubble::new(prt)
                .with_part("info", "Fahrenheit")
                .inside(|prt| {
                    self.far = efltk::Entry::new(prt)
                        .with_size(45, 45)
                        .with_value("0")
                        .with_editable(true)
                        .with_changed({
                            let sender = sender.clone();
                            move |wgt| {
                                if wgt.focus() {
                                    let value = wgt.value().parse::<f64>().unwrap_or_default();
                                    sender.send(Msg::Far(value)).unwrap();
                                }
                            }
                        });
                });
        });
    }
}
