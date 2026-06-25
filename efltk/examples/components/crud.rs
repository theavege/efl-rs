use efltk::prelude::*;

mod models {
    #[derive(Default, Clone, PartialEq, Eq)]
    pub struct Name {
        pub first: String,
        pub last: String,
    }

    impl Name {
        pub fn full(&self) -> String {
            format!(
                "{}, {}",
                self.last.to_uppercase(),
                self.first.to_uppercase()
            )
        }
    }

    #[derive(Default)]
    pub struct Model {
        pub names: Vec<Name>,
        pub prefix: String,
        pub first: String,
        pub last: String,
        pub selected: Option<String>,
    }

    impl Model {
        pub fn filtered(&self) -> Vec<String> {
            let mut result: Vec<String> = self
                .names
                .iter()
                .filter(|n| {
                    if self.prefix.is_empty() {
                        true
                    } else {
                        n.last
                            .to_uppercase()
                            .starts_with(&self.prefix.to_uppercase())
                    }
                })
                .map(|n| n.full())
                .collect();
            result.sort();
            result
        }

        pub fn find_index(&self, name: &str) -> Option<usize> {
            self.filtered().iter().position(|n| n == name)
        }
    }
}

pub enum Msg {
    SetPrefix(String),
    SetFirst(String),
    SetLast(String),
    Select(String),
    Create,
    Update,
    Delete,
}

#[derive(Default)]
pub struct Crud {
    prefix_entry: efltk::Entry,
    first_entry: efltk::Entry,
    last_entry: efltk::Entry,
    list: efltk::List,
    create_btn: efltk::Button,
    update_btn: efltk::Button,
    delete_btn: efltk::Button,
}

impl Component for Crud {
    type Event = Msg;
    type State = models::Model;

    fn update(&self, model: &Self::State) {
        self.prefix_entry.set_value(&model.prefix);
        self.first_entry.set_value(&model.first);
        self.last_entry.set_value(&model.last);

        self.list.clear();
        for name in &model.filtered() {
            self.list.add(name);
        }

        if let Some(ref selected) = model.selected
            && let Some(index) = model.find_index(selected)
        {
            self.list.set_value(index as u32);
        }

        let has_selection = model.selected.is_some();
        self.update_btn.set_disabled(!has_selection);
        self.delete_btn.set_disabled(!has_selection);
    }

    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::SetPrefix(value) => {
                model.prefix = value;
                model.selected = None;
            }
            Msg::SetFirst(value) => {
                model.first = value;
            }
            Msg::SetLast(value) => {
                model.last = value;
            }
            Msg::Select(name) => {
                model.selected = Some(name);
            }
            Msg::Create => {
                if !model.first.is_empty() && !model.last.is_empty() {
                    let name = models::Name {
                        first: model.first.clone(),
                        last: model.last.clone(),
                    };
                    let full_name = name.full();
                    if !model.names.iter().any(|n| n.full() == full_name) {
                        model.names.push(name);
                        model.names.sort_by_key(|a| a.full());
                        model.first = String::new();
                        model.last = String::new();
                        model.selected = None;
                    }
                }
            }
            Msg::Update => {
                if let Some(ref selected_name) = model.selected
                    && !model.first.is_empty()
                    && !model.last.is_empty()
                {
                    let new_name = models::Name {
                        first: model.first.clone(),
                        last: model.last.clone(),
                    };
                    let new_full = new_name.full();
                    if !model.names.iter().any(|n| n.full() == new_full)
                        && let Some(index) =
                            model.names.iter().position(|n| n.full() == *selected_name)
                    {
                        model.names[index] = new_name;
                        model.names.sort_by_key(|a| a.full());
                        model.selected = Some(new_full);
                        model.first = String::new();
                        model.last = String::new();
                    }
                }
            }
            Msg::Delete => {
                if let Some(ref selected_name) = model.selected
                    && let Some(index) = model.names.iter().position(|n| n.full() == *selected_name)
                {
                    model.names.remove(index);
                    model.selected = None;
                    model.first = String::new();
                    model.last = String::new();
                }
            }
        }
        true
    }

    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        efltk::Box::new(prt).with_horizontal(false).inside(|prt| {
            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Label::new(prt).with_text("Filter prefix: ");
                self.prefix_entry = efltk::Entry::new(prt).with_callback({
                    let sender = sender.clone();
                    move |wgt| {
                        sender.send(Msg::SetPrefix(wgt.value())).unwrap();
                    }
                });
            });

            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Label::new(prt).with_text("Name: ");
                self.first_entry = efltk::Entry::new(prt).with_callback({
                    let sender = sender.clone();
                    move |wgt| {
                        sender.send(Msg::SetFirst(wgt.value())).unwrap();
                    }
                });
            });

            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                efltk::Label::new(prt).with_text("Surname: ");
                self.last_entry = efltk::Entry::new(prt).with_callback({
                    let sender = sender.clone();
                    move |wgt| {
                        sender.send(Msg::SetLast(wgt.value())).unwrap();
                    }
                });
            });

            self.list = efltk::List::new(prt)
                .with_weight(true, true)
                .with_callback({
                    let sender = sender.clone();
                    move |wgt| {
                        sender.send(Msg::Select(wgt.selected().text())).unwrap();
                    }
                });

            efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                self.create_btn = efltk::Button::new(prt).with_text("Create").with_callback({
                    let sender = sender.clone();
                    move |_| {
                        sender.send(Msg::Create).unwrap();
                    }
                });

                self.update_btn = efltk::Button::new(prt)
                    .with_text("Update")
                    .with_disabled(true)
                    .with_callback({
                        let sender = sender.clone();
                        move |_| {
                            sender.send(Msg::Update).unwrap();
                        }
                    });

                self.delete_btn = efltk::Button::new(prt)
                    .with_text("Delete")
                    .with_disabled(true)
                    .with_callback({
                        let sender = sender.clone();
                        move |_| {
                            sender.send(Msg::Delete).unwrap();
                        }
                    });
            });
        });
    }
}
