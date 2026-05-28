mod models {
    #[derive(Default)]
    pub struct Model {
        pub index: u32,
    }
    impl Model {
        pub fn set_index(&mut self, index: u32) {
            self.index = index;
        }
    }
}

use refl::prelude::*;

pub enum Msg {
    Selected(u32),
}

#[derive(Default)]
pub struct GenlistView {
    list: refl::Genlist,
    label: refl::Label,
}

impl Component for GenlistView {
    type Event = Msg;
    type State = models::Model;

    fn update(&self, model: &Self::State) {
        self.label
            .set_text(&format!("Selected index: {}", model.index));
    }

    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Selected(idx) => model.set_index(idx),
        };
        true
    }

    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        let items = [
            "Apple",
            "Banana",
            "Cherry",
            "Date",
            "Elderberry",
            "Fig",
            "Grape",
            "Honeydew",
        ];
        refl::Box::new(prt).with_homogeneous(false).inside(|prt| {
            self.list = refl::Genlist::new(prt)
                .with_multi_select(false)
                .with_items(&items)
                .with_selected({
                    let sender = sender.clone();
                    move |wgt| {
                        sender.send(Msg::Selected(wgt.value())).unwrap();
                    }
                });
            self.label = refl::Label::new(prt).with_text("Selected index: 0");
        });
    }
}
