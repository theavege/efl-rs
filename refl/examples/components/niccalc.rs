mod models {
    #[derive(Default)]
    pub struct Model {
        shotstr: f64,
        targstr: f64,
        targvol: f64,
        aromavol: f64,
    }

    impl Model {
        pub fn shotstr(&self) -> f64 {
            self.shotstr
        }
        pub fn limit(&self) -> f64 {
            self.targvol - self.calculate_nic()
        }
        fn calculate_nic(&self) -> f64 {
            match self.shotstr {
                0.0 => self.shotstr,
                _ => (self.targvol * self.targstr) / self.shotstr,
            }
        }
        pub fn set_shotstr(&mut self, value: f64) {
            self.shotstr = value;
        }
        pub fn set_targstr(&mut self, value: f64) {
            self.targstr = value;
        }
        pub fn set_targvol(&mut self, value: f64) {
            self.targvol = value;
        }
        pub fn set_aromavol(&mut self, value: f64) {
            self.aromavol = value;
        }
        pub fn output(&self) -> [(&str, f64); 4] {
            let shots = self.calculate_nic();
            [
                ("Nicotine Base", shots),
                ("Base", self.targvol - (shots + self.aromavol)),
                ("Flavour", self.aromavol),
                ("Total", self.targvol),
            ]
        }
    }
}

use refl::prelude::*;

pub enum Msg {
    Shotstr(f64),
    Targstr(f64),
    Targvol(f64),
    Aromavol(f64),
}

#[derive(Default)]
pub struct NicCalc {
    base: refl::ProgressBar,
    nicotine_base: refl::ProgressBar,
    flavour: refl::ProgressBar,
    list: refl::List,
}

impl Component for NicCalc {
    type Event = Msg;
    type State = models::Model;
    fn update(&self, model: &Self::State) {
        let [nb, b, f, t] = model.output();
        self.base.set_value((t.1 / 100.0 * b.1) / 100.0);
        self.flavour.set_value((t.1 / 100.0 * f.1) / 100.0);
        self.nicotine_base.set_value((t.1 / 100.0 * nb.1) / 100.0);
        self.list.clear();
        self.list.add("Ingredient: Amount(ml)", "", |_| {});
        for (x, y) in model.output() {
            self.list.add(&format!("{x}: {y}"), "", |_| {});
        }
        self.list.go();
        self.list.show();
    }
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Shotstr(value) => {
                if (0f64..1000f64).contains(&value) {
                    model.set_shotstr(value);
                }
            }
            Msg::Targstr(value) => {
                if (0f64..=model.shotstr()).contains(&value) {
                    model.set_targstr(value);
                }
            }
            Msg::Targvol(value) => {
                if (0f64..=100000f64).contains(&value) {
                    model.set_targvol(value);
                }
            }
            Msg::Aromavol(value) => {
                if (0f64..=model.limit()).contains(&value) {
                    model.set_aromavol(value);
                }
            }
        };
        true
    }
    fn view(&mut self, parent: &impl ContainerExt, sender: Sender<Self::Event>) {
        refl::Box::new(parent).with_homogeneous(false).insert(|vbox| {
            refl::Box::new(vbox).with_horizontal(true).insert(|hbox| {
                refl::Label::new(hbox).with_text("Nicotine base strength (mg/ml):");
                refl::Entry::new(hbox)
                    .with_tooltip("Nicotine base strength must be between 0.0 und 999.9mg/ml")
                    .with_changed({
                        let sender = sender.clone();
                        move |wgt| {
                            if wgt.focus() {
                                let value = wgt.value().parse::<f64>().unwrap_or_default();
                                sender.send(Msg::Shotstr(value)).unwrap();
                            }
                        }
                    });
            });
            refl::Box::new(vbox).with_horizontal(true).insert(|hbox| {
                refl::Label::new(hbox).with_text("Nicotine strength wanted (mg/ml):");
                refl::Entry::new(hbox)
                    .with_tooltip("Nicotine strength wanted must be between  0 and value of nicotine base strength")
                    .with_changed({
                        let sender = sender.clone();
                        move |wgt| {
                            if wgt.focus() {
                                let value = wgt.value().parse::<f64>().unwrap_or_default();
                                sender.send(Msg::Targstr(value)).unwrap();
                            }
                        }
                    });
            });
            refl::Box::new(vbox).with_horizontal(true).insert(|hbox| {
                refl::Label::new(hbox).with_text("Amount wanted (ml):");
                refl::Entry::new(hbox)
                    .with_tooltip("Nicotine strength wanted must be between  0 and value of nicotine base strength")
                    .with_changed({
                        let sender = sender.clone();
                        move |wgt| {
                            if wgt.focus() {
                                let value = wgt.value().parse::<f64>().unwrap_or_default();
                                sender.send(Msg::Targvol(value)).unwrap();
                            }
                        }
                    });
            });
            refl::Box::new(vbox).with_horizontal(true).insert(|hbox| {
                refl::Label::new(hbox).with_text("Flavour amount (ml):");
                refl::Entry::new(hbox)
                    .with_tooltip("The flavour amount must be between 0 and the base amount minus nicotine base amount!")
                    .with_changed({
                        let sender = sender.clone();
                        move |wgt| {
                            if wgt.focus() {
                                let value = wgt.value().parse::<f64>().unwrap_or_default();
                                sender.send(Msg::Aromavol(value)).unwrap();
                            }
                        }
                    });
            });
            refl::Separator::new(vbox).with_horizontal(true);
            refl::Box::new(vbox).with_horizontal(true).insert(|hbox| {
                refl::Label::new(hbox).with_text("Nicotin base");
                self.nicotine_base = refl::ProgressBar::new(hbox);
            });
            refl::Box::new(vbox).with_horizontal(true).insert(|hbox| {
                refl::Label::new(hbox).with_text("Base");
                self.base = refl::ProgressBar::new(hbox);
            });
            refl::Box::new(vbox).with_horizontal(true).insert(|hbox| {
                refl::Label::new(hbox).with_text("Flavour");
                self.flavour = refl::ProgressBar::new(hbox);
            });
            refl::Box::new(vbox).with_horizontal(true).insert(|hbox| {
                refl::Label::new(hbox).with_text("Total");
                refl::ProgressBar::new(hbox).set_value(1.0);
            });
            refl::Separator::new(vbox).with_horizontal(true);
            self.list = refl::List::new(vbox);
        });
    }
}
