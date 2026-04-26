mod components;

use refl::prelude::*;

pub enum Msg {
    Set(usize),
}

#[derive(Default)]
pub struct View([refl::Frame; 4]);

impl Component for View {
    type Event = Msg;
    type State = usize;
    fn update(&self, model: &Self::State) {
        for idx in 0..self.0.len() {
            self.0[idx].set_collapse(idx != *model);
        }
    }
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Set(value) => *model = value,
        };
        true
    }
    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        let items = ["Simple", "NicCalc", "Calc"];
        let nav = refl::Naviframe::new(prt).inside(|prt| {
            refl::Box::new(prt)
                .inside(|prt| {
                    for idx in 0..4 {
                        refl::Box::new(prt)
                            .inside(|prt| {
                                self.0[idx] = refl::Frame::new(prt)
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
                                    .inside(|frm| {
                                        match idx {
                                            0 => components::Ranger::mount(frm),
                                            1 => components::Selector::mount(frm),
                                            2 => components::Booker::mount(frm),
                                            _ => components::Converter::mount(frm),
                                        };
                                    });
                            })
                            .with_horizontal(true);
                    }
                    refl::Label::new(prt);
                })
                .with_homogeneous(false);
            //~ components::Sudoku::mount(prt);
            components::NicCalc::mount(prt);
            components::Calc::mount(prt);
        });
        nav.promote();
        refl::Menu::main_menu(prt).with_items(&items, {
            let nav = nav.clone();
            move |wgt| nav.set_top(wgt.index() as usize)
        });
        refl::Panel::new(prt).inside(|prt| {
            refl::List::new(prt).with_items(&items, {
                let nav = nav.clone();
                let prt = prt.clone();
                move |wgt| {
                    if wgt.focus() {
                        nav.set_top(wgt.index() as usize);
                        prt.set_hidden(true);
                    }
                }
            });
        });
    }
}

fn main() {
    View::run("Simple");
}
