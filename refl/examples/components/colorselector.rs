use refl::prelude::*;

#[derive(Default)]
pub struct ColorselectorDemo;

impl Component for ColorselectorDemo {
    type Event = ();
    type State = ();

    fn handle(_: Self::Event, _: &mut Self::State, _: Sender<Self::Event>) -> bool {
        false
    }

    fn update(&self, _: &Self::State) {}

    fn view(&mut self, prt: &impl ContainerExt, _: Sender<Self::Event>) {
        refl::Box::new(prt).inside(|prt| {
            refl::Label::new(prt)
                .with_text("Color Selector Demo")
                .show();

            let colorselector = refl::Colorselector::new(prt);
            colorselector.show();
        });
    }
}