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

        pub fn contains(&self, x: i32, y: i32) -> bool {
            let dx = self.x - x;
            let dy = self.y - y;
            let radius = self.diameter / 2;
            dx * dx + dy * dy <= radius * radius
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

        pub fn find_circle_at(&self, x: i32, y: i32) -> Option<usize> {
            self.circles
                .iter()
                .enumerate()
                .find(|(_, c)| c.contains(x, y))
                .map(|(i, _)| i)
        }
    }
}

use efltk::prelude::*;

pub enum Msg {
    CreateCircle(i32, i32),
    SelectCircle(usize),
    ShowDiameterDialog,
    HideDiameterDialog,
    SetDiameter(f64),
    Redraw,
}

#[derive(Default)]
pub struct CircleDrawer {
    canvas: efltk::Canvas,
    undo_btn: efltk::Button,
    redo_btn: efltk::Button,
    diameter_slider: efltk::Slider,
    diameter_popup: efltk::Popup,
}

impl Component for CircleDrawer {
    type Event = Msg;
    type State = models::Model;

    fn handle(msg: Self::Event, model: &mut Self::State, sender: Sender<Self::Event>) -> bool {
        match msg {
            Msg::CreateCircle(x, y) => {
                model.add_circle(x, y);
                sender.send(Msg::Redraw).unwrap();
            }
            Msg::SelectCircle(index) => {
                model.select_circle(index);
                sender.send(Msg::Redraw).unwrap();
            }
            Msg::ShowDiameterDialog => {
                model.show_diameter_dialog();
            }
            Msg::HideDiameterDialog => {
                model.hide_diameter_dialog();
            }
            Msg::SetDiameter(value) => {
                model.set_diameter(value as i32);
                sender.send(Msg::Redraw).unwrap();
            }
            Msg::Redraw => {
                // Redraw will be handled in update
            }
        }
        true
    }

    fn update(&self, model: &Self::State) {
        // Clear and redraw all circles
        self.canvas.clear();
        
        for (i, circle) in model.circles.iter().enumerate() {
            let radius = circle.diameter / 2;
            let is_selected = Some(i) == model.selected;
            
            // Draw circle: filled if selected, outline if not
            if is_selected {
                // Selected circle is filled gray
                self.canvas.draw_circle(
                    circle.x,
                    circle.y,
                    radius,
                    true,
                    (128, 128, 128, 255), // Gray
                );
            } else {
                // Unselected circle is outline (black border)
                self.canvas.draw_circle(
                    circle.x,
                    circle.y,
                    radius,
                    false,
                    (0, 0, 0, 255), // Black
                );
            }
        }

        // Update diameter slider
        if let Some(index) = model.selected {
            let diameter = model.circles[index].diameter;
            if (diameter as f64) != self.diameter_slider.value() {
                self.diameter_slider.set_value(diameter as f64);
            }
        }
    }

    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        let main_box = efltk::Box::new(prt)
            .with_vertical(true)
            .with_homogeneous(false);

        // Button row
        let btn_box = efltk::Box::new(&main_box)
            .with_horizontal(true)
            .with_homogeneous(true);

        self.undo_btn = efltk::Button::new(&btn_box)
            .with_text("Undo")
            .with_callback({
                let sender = sender.clone();
                move |_| sender.send(Msg::CreateCircle(100, 100)).unwrap()
            });

        self.redo_btn = efltk::Button::new(&btn_box)
            .with_text("Redo")
            .with_callback({
                let sender = sender.clone();
                move |_| sender.send(Msg::ShowDiameterDialog).unwrap()
            });

        // Canvas area
        self.canvas = efltk::Canvas::new(&main_box);

        // Diameter adjustment popup
        self.diameter_popup = efltk::Popup::new(&main_box);
        
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
