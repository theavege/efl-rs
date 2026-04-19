mod models {
    #[derive(Default)]
    pub struct Model(usize);
    impl Model {
        pub fn value(&self) -> usize {
            self.0
        }
        pub fn set_value(&mut self, value: usize) {
            self.0 = value;
        }
    }
}
mod components;

use refl::prelude::*;

pub enum Msg {
    Set(usize),
}

#[derive(Default)]
pub struct View([refl::Frame; 4]);

impl Component for View {
    type Event = Msg;
    type State = models::Model;
    fn update(&self, model: &Self::State) {
        for idx in 0..self.0.len() {
            self.0[idx].set_collapse(idx != model.value());
        }
    }
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Set(value) => model.set_value(value),
        };
        true
    }
    fn view(&mut self, parent: &impl ContainerExt, sender: Sender<Self::Event>) {
        let nav = refl::Naviframe::new(parent).insert(|nav| {
            refl::Box::new(nav).with_homogeneous(false).insert(|page| {
                for idx in 0..4 {
                    refl::Box::new(page).with_horizontal(true).insert(|hbox| {
                        self.0[idx] = refl::Frame::new(hbox)
                            .with_autocollapse(false)
                            .with_text(match idx {
                                0 => "Rangers",
                                1 => "Selectors",
                                2 => "Booker",
                                _ => "Converter",
                            })
                            .with_clicked({
                                let sender = sender.clone();
                                move |_| sender.send(Msg::Set(idx)).unwrap()
                            })
                            .insert(|frm| {
                                match idx {
                                    0 => components::Ranger::mount(frm),
                                    1 => components::Selector::mount(frm),
                                    2 => components::Booker::mount(frm),
                                    _ => components::Converter::mount(frm),
                                };
                            });
                    });
                }
                refl::Label::new(page);
            });
            //~ components::Sudoku::mount(nav);
            components::NicCalc::mount(nav);
            components::Calc::mount(nav);
        });
        nav.promote();
        refl::Panel::new(parent).insert(|parent| {
            refl::Box::new(parent).insert(|parent| {
                refl::Button::new(parent).with_text("Simple").on_clicked({
                    let nav = nav.clone();
                    move |_| nav.set_top(0)
                });
                refl::Button::new(parent).with_text("NicCalc").on_clicked({
                    let nav = nav.clone();
                    move |_| nav.set_top(1)
                });
                refl::Button::new(parent).with_text("Calc").on_clicked({
                    let nav = nav.clone();
                    move |_| nav.set_top(2)
                });
                //~ refl::Button::new(parent).with_text("Sudoku").on_clicked({
                //~ let nav = nav.clone();
                //~ move |_| nav.set_top(3)
                //~ });
            });
        });
    }
}

fn main() {
    View::run("Simple");
}
