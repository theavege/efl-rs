mod models {
    #[derive(Default)]
    pub struct Model {
        pub path: String,
        pub selected: String,
    }
    impl Model {
        pub fn set_path(&mut self, path: String) {
            self.path = path;
        }
        pub fn set_selected(&mut self, selected: String) {
            self.selected = selected;
        }
    }
}

use refl::prelude::*;

pub enum Msg {
    PathChanged(String),
    FileSelected(String),
}

#[derive(Default)]
pub struct FileSelectorView {
    fs: refl::FileSelector,
    label: refl::Label,
}

impl Component for FileSelectorView {
    type Event = Msg;
    type State = models::Model;

    fn update(&self, model: &Self::State) {
        if !model.selected.is_empty() {
            self.label.set_text(&model.selected);
        } else {
            self.label.set_text(&model.path);
        }
    }

    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::PathChanged(path) => model.set_path(path),
            Msg::FileSelected(sel) => model.set_selected(sel),
        };
        true
    }

    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        refl::Box::new(prt).with_homogeneous(false).inside(|prt| {
            self.fs = refl::FileSelector::new(prt)
                .with_path("/tmp")
                .with_expandable(false)
                .with_hidden_visible(false)
                .with_buttons_ok_cancel(true)
                .with_activated({
                    let sender = sender.clone();
                    move |wgt| {
                        sender
                            .send(Msg::FileSelected(wgt.selected()))
                            .unwrap();
                    }
                })
                .with_done({
                    let sender = sender.clone();
                    move |wgt| {
                        sender
                            .send(Msg::PathChanged(wgt.path()))
                            .unwrap();
                    }
                });
            self.label = refl::Label::new(prt).with_text("No file selected");
        });
    }
}
