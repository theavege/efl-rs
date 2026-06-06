use efltk::prelude::*;

pub enum Msg {
    Set(f64),
    Add(f64),
}

#[derive(Default)]
pub struct Ranger {
    progress: efltk::ProgressBar,
    slider: efltk::Slider,
    spinner: efltk::Spinner,
    label: efltk::Label,
}

impl Component for Ranger {
    type Event = Msg;
    type State = f64;
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Set(value) => *model = value,
            Msg::Add(value) => *model += value,
        };
        true
    }
    fn update(&self, model: &Self::State) {
        self.progress.set_value(*model);
        self.slider.update(*model);
        self.spinner.update(*model);
        self.label.set_text(&model.to_string());
    }
    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        efltk::Box::new(prt).with_homogeneous(false).inside(|prt| {
            efltk::Box::new(prt)
                .with_horizontal(true)
                .with_homogeneous(false)
                .inside(|prt| {
                    efltk::Button::new(prt)
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
                    self.label = efltk::Label::new(prt).with_style("marker");
                    efltk::Button::new(prt)
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
            self.spinner = efltk::Spinner::new(prt)
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
            self.progress = efltk::ProgressBar::new(prt).with_size(0, 45);
            self.slider = efltk::Slider::new(prt).with_size(0, 45).with_changed({
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
