mod models {
    use {regex::Regex, std::fs, walkdir::WalkDir};

    #[derive(Default)]
    pub struct Model {
        pub path: String,
        pub ext: String,
        pub reg: String,
        pub log: String,
        pub case: bool,
        pub limit: usize,
    }

    impl Model {
        pub fn search(&self) -> String {
            if self.reg.is_empty() {
                String::new()
            } else {
                let extensions = Regex::new(&format!(
                    r#"({})$"#,
                    self.ext
                        .split(",")
                        .map(|item| {
                            format!(
                                r#"{}{item}"#,
                                if item.starts_with(".") { r#"\"# } else { "" }
                            )
                        })
                        .collect::<Vec<String>>()
                        .join("|"),
                ))
                .unwrap();
                let regex = Regex::new(&format!(
                    r#"{}{}"#,
                    if self.case { "" } else { r#"(?i)"# },
                    self.reg,
                ))
                .unwrap();
                let mut output: Vec<String> = Vec::with_capacity(self.limit);
                for entry in WalkDir::new(&self.path)
                    .into_iter()
                    .filter_map(|entry| entry.ok())
                    .filter(|entry| {
                        entry.file_type().is_file()
                            && extensions.is_match(entry.file_name().to_str().unwrap())
                    })
                {
                    if output.len() < self.limit {
                        if regex.is_match(&fs::read_to_string(entry.path()).unwrap()) {
                            output.push(entry.path().to_string_lossy().to_string());
                        }
                    } else {
                        output.push(format!("Found files exceeded the limit of {}", self.limit));
                        break;
                    }
                }
                let result = output.join("\n");
                if !self.log.is_empty() {
                    fs::write(&self.log, result.as_bytes()).unwrap();
                };
                result
            }
        }
    }
}

use efltk::prelude::*;

pub enum Msg {
    Path(String),
    Ext(String),
    Log(String),
    Reg(String),
    Case(bool),
    Limit(usize),
    Run,
}

#[derive(Default)]
pub struct Search {
    path: efltk::Entry,
    ext: efltk::Entry,
    reg: efltk::Entry,
    log: efltk::Entry,
    limit: efltk::Entry,
    case: efltk::Check,
}

impl Component for Search {
    type Event = Msg;
    type State = models::Model;
    fn update(&self, _model: &Self::State) {}
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Path(value) => model.path = value,
            Msg::Ext(value) => model.ext = value,
            Msg::Log(value) => model.log = value,
            Msg::Case(value) => model.case = value,
            Msg::Limit(value) => model.limit = value,
            Msg::Reg(value) => model.reg = value,
            Msg::Run => model.log = model.search(),
        }
        true
    }
    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        const WIDTH: i32 = 150;
        efltk::Box::new(prt).inside(|prt| {
            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Button::new(prt)
                    .with_size(WIDTH, -1)
                    .set_text("PATH: ");
                self.path = efltk::Entry::new(prt)
                    .with_text(
                        &std::env::var(match cfg!(target_os = "windows") {
                            true => "%HOMEPATH%",
                            false => "HOME",
                        })
                        .unwrap(),
                    )
                    .with_callback({
                        let sender = sender.clone();
                        move |wgt| {
                            if wgt.focus() {
                                sender.send(Msg::Path(wgt.value())).unwrap();
                            }
                        }
                    });
            });
            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Button::new(prt)
                    .with_size(WIDTH, -1)
                    .set_text("EXTENSIONS: ");
                self.ext = efltk::Entry::new(prt).with_text("txt,md").with_callback({
                    let sender = sender.clone();
                    move |wgt| {
                        if wgt.focus() {
                            if wgt.value().is_empty() {
                                efltk::Popup::warning(
                                    &wgt.window(),
                                    "You need at least 1 file type!",
                                )
                            } else if wgt.value().split(",").count() > 25 {
                                efltk::Popup::warning(&wgt.window(), "Maximum of 25 file types!")
                            } else {
                                sender.send(Msg::Ext(wgt.value())).unwrap()
                            }
                        }
                    }
                });
            });
            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Button::new(prt)
                    .with_size(WIDTH, -1)
                    .set_text("REGEXP: ");
                self.reg = efltk::Entry::new(prt).with_text("*").with_callback({
                    let sender = sender.clone();
                    move |wgt| {
                        if wgt.focus() {
                            if wgt.value().is_empty() {
                                efltk::Popup::warning(
                                    &wgt.window(),
                                    "You need a search regular expression!",
                                )
                            } else if regex::Regex::new(&wgt.value()).is_err() {
                                efltk::Popup::warning(&wgt.window(), "Maximum of 25 file types!")
                            } else {
                                sender.send(Msg::Reg(wgt.value())).unwrap()
                            }
                        }
                    }
                });
            });
            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Button::new(prt)
                    .with_size(WIDTH, -1)
                    .set_text("LIMIT: ");
                self.limit = efltk::Entry::new(prt).with_text("0").with_callback({
                    let sender = sender.clone();
                    move |wgt| {
                        if wgt.focus() {
                            let value = wgt.value().parse::<usize>().unwrap_or_default();
                            if (1..=9999).contains(&value) {
                                sender.send(Msg::Limit(value)).unwrap();
                            } else {
                                efltk::Popup::warning(
                                    &wgt.window(),
                                    "Max found files must be a number from 1 to 9999!",
                                )
                            }
                        }
                    }
                });
            });
            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Button::new(prt)
                    .with_size(WIDTH, -1)
                    .set_text("RESULT: ");
                self.log = efltk::Entry::new(prt)
                    .with_single_line(false)
                    .with_editable(false)
                    .with_signal(Signal::Clicked, {
                        let sender = sender.clone();
                        move |wgt| sender.send(Msg::Log(wgt.value())).unwrap()
                    });
            });
            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Button::new(prt)
                    .with_size(WIDTH, -1)
                    .set_text("CASE: ");
                self.case = efltk::Check::new(prt)
                    .with_text("sensitive")
                    .with_callback({
                        let sender = sender.clone();
                        move |wgt| sender.send(Msg::Case(wgt.value())).unwrap()
                    });
            });
            efltk::Button::new(prt).with_text("Search").with_callback({
                let sender = sender.clone();
                move |_| sender.send(Msg::Run).unwrap()
            });
            efltk::Label::new(prt);
        });
    }
}
