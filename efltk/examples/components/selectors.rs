use efltk::prelude::*;

#[derive(Default)]
pub struct Selector(efltk::List, efltk::Button, efltk::Radio);

impl Component for Selector {
    type Event = (i32, String);
    type State = (i32, String);
    fn update(&self, model: &Self::State) {
        self.0.update(model.0);
        self.1.set_text(&model.1);
        self.2.update(model.0);
    }
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        *model = msg;
        true
    }
    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        let items = [
            "home",
            "close",
            "folder",
            "apps",
            "arrow_up",
            "chat",
            "clock",
            "delete",
            "refresh",
            "file",
            "dialog-info",
        ];
        efltk::Box::new(prt).inside(|prt| {
            self.0 = efltk::List::new(&efltk::Box::new(prt).with_size(-1, 90))
                .with_tooltip("List")
                .with_items(&items)
                .with_callback({
                    let sender = sender.clone();
                    move |wgt| {
                        sender.send((wgt.value(), wgt.selected().text())).unwrap();
                    }
                });
            efltk::Separator::new(prt);
            self.1 = efltk::Button::with_menu(
                prt,
                efltk::Menu::popup(prt).with_items(&items).with_callback({
                    let sender = sender.clone();
                    move |wgt| {
                        sender.send((wgt.value(), wgt.selected().text())).unwrap();
                    }
                }),
            );
            efltk::Separator::new(prt);
            self.2 = efltk::Radio::from_items(prt, &items, {
                let sender = sender.clone();
                move |wgt| {
                    sender
                        .send((wgt.value(), items[wgt.value() as usize].to_string()))
                        .unwrap();
                }
            });
        });
    }
}
