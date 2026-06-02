use refl::prelude::*;

#[derive(Default)]
pub struct ImageExample {}

impl Component for ImageExample {
    type Event = ();
    type State = ();
    
    fn update(&self, _model: &Self::State) {}
    
    fn handle(_msg: Self::Event, _model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        true
    }
    
    fn view(&mut self, prt: &impl ContainerExt, _sender: Sender<Self::Event>) {
        refl::Box::new(prt).with_vertical(true).inside(|prt| {
            refl::Label::new(prt).with_text("Image Widget Example");
            
            // Create an image widget
            refl::Image::new(prt)
                .with_size(300, 300)
                .with_mmap(false);
        });
    }
}