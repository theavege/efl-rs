#![forbid(unsafe_code)]

mod components;

use efltk::prelude::*;

pub enum Msg {
    Set(usize),
    Slide(usize),
}

#[derive(Default)]
pub struct View {
    frames: [efltk::Frame; 5],
    nav: efltk::Naviframe,
    mounts: Vec<efltk::ComponentMount>,
}

impl Component for View {
    type Event = Msg;
    type State = (usize, usize);
    fn update(&self, model: &Self::State) {
        self.nav.set_top(model.1);
        for idx in 0..self.frames.len() {
            self.frames[idx].set_value(idx != model.0);
        }
    }
    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Set(value) => model.0 = value,
            Msg::Slide(value) => model.1 = value,
        };
        true
    }
    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        efltk::Box::new(prt).inside(|prt| {
            let items = [
                "home", "Calc", "Sudoku", "NicCalc", "Siege", "Search", "Dialect",
            ];
            efltk::SegmentControl::new(prt)
                .with_items(&items)
                .with_callback({
                    let sender = sender.clone();
                    move |wgt| sender.send(Msg::Slide(wgt.value() as usize)).unwrap()
                });
            self.nav = efltk::Naviframe::new(prt);
            self.nav.inside(|prt| {
                efltk::Box::new(prt).inside(|prt| {
                    for (idx, item) in ["Inputs", "Rangers", "Selectors", "Booker", "CRUD"]
                        .iter()
                        .enumerate()
                    {
                        self.frames[idx] = efltk::Frame::new(prt).with_text(item).with_callback({
                            let sender = sender.clone();
                            move |_| sender.send(Msg::Set(idx)).unwrap()
                        });
                        match idx {
                            0 => self
                                .mounts
                                .push(components::Converter::mount(&self.frames[idx])),
                            1 => self
                                .mounts
                                .push(components::Ranger::mount(&self.frames[idx])),
                            2 => self
                                .mounts
                                .push(components::Selector::mount(&self.frames[idx])),
                            3 => self
                                .mounts
                                .push(components::Booker::mount(&self.frames[idx])),
                            _ => self.mounts.push(components::Crud::mount(&self.frames[idx])),
                        };
                    }
                    efltk::Label::new(prt);
                });
                self.mounts.push(components::Calc::mount(prt));
                self.mounts.push(components::Sudoku::mount(prt));
                self.mounts.push(components::NicCalc::mount(prt));
                self.mounts.push(components::Siege::mount(prt));
                self.mounts.push(components::Search::mount(prt));
                self.mounts.push(components::Dialect::mount(prt));
                prt.promote();
            });
        });
    }
}

fn main() {
    View::run("Simple", 400, 640);
}
