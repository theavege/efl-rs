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
                diameter: 24, // Default diameter
            }
        }

        pub fn contains(&self, x: i32, y: i32) -> bool {
            let dx = self.x - x;
            let dy = self.y - y;
            let radius = self.diameter / 2;
            dx * dx + dy * dy <= radius * radius
        }

        pub fn radius(&self) -> i32 {
            self.diameter / 2
        }
    }

    #[derive(Debug, Default)]
    pub struct Model {
        pub circles: Vec<Circle>,
        pub selected: Option<usize>,
        pub show_diameter_dialog: bool,
        pub diameter_value: i32,
        pub history: Vec<HistoryAction>,
        pub history_pos: usize,
    }

    impl Model {
        pub fn add_circle(&mut self, x: i32, y: i32) {
            let circle = Circle::new(x, y);
            let index = self.circles.len();
            self.circles.push(circle);
            
            // Clear redo history
            self.history.truncate(self.history_pos);
            self.history.push(HistoryAction::Create(index));
            self.history_pos += 1;
            
            self.selected = Some(index);
        }

        pub fn select_circle(&mut self, index: usize) {
            self.selected = Some(index);
        }

        pub fn set_diameter(&mut self, diameter: i32) {
            if let Some(index) = self.selected {
                let old_diameter = self.circles[index].diameter;
                if old_diameter != diameter {
                    let clamped = diameter.clamp(8, 100);
                    self.circles[index].diameter = clamped;
                    
                    // Clear redo history
                    self.history.truncate(self.history_pos);
                    self.history.push(HistoryAction::Diameter {
                        index,
                        old_diameter,
                        new_diameter: clamped,
                    });
                    self.history_pos += 1;
                }
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

        pub fn undo(&mut self) {
            if self.history_pos > 0 {
                self.history_pos -= 1;
                match self.history[self.history_pos] {
                    HistoryAction::Create(index) => {
                        self.circles.remove(index);
                        // Adjust selected index
                        if let Some(selected) = self.selected {
                            if selected >= index {
                                self.selected = Some(selected - 1);
                            }
                        } else {
                            self.selected = None;
                        }
                    }
                    HistoryAction::Diameter {
                        index,
                        old_diameter,
                        ..
                    } => {
                        self.circles[index].diameter = old_diameter;
                    }
                }
            }
        }

        pub fn redo(&mut self) {
            if self.history_pos < self.history.len() {
                match self.history[self.history_pos] {
                    HistoryAction::Create(index) => {
                        // This is tricky - we need to recreate the circle
                        // For now, we'll just skip redo for creation
                        // In a real implementation, we'd need to store the circle data
                    }
                    HistoryAction::Diameter {
                        index,
                        new_diameter,
                        ..
                    } => {
                        self.circles[index].diameter = new_diameter;
                    }
                }
                self.history_pos += 1;
            }
        }

        pub fn can_undo(&self) -> bool {
            self.history_pos > 0
        }

        pub fn can_redo(&self) -> bool {
            self.history_pos < self.history.len()
        }

        pub fn find_circle_at(&self, x: i32, y: i32) -> Option<usize> {
            self.circles
                .iter()
                .enumerate()
                .find(|(_, c)| c.contains(x, y))
                .map(|(i, _)| i)
        }
    }

    #[derive(Debug, Clone)]
    pub enum HistoryAction {
        Create(usize),
        Diameter {
            index: usize,
            old_diameter: i32,
            new_diameter: i32,
        },
    }
}

use efltk::prelude::*;

pub enum Msg {
    CreateCircle(i32, i32),
    SelectCircle(usize),
    ShowDiameterDialog,
    HideDiameterDialog,
    SetDiameter(i32),
    Undo,
    Redo,
}

#[derive(Default)]
pub struct CircleDrawer {
    canvas: efltk::Box,
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
                model.set_diameter(value);
            }
            Msg::Undo => {
                model.undo();
            }
            Msg::Redo => {
                model.redo();
            }
        }
        true
    }

    fn update(&self, model: &Self::State) {
        // Update undo/redo button sensitivity
        self.undo_btn.set_disabled(!model.can_undo());
        self.redo_btn.set_disabled(!model.can_redo());

        // Update diameter slider value
        if let Some(index) = model.selected {
            let diameter = model.circles[index].diameter;
            if diameter != self.diameter_slider.value() as i32 {
                self.diameter_slider.set_value(diameter as f64);
            }
        }

        // Show/hide diameter dialog
        if model.show_diameter_dialog {
            self.diameter_popup.show();
        } else {
            self.diameter_popup.del();
        }
    }

    fn view(&mut self, prt: &impl ContainerExt, sender: Sender<Self::Event>) {
        // Main vertical box
        let main_box = efltk::Box::new(prt).with_vertical(true).with_homogeneous(false);

        // Button row
        let btn_box = efltk::Box::new(&main_box)
            .with_horizontal(true)
            .with_homogeneous(true);

        self.undo_btn = efltk::Button::new(&btn_box)
            .with_text("Undo")
            .with_callback({
                let sender = sender.clone();
                move |_| sender.send(Msg::Undo).unwrap()
            });

        self.redo_btn = efltk::Button::new(&btn_box)
            .with_text("Redo")
            .with_callback({
                let sender = sender.clone();
                move |_| sender.send(Msg::Redo).unwrap()
            });

        // Canvas area
        self.canvas = efltk::Box::new(&main_box)
            .with_vertical(false)
            .with_homogeneous(true)
            .with_weight(true, true)
            .with_min_size(400, 400);

        // Set up canvas for drawing
        // In EFL, we would use Evas drawing APIs here
        // For now, we'll use a placeholder
        
        // Diameter adjustment popup
        self.diameter_popup = efltk::Popup::new(&main_box);
        
        let popup_box = efltk::Box::new(&self.diameter_popup)
            .with_vertical(true)
            .with_homogeneous(false);
        
        efltk::Label::new(&popup_box).with_text("Adjust Diameter");
        
        self.diameter_slider = efltk::Slider::new(&popup_box)
            .with_horizontal(true)
            .with_min_size(200, 30)
            .with_range(8.0, 100.0)
            .with_step(1.0)
            .with_callback({
                let sender = sender.clone();
                move |wgt| sender.send(Msg::SetDiameter(wgt.value() as i32)).unwrap()
            });
        
        efltk::Button::new(&popup_box)
            .with_text("Close")
            .with_callback({
                let sender = sender.clone();
                move |_| sender.send(Msg::HideDiameterDialog).unwrap()
            });

        // Set up mouse click handling on canvas
        // This would require Evas event callbacks
        // For now, we'll use a placeholder
    }
}
