use refl::prelude::*;

#[derive(Default)]
pub struct PhotoExample {}

impl Component for PhotoExample {
    type Event = ();
    type State = ();
    
    fn update(&self, _model: &Self::State) {}
    
    fn handle(_msg: Self::Event, _model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        true
    }
    
    fn view(&mut self, prt: &impl ContainerExt, _sender: Sender<Self::Event>) {
        refl::Box::new(prt).with_vertical(true).inside(|prt| {
            refl::Label::new(prt).with_text("Photo Widget Example");
            
            // Create a photo widget
            refl::Photo::new(prt)
                .with_size(300, 300)
                .with_fill_inside(true)
                .with_aspect(true);
        });
    }
}