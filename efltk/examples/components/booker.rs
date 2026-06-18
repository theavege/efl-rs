mod models {
    pub struct Model {
        pub start: String,
        pub back: String,
        pub flight: bool,
    }
    impl Default for Model {
        fn default() -> Self {
            let current_date = chrono::offset::Local::now()
                .naive_local()
                .date()
                .format("%Y-%m-%d")
                .to_string();
            Self {
                start: current_date.clone(),
                back: current_date,
                flight: false,
            }
        }
    }
}

use efltk::prelude::*;

pub enum Msg {
    Book,
    Flight(bool),
    Start(String),
    Back(String),
}

#[derive(Default)]
pub struct Booker {
    start: efltk::Entry,
    back: efltk::Entry,
    flight: efltk::Check,
    book: efltk::Button,
}

impl Component for Booker {
    type Event = Msg;
    type State = models::Model;
    fn update(&self, model: &Self::State) {
        self.flight.set_value(model.flight);
        self.start.set_value(&model.start);
        self.back.set_disabled(model.flight);
        self.back.set_value(&model.back);
    }
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Start(value) => {
                model.start = value;
                return false;
            }
            Msg::Back(value) => {
                model.back = value;
                return false;
            }
            Msg::Flight(value) => model.flight = value,
            Msg::Book => {}
        };
        true
    }
    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        efltk::Box::new(prt).inside(|prt| {
            efltk::Box::new(prt)
                .with_homogeneous(true)
                .with_horizontal(true)
                .inside(|prt| {
                    self.start =
                        efltk::Entry::new(&efltk::Bubble::new(prt).with_info("Departure data"))
                            .with_signal(Signal::Unfocused, {
                                let sender = sender.clone();
                                move |wgt| match chrono::NaiveDate::parse_from_str(
                                    &wgt.text(),
                                    "%Y-%m-%d",
                                )
                                .is_ok()
                                {
                                    true => sender.send(Msg::Start(wgt.text())).unwrap(),
                                    false => efltk::Popup::warning(&wgt.window(), "ERROR"),
                                }
                            });
                    self.back =
                        efltk::Entry::new(&efltk::Bubble::new(prt).with_info("Return data"))
                            .with_signal(Signal::Unfocused, {
                                let sender = sender.clone();
                                move |wgt| match chrono::NaiveDate::parse_from_str(
                                    &wgt.text(),
                                    "%Y-%m-%d",
                                )
                                .is_ok()
                                {
                                    true => sender.send(Msg::Back(wgt.text())).unwrap(),
                                    false => efltk::Popup::new(&wgt.window())
                                        .with_timeout(0.0)
                                        .set_message("home", "ERROR", "ERROR"),
                                }
                            });
                });
            efltk::Box::new(prt)
                .with_homogeneous(true)
                .with_horizontal(true)
                .inside(|prt| {
                    self.flight = efltk::Check::new(&efltk::Bubble::new(prt).with_info("Flight"))
                        .with_tooltip("Check")
                        .with_text("Return")
                        .with_callback({
                            let sender = sender.clone();
                            move |wgt| sender.send(Msg::Flight(wgt.value())).unwrap()
                        });
                    self.book = efltk::Button::new(prt)
                        .with_icon("home")
                        .with_tooltip("Book")
                        .with_callback({
                            let sender = sender.clone();
                            move |_wgt| sender.send(Msg::Book).unwrap()
                        });
                });
        });
    }
}
