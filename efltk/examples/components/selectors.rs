use efltk::prelude::*;

pub enum Msg {
    Set(String),
}

#[derive(Default)]
pub struct Selector(efltk::Entry);

impl Component for Selector {
    type Event = Msg;
    type State = String;
    fn update(&self, model: &Self::State) {
        self.0.set_value(model);
    }
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Set(value) => *model = value,
        };
        true
    }
    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        let items = [
            "home",
            "close",
            "folder",
            "apps",
            "arrow_up",
            "arrow_down",
            "arrow_left",
            "arrow_right",
            "chat",
            "clock",
            "delete",
            "refresh",
            "folder",
            "file",
            "dialog-info",
        ];
        efltk::Box::new(prt).inside(|prt| {
            efltk::List::new(prt)
                .with_tooltip("List")
                .with_items(&items)
                .with_size(-1, 90);
            efltk::Separator::new(prt);
            self.0 = efltk::Entry::with_menu(
                prt,
                efltk::Menu::popup(prt).with_items(&items).with_callback({
                    let sender = sender.clone();
                    move |wgt| {
                        sender.send(Msg::Set(wgt.selected().text())).unwrap();
                    }
                }),
            );
        });
    }
}
