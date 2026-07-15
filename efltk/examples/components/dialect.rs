mod models {
    #[derive(Default)]
    pub struct Model {
        pub lang: Vec<(String, String)>,
        pub source: String,
        pub target: String,
        pub from: i32,
        pub to: i32,
    }

    impl Model {
        const SERVICE: &str = r#"https://translate.plausibility.cloud/api/v1"#;
        const NAME: &str = "Dialect";
        pub fn read(&mut self, value: Vec<(String, String)>) {
            self.lang = value;
            if let Ok(value) = std::fs::read(Self::file()) {
                self.from = value[0] as i32;
                self.to = value[1] as i32;
            };
        }
        fn file() -> String {
            format!(
                "{}/.config/{}",
                std::env::var(match cfg!(target_os = "windows") {
                    true => "HOMEPATH",
                    false => "HOME",
                })
                .unwrap(),
                Self::NAME
            )
        }
        pub fn switch(&mut self) {
            std::mem::swap(&mut self.from, &mut self.to);
        }
        pub fn url(&self) -> String {
            format!(
                "{}/{}/{}/{}",
                Self::SERVICE,
                self.lang[self.from as usize].0,
                self.lang[self.to as usize].0,
                &self
                    .source
                    .replace("%", "%25")
                    .replace("/", "%20")
                    .replace(r#"\"#, "%20")
                    .replace(" ", "%20")
                    .replace("\n", "%0A")
                    .replace("?", "%3F")
            )
        }
        pub fn lang(&self) -> Vec<&str> {
            let mut rsl = Vec::<&str>::new();
            for (item, _) in &self.lang {
                rsl.push(item);
            }
            rsl
        }
    }
}

use efltk::prelude::*;
use std::collections::HashMap;

pub enum Msg {
    Run,
    Switch,
    Source(String),
    Target(String),
    To(i32),
    From(i32),
    Lang(Vec<(String, String)>),
}

#[derive(Default)]
pub struct Dialect {
    source: efltk::Entry,
    target: efltk::Entry,
    from: efltk::List,
    to: efltk::List,
}

impl Component for Dialect {
    type Event = Msg;
    type State = models::Model;
    fn handle(msg: Self::Event, model: &mut Self::State, sender: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Target(value) => {
                model.target = value;
            }
            Msg::Lang(value) => {
                model.read(value);
            }
            Msg::To(value) => {
                model.to = value;
                return false;
            }
            Msg::From(value) => {
                model.from = value;
            }
            Msg::Source(value) => {
                model.source = value;
            }
            Msg::Switch => {
                model.switch();
            }
            Msg::Run => {
                if model.from != model.to && !model.source.is_empty() {
                    let url = model.url();
                    std::thread::spawn({
                        let sender = sender.clone();
                        move || {
                            if let Ok(value) = reqwest::blocking::get(&url) {
                                let target = value
                                    .json::<HashMap<String, String>>()
                                    .unwrap()
                                    .get("translation")
                                    .unwrap()
                                    .to_string();
                                sender.send(Msg::Target(target)).unwrap();
                            }
                        }
                    });
                };
                return false;
            }
        }
        true
    }
    fn update(&self, state: &Self::State) {
        self.to.update((&state.lang()[..], state.to));
        self.from.update((&state.lang()[..], state.from));
        self.source.update(&state.source);
        self.target.update(&state.target);
    }
    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        const WIDTH: i32 = 125;
        efltk::Box::new(prt)
            .with_horizontal(true)
            .with_homogeneous(true)
            .inside(|prt| {
                efltk::Panes::new(prt).with_right_size(WIDTH).inside(|prt| {
                    self.from = efltk::List::new(prt).with_callback({
                        let sender = sender.clone();
                        move |wgt| {
                            sender.send(Msg::From(wgt.value())).unwrap();
                        }
                    });
                    efltk::Box::new(prt).inside(|prt| {
                        self.source = efltk::Entry::new(prt)
                            .with_single_line(false)
                            .with_callback({
                                let sender = sender.clone();
                                move |wgt| {
                                    if wgt.focus() {
                                        sender.send(Msg::Source(wgt.value())).unwrap();
                                    }
                                }
                            });
                        efltk::Button::new(prt).with_text("Switch").with_callback({
                            let sender = sender.clone();
                            move |_| {
                                sender.send(Msg::Switch).unwrap();
                            }
                        });
                    });
                });
                efltk::Panes::new(prt).with_left_size(WIDTH).inside(|prt| {
                    efltk::Box::new(prt).inside(|prt| {
                        self.target = efltk::Entry::new(prt)
                            .with_single_line(false)
                            .with_editable(false);
                        efltk::Button::new(prt)
                            .with_text("Translate")
                            .with_callback({
                                let sender = sender.clone();
                                move |_| {
                                    sender.send(Msg::Run).unwrap();
                                }
                            });
                    });
                    self.to = efltk::List::new(prt).with_callback({
                        let sender = sender.clone();
                        move |wgt| {
                            sender.send(Msg::To(wgt.value())).unwrap();
                        }
                    });
                });
            });
        std::thread::spawn(move || {
            if let Ok(get) =
                reqwest::blocking::get("https://translate.plausibility.cloud/api/v1/languages")
            {
                let value = get
                    .json::<HashMap<String, Vec<HashMap<String, String>>>>()
                    .unwrap()
                    .get("languages")
                    .unwrap()
                    .iter()
                    .map(|lang| (lang["code"].clone(), lang["name"].clone()))
                    .collect::<Vec<(String, String)>>();
                if !value.is_empty() {
                    sender.send(Msg::Lang(value)).unwrap();
                }
            }
        });
    }
}
