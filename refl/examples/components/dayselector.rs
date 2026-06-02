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

use refl::prelude::*;

pub enum Msg {
    Set(u32),
}

#[derive(Default)]
pub struct DayselectorDemo {
    dayselector: refl::Dayselector,
}

impl Component for DayselectorDemo {
    type Event = Msg;
    type State = models::Model;
    fn update(&self, model: &Self::State) {
        self.dayselector.set_value(model.value());
    }
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Set(value) => model.set_value(value),
        };
        true
    }
    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        let items = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
        refl::Box::new(prt).inside(|prt| {
            self.dayselector = refl::Dayselector::new(prt)
                .with_size(0, 30)
                .with_items(&items)
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
