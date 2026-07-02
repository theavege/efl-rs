#![forbid(unsafe_code)]

mod components;

use efltk::prelude::*;

pub enum Msg {
    Set(usize),
    Slide(usize),
}

#[derive(Default)]
pub struct View([efltk::Frame; 5], efltk::Naviframe);

impl Component for View {
    type Event = Msg;
    type State = (usize, usize);
    fn update(&self, model: &Self::State) {
        self.1.set_top(model.1);
        for idx in 0..self.0.len() {
            self.0[idx].set_value(idx != model.0);
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
            self.1 = efltk::Naviframe::new(prt);
            self.1.inside(|prt| {
                efltk::Box::new(prt).inside(|prt| {
                    for (idx, item) in ["Inputs", "Rangers", "Selectors", "Booker", "CRUD"]
                        .iter()
                        .enumerate()
                    {
                        self.0[idx] = efltk::Frame::new(prt).with_text(item).with_callback({
                            let sender = sender.clone();
                            move |_| sender.send(Msg::Set(idx)).unwrap()
                        });
                        match idx {
                            0 => components::Converter::mount(&self.0[idx]),
                            1 => components::Ranger::mount(&self.0[idx]),
                            2 => components::Selector::mount(&self.0[idx]),
                            3 => components::Booker::mount(&self.0[idx]),
                            _ => components::Crud::mount(&self.0[idx]),
                        };
                    }
                    efltk::Label::new(prt);
                });
                components::Calc::mount(prt);
                components::Sudoku::mount(prt);
                components::NicCalc::mount(prt);
                components::Siege::mount(prt);
                components::Search::mount(prt);
                components::Dialect::mount(prt);
                prt.promote();
            });
        });
    }
}

fn main() {
    View::run("Simple", 400, 640);
}
