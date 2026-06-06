use efltk::prelude::*;

pub enum Msg {
    Set(f64),
}

#[derive(Default)]
pub struct Ranger {
    progress: efltk::ProgressBar,
    slider: efltk::Slider,
    spinner: efltk::Spinner,
}

impl Component for Ranger {
    type Event = Msg;
    type State = f64;
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Set(value) => *model = value,
        };
        true
    }
    fn update(&self, model: &Self::State) {
        self.progress.set_value(*model);
        self.slider.update(*model);
        self.spinner.update(*model);
    }
    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        efltk::Box::new(prt)
            .with_homogeneous(true)
            .with_horizontal(true)
            .inside(|prt| {
                self.spinner = efltk::Spinner::new(prt)
                    .with_tooltip("Spinner")
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
                self.progress = efltk::ProgressBar::new(prt).with_tooltip("ProgressBar");
                self.slider = efltk::Slider::new(prt)
                    .with_tooltip("Slider")
                    .with_format("%1.2f")
                    .with_changed({
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
