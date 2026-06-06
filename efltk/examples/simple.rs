#![forbid(unsafe_code)]

mod components;

use efltk::prelude::*;

pub enum Msg {
    Set(usize),
    Slide(usize),
}

#[derive(Default)]
pub struct View([efltk::Frame; 4], efltk::Naviframe);

impl Component for View {
    type Event = Msg;
    type State = (usize, usize);
    fn update(&self, model: &Self::State) {
        self.1.set_top(model.1);
        for idx in 0..self.0.len() {
            self.0[idx].set_collapse(idx != model.0);
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
        let items = ["Simple", "Calc", "Sudoku", "Dialect", "NicCalc"];
        efltk::Menu::main_menu(prt).with_appends(&items, {
            let sender = sender.clone();
            move |wgt| sender.send(Msg::Slide(wgt.value() as usize)).unwrap()
        });
        self.1 = efltk::Naviframe::new(prt);
        self.1.inside(|prt| {
            efltk::Box::new(prt).inside(|prt| {
                for (idx, item) in ["Inputs", "Rangers", "Selectors", "Booker"]
                    .iter()
                    .enumerate()
                {
                    efltk::Box::new(prt).with_horizontal(true).inside(|prt| {
                        self.0[idx] = efltk::Frame::new(prt)
                            .with_autocollapse(false)
                            .with_text(item)
                            .with_clicked({
                                let sender = sender.clone();
                                move |_| sender.send(Msg::Set(idx)).unwrap()
                            });
                        self.0[idx].inside(move |prt| {
                            match idx {
                                0 => components::Converter::mount(prt),
                                1 => components::Ranger::mount(prt),
                                2 => components::Selector::mount(prt),
                                _ => components::Booker::mount(prt),
                            };
                        });
                    })
                }
                efltk::Label::new(prt);
            });
            components::Calc::mount(prt);
            components::Sudoku::mount(prt);
            components::Dialect::mount(prt);
            components::NicCalc::mount(prt);
        });
        self.1.promote();
    }
}

fn main() {
    View::run("Simple");
}
