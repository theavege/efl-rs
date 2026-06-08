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
        pub fn output(&self) -> [f64; 4] {
            let shots = self.calculate_nic();
            [
                shots,
                self.targvol - (shots + self.aromavol),
                self.aromavol,
                self.targvol,
            ]
        }
    }
}

use efltk::prelude::*;

pub enum Msg {
    Shotstr(f64),
    Targstr(f64),
    Targvol(f64),
    Aromavol(f64),
}

#[derive(Default)]
pub struct NicCalc {
    base: efltk::ProgressBar,
    nicotine_base: efltk::ProgressBar,
    flavour: efltk::ProgressBar,
    list: efltk::Entry,
}

impl Component for NicCalc {
    type Event = Msg;
    type State = models::Model;
    fn update(&self, model: &Self::State) {
        let [nb, b, f, t] = model.output();
        self.base.set_value((t / 100.0 * b) / 100.0);
        self.flavour.set_value((t / 100.0 * f) / 100.0);
        self.nicotine_base.set_value((t / 100.0 * nb) / 100.0);
        self.list.update(&format!(
            "Ingredient:   Amount(ml)<br>
Nicotine Base: {nb}<br>
Base:          {b}<br>
Flavour:       {f}<br>
Total:         {t}<br>"
        ));
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
    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        efltk::Box::new(prt).inside(|prt| {
            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Button::new(prt).with_size(250, -1).set_text("Nicotine base strength (mg/ml):");
                efltk::Entry::new(prt)
                    .with_tooltip("Nicotine base strength must be between 0.0 und 999.9mg/ml")
                    .with_callback({
                        let sender = sender.clone();
                        move |wgt| {
                            if wgt.focus() {
                                let value = wgt.value().parse::<f64>().unwrap_or_default();
                                sender.send(Msg::Shotstr(value)).unwrap();
                            }
                        }
                    });
            });
            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Button::new(prt).with_size(250, -1).set_text("Nicotine strength wanted (mg/ml):");
                efltk::Entry::new(prt)
                    .with_tooltip("Nicotine strength wanted must be between  0 and value of nicotine base strength")
                    .with_callback({
                        let sender = sender.clone();
                        move |wgt| {
                            if wgt.focus() {
                                let value = wgt.value().parse::<f64>().unwrap_or_default();
                                sender.send(Msg::Targstr(value)).unwrap();
                            }
                        }
                    });
            });
            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Button::new(prt).with_size(250, -1).set_text("Amount wanted (ml):");
                efltk::Entry::new(prt)
                    .with_tooltip("Nicotine strength wanted must be between  0 and value of nicotine base strength")
                    .with_callback({
                        let sender = sender.clone();
                        move |wgt| {
                            if wgt.focus() {
                                let value = wgt.value().parse::<f64>().unwrap_or_default();
                                sender.send(Msg::Targvol(value)).unwrap();
                            }
                        }
                    });
            });
            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Button::new(prt).with_size(250, -1).set_text("Flavour amount (ml):");
                efltk::Entry::new(prt)
                    .with_tooltip("The flavour amount must be between 0 and the base amount minus nicotine base amount!")
                    .with_callback({
                        let sender = sender.clone();
                        move |wgt| {
                            if wgt.focus() {
                                let value = wgt.value().parse::<f64>().unwrap_or_default();
                                sender.send(Msg::Aromavol(value)).unwrap();
                            }
                        }
                    });
            });
            efltk::Separator::new(prt).set_horizontal(true);
            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Button::new(prt).with_size(250, -1).set_text("Nicotin base:");
                self.nicotine_base = efltk::ProgressBar::new(prt);
            });
            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Button::new(prt).with_size(250, -1).set_text("Base:");
                self.base = efltk::ProgressBar::new(prt);
            });
            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Button::new(prt).with_size(250, -1).set_text("Flavour:");
                self.flavour = efltk::ProgressBar::new(prt);
            });
            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Button::new(prt).with_size(250, -1).set_text("Total:");
                efltk::ProgressBar::new(prt).set_value(1.0);
            });
            efltk::Separator::new(prt).set_horizontal(true);
            self.list = efltk::Entry::new(prt).with_editable(false).with_single_line(false);
        });
    }
}
