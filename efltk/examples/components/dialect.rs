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
        pub fn save(&self) {
            std::fs::write(Self::file(), [self.from as u8, self.to as u8]).unwrap();
            std::process::exit(0);
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
        pub fn lang(&self) -> Vec<String> {
            self.lang
                .iter()
                .map(|lang| lang.1.clone())
                .collect::<Vec<String>>()
        }
    }
}

use efltk::prelude::*;
use std::collections::HashMap;

pub enum Msg {
    Run,
    Quit,
    Switch,
    Source(String),
    Target(String),
    SaveAs(String),
    Open(String),
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
            Msg::Open(value) => {
                model.source = std::fs::read_to_string(value).unwrap();
            }
            Msg::SaveAs(value) => {
                std::fs::write(value, model.target.as_bytes()).unwrap();
                return false;
            }
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
            Msg::Quit => {
                model.save();
                return false;
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
        self.to.update((state.lang(), state.to as u32));
        self.from.update((state.lang(), state.from as u32));
        self.source.update(&state.source);
        self.target.update(&state.target);
    }
    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        efltk::Box::new(prt)
            .with_horizontal(true)
            .with_homogeneous(true)
            .inside(|prt| {
                self.from = efltk::List::new(prt).with_callback({
                    let sender = sender.clone();
                    move |wgt| {
                        sender.send(Msg::From(wgt.value() as i32)).unwrap();
                    }
                });
                efltk::Box::new(prt).inside(|prt| {
                    self.source = efltk::Entry::new(prt)
                        .with_single_line(false)
                        .with_callback({
                            let sender = sender.clone();
                            move |wgt| {
                                if wgt.focus() {
                                    sender.send(Msg::Source(wgt.text())).unwrap();
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
                        sender.send(Msg::To(wgt.value() as i32)).unwrap();
                    }
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
