mod models {
    #[derive(Default)]
    pub struct Model(f64);
    impl Model {
        pub fn value(&self) -> f64 {
            self.0
        }
        pub fn set_value(&mut self, value: f64) {
            self.0 = value;
        }
        pub fn add_value(&mut self, value: f64) {
            self.0 += value;
        }
    }
}

use refl::prelude::*;

pub enum Msg {
    Set(f64),
    Add(f64),
}

#[derive(Default)]
pub struct Ranger {
    progress: refl::ProgressBar,
    slider: refl::Slider,
    spinner: refl::Spinner,
    label: refl::Label,
}

impl Component for Ranger {
    type Event = Msg;
    type State = models::Model;
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Set(value) => model.set_value(value),
            Msg::Add(value) => model.add_value(value),
        };
        true
    }
    fn update(&self, model: &Self::State) {
        self.progress.set_value(model.value());
        self.slider.set_value(model.value());
        self.spinner.set_value(model.value());
        self.label.set_text(&model.value().to_string());
    }
    fn view(&mut self, parent: &impl ContainerExt, sender: Sender<Self::Event>) {
        refl::Box::new(parent)
            .with_homogeneous(false)
            .insert(|vbox| {
                refl::Box::new(vbox)
                    .with_horizontal(true)
                    .with_homogeneous(false)
                    .insert(|hbox| {
                        refl::Button::new(hbox)
                            .with_size(45, 45)
                            .with_icon("media_player/prev")
                            .on_clicked({
                                let sender = sender.clone();
                                move |wgt| {
                                    if wgt.focus() {
                                        sender.send(Msg::Add(-0.1)).unwrap();
                                    }
                                }
                            });
                        self.label = refl::Label::new(hbox).with_style("marker");
                        refl::Button::new(hbox)
                            .with_size(45, 45)
                            .with_icon("media_player/next")
                            .on_clicked({
                                let sender = sender.clone();
                                move |wgt| {
                                    if wgt.focus() {
                                        sender.send(Msg::Add(0.1)).unwrap();
                                    }
                                }
                            });
                    });
                self.spinner = refl::Spinner::new(vbox)
                    .with_size(0, 45)
                    .with_range(0.0, 1.0)
                    .with_step(0.1)
                    .with_format("%1.2f")
                    .with_changed({
                        let sender = sender.clone();
                        move |wgt| {
                            if wgt.focus() {
                                sender.send(Msg::Set(wgt.value())).unwrap();
                            }
                        }
                    });
                self.progress = refl::ProgressBar::new(vbox).with_size(0, 45);
                self.slider = refl::Slider::new(vbox).with_size(0, 45).with_changed({
                    let sender = sender.clone();
                    move |wgt| {
                        if wgt.focus() {
                            sender.send(Msg::Set(wgt.value())).unwrap();
                        }
                    }
                });
            });
    }
}
