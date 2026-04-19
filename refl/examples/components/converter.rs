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

use refl::prelude::*;

pub enum Msg {
    Cel(f64),
    Far(f64),
}

#[derive(Default)]
pub struct Converter {
    cel: refl::Entry,
    far: refl::Entry,
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
        if let Some(value) = model.cel
            && value.to_string() != self.cel.value()
        {
            self.cel.set_value(&value.to_string());
        }
        if let Some(value) = model.far
            && value.to_string() != self.far.value()
        {
            self.far.set_value(&value.to_string());
        }
    }
    fn view(&mut self, parent: &impl ContainerExt, sender: Sender<Self::Event>) {
        refl::Box::new(parent).insert(|page| {
            refl::Bubble::new(page)
                .with_part("info", "Celsius")
                .insert(|bub| {
                    self.cel = refl::Entry::new(bub)
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
            refl::Bubble::new(page)
                .with_part("info", "Fahrenheit")
                .insert(|bub| {
                    self.far = refl::Entry::new(bub)
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
