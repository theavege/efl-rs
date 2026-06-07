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
        //~ self.back.set_enable(model.flight);
        self.back.set_value(&model.back);
    }
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Start(value) => {
                model.start = value;
                false
            }
            Msg::Back(value) => {
                model.back = value;
                false
            }
            Msg::Flight(value) => {
                model.flight = value;
                true
            }
            Msg::Book => true,
        };
        true
    }
    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        efltk::Box::new(prt).with_homogeneous(true).inside(|prt| {
            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Button::new(prt).with_size(150, 0).set_text("Flight");
                self.flight = efltk::Check::new(prt).with_text("Return").with_callback(
                    CheckSignal::Changed,
                    {
                        let sender = sender.clone();
                        move |wgt| sender.send(Msg::Flight(wgt.value())).unwrap()
                    },
                );
                self.book = efltk::Button::new(prt)
                    .with_icon("home")
                    .with_size(45, 0)
                    .with_tooltip("Book")
                    .with_style("anchor")
                    .with_callback(ButtonSignal::Clicked, {
                        let sender = sender.clone();
                        move |_wgt| sender.send(Msg::Book).unwrap()
                    });
            });
            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Button::new(prt)
                    .with_size(150, 0)
                    .set_text("Departure data");
                self.start = efltk::Entry::new(prt).with_callback(EntrySignal::Changed, {
                    let sender = sender.clone();
                    move |wgt| {
                        if wgt.focus() {
                            match chrono::NaiveDate::parse_from_str(&wgt.text(), "%Y-%m-%d").is_ok()
                            {
                                true => sender.send(Msg::Start(wgt.text())).unwrap(),
                                false => efltk::Popup::new(&wgt)
                                    .with_timeout(0.0)
                                    .set_message("home", "ERROR", "ERROR"),
                            }
                        }
                    }
                });
            });
            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Button::new(prt)
                    .with_size(150, 0)
                    .set_text("Return data");
                self.back = efltk::Entry::new(prt).with_callback(EntrySignal::Changed, {
                    let sender = sender.clone();
                    move |wgt| {
                        if wgt.focus() {
                            match chrono::NaiveDate::parse_from_str(&wgt.text(), "%Y-%m-%d").is_ok()
                            {
                                true => sender.send(Msg::Back(wgt.text())).unwrap(),
                                false => efltk::Popup::new(&wgt)
                                    .with_timeout(0.0)
                                    .set_message("home", "ERROR", "ERROR"),
                            }
                        }
                    }
                });
            });
        });
    }
}
