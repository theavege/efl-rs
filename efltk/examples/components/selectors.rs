use efltk::prelude::*;

pub enum Msg {
    Set(u32),
}

#[derive(Default)]
pub struct Selector {
    list: efltk::Menu,
    butt: efltk::Button,
}

impl Component for Selector {
    type Event = Msg;
    type State = u32;
    fn update(&self, model: &Self::State) {
        self.list.set_value(*model);
        self.butt.set_text(&model.to_string());
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
            self.list = efltk::Menu::popup(prt).with_items(&items).with_callback(
                SelectorSignal::Selected,
                {
                    let sender = sender.clone();
                    move |wgt| {
                        sender.send(Msg::Set(wgt.value())).unwrap();
                    }
                },
            );
            self.butt = efltk::Button::new(prt).with_icon("home").with_callback({
                let menu = self.list.clone();
                move |wgt| {
                    let pos = wgt.geometry();
                    menu.open(pos.0, pos.1 + pos.3);
                }
            });
        });
    }
}
