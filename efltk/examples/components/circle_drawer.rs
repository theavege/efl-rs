mod models {
    #[derive(Debug, Default, Clone)]
    pub struct Circle {
        pub x: i32,
        pub y: i32,
        pub diameter: i32,
    }

    impl Circle {
        pub fn new(x: i32, y: i32) -> Self {
            Self {
                x,
                y,
                diameter: 24,
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct Model {
        pub circles: Vec<Circle>,
        pub selected: Option<usize>,
        pub show_diameter_dialog: bool,
        pub diameter_value: i32,
    }

    impl Model {
        pub fn add_circle(&mut self, x: i32, y: i32) {
            self.circles.push(Circle::new(x, y));
            self.selected = Some(self.circles.len() - 1);
        }

        pub fn select_circle(&mut self, index: usize) {
            self.selected = Some(index);
        }

        pub fn set_diameter(&mut self, diameter: i32) {
            if let Some(index) = self.selected {
                self.circles[index].diameter = diameter.clamp(8, 100);
            }
        }

        pub fn show_diameter_dialog(&mut self) {
            if let Some(index) = self.selected {
                self.diameter_value = self.circles[index].diameter;
                self.show_diameter_dialog = true;
            }
        }

        pub fn hide_diameter_dialog(&mut self) {
            self.show_diameter_dialog = false;
        }
    }
}

use efltk::prelude::*;

pub enum Msg {
    CreateCircle,
    SelectCircle(usize),
    ShowDiameterDialog,
    HideDiameterDialog,
    SetDiameter(f64),
}

#[derive(Default)]
pub struct CircleDrawer {
    main_box: efltk::Box,
    canvas: efltk::Frame,
    undo_btn: efltk::Button,
    redo_btn: efltk::Button,
    diameter_slider: efltk::Slider,
    diameter_popup: efltk::Popup,
}

impl Component for CircleDrawer {
    type Event = Msg;
    type State = models::Model;

    fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
        match msg {
            Msg::CreateCircle => {
                // For now, create circle at fixed position
                model.add_circle(100, 100);
            }
            Msg::SelectCircle(index) => {
                model.select_circle(index);
            }
            Msg::ShowDiameterDialog => {
                model.show_diameter_dialog();
            }
            Msg::HideDiameterDialog => {
                model.hide_diameter_dialog();
            }
            Msg::SetDiameter(value) => {
                model.set_diameter(value as i32);
            }
        }
        true
    }

    fn update(&self, model: &Self::State) {
        // Update diameter slider
        if let Some(index) = model.selected {
            let diameter = model.circles[index].diameter;
            if (diameter as f64) != self.diameter_slider.value() {
                self.diameter_slider.set_value(diameter as f64);
            }
        }

        // Show/hide diameter dialog
        // Note: Popup handling needs proper implementation
    }

    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        self.main_box = efltk::Box::new(prt)
            .with_vertical(true)
            .with_homogeneous(false);

        // Button row
        let btn_box = efltk::Box::new(&self.main_box)
            .with_horizontal(true)
            .with_homogeneous(true);

        self.undo_btn = efltk::Button::new(&btn_box)
            .with_text("Undo")
            .with_callback({
                let sender = sender.clone();
                move |_| sender.send(Msg::CreateCircle).unwrap()
            });

        self.redo_btn = efltk::Button::new(&btn_box)
            .with_text("Redo")
            .with_callback({
                let sender = sender.clone();
                move |_| sender.send(Msg::ShowDiameterDialog).unwrap()
            });

        // Canvas area (Frame as placeholder)
        self.canvas = efltk::Frame::new(&self.main_box)
            .with_min_size(400, 400)
            .with_text("Circle Drawer Canvas - Click to add circles");

        // Diameter adjustment popup
        self.diameter_popup = efltk::Popup::new(&self.main_box);
        
        let popup_box = efltk::Box::new(&self.diameter_popup)
            .with_vertical(true);
        
        efltk::Label::new(&popup_box).with_text("Adjust Diameter");
        
        self.diameter_slider = efltk::Slider::new(&popup_box)
            .with_horizontal(true)
            .with_min_size(200, 30)
            .with_range(8.0, 100.0)
            .with_step(1.0)
            .with_callback({
                let sender = sender.clone();
                move |wgt| sender.send(Msg::SetDiameter(wgt.value())).unwrap()
            });
        
        efltk::Button::new(&popup_box)
            .with_text("Close")
            .with_callback({
                let sender = sender.clone();
                move |_| sender.send(Msg::HideDiameterDialog).unwrap()
            });
    }
}
