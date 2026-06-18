mod models {
    #[derive(Debug, Default, Clone)]
    pub struct Circle {
        pub x: i32,
        pub y: i32,
        pub diameter: i32,
    }

    impl Circle {
        pub fn new(x: i32, y: i32, diameter: i32) -> Self {
            Self { x, y, diameter }
        }

        pub fn radius(&self) -> i32 {
            self.diameter / 2
        }

        pub fn contains(&self, x: i32, y: i32) -> bool {
            let dx = self.x - x;
            let dy = self.y - y;
            let distance_squared = dx * dx + dy * dy;
            let radius = self.radius();
            distance_squared < radius * radius
        }
    }

    #[derive(Debug, Default)]
    pub struct Model {
        pub circles: Vec<Circle>,
        pub selected: Option<usize>,
        pub history: Vec<HistoryItem>,
        pub history_index: usize,
        pub show_diameter_dialog: bool,
        pub diameter_value: i32,
        pub mouse_x: i32,
        pub mouse_y: i32,
    }

    #[derive(Debug, Clone)]
    pub enum HistoryItem {
        AddCircle(Circle),
        UpdateDiameter(usize, i32),
    }

    impl Model {
        pub fn add_circle(&mut self, x: i32, y: i32) {
            let circle = Circle::new(x, y, 24);
            // Clear redo history
            self.history.truncate(self.history_index);
            self.history.push(HistoryItem::AddCircle(circle.clone()));
            self.history_index += 1;
            self.circles.push(circle);
            self.selected = Some(self.circles.len() - 1);
        }

        pub fn select_circle(&mut self, index: Option<usize>) {
            self.selected = index;
        }

        pub fn set_diameter(&mut self, diameter: i32) {
            if let Some(index) = self.selected {
                let old_diameter = self.circles[index].diameter;
                if diameter != old_diameter {
                    // Clear redo history
                    self.history.truncate(self.history_index);
                    self.history.push(HistoryItem::UpdateDiameter(index, old_diameter));
                    self.history_index += 1;
                    self.circles[index].diameter = diameter.clamp(8, 100);
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
            if self.history_index > 0 {
                self.history_index -= 1;
                match &self.history[self.history_index] {
                    HistoryItem::AddCircle(_) => {
                        self.circles.pop();
                        if let Some(idx) = self.selected {
                            if idx >= self.circles.len() {
                                self.selected = None;
                            }
                        }
                    }
                    HistoryItem::UpdateDiameter(index, old_diameter) => {
                        if let Some(circle) = self.circles.get_mut(*index) {
                            circle.diameter = *old_diameter;
                        }
                    }
                }
            }
        }

        pub fn redo(&mut self) {
            if self.history_index < self.history.len() {
                match &self.history[self.history_index] {
                    HistoryItem::AddCircle(circle) => {
                        self.circles.push(circle.clone());
                        self.selected = Some(self.circles.len() - 1);
                        self.history_index += 1;
                    }
                    HistoryItem::UpdateDiameter(index, new_diameter) => {
                        if let Some(circle) = self.circles.get_mut(*index) {
                            circle.diameter = *new_diameter;
                        }
                        self.history_index += 1;
                    }
                }
            }
        }

        pub fn can_undo(&self) -> bool {
            self.history_index > 0
        }

        pub fn can_redo(&self) -> bool {
            self.history_index < self.history.len()
        }

        pub fn find_circle_at(&self, x: i32, y: i32) -> Option<usize> {
            // Check from top to bottom (reverse order)
            for (idx, circle) in self.circles.iter().enumerate().rev() {
                if circle.contains(x, y) {
                    return Some(idx);
                }
            }
            None
        }

        pub fn update_mouse(&mut self, x: i32, y: i32) {
            self.mouse_x = x;
            self.mouse_y = y;
            if !self.show_diameter_dialog {
                let new_selection = self.find_circle_at(x, y);
                if self.selected != new_selection {
                    self.selected = new_selection;
                }
            }
        }
    }
}

use efltk::prelude::*;

pub enum Msg {
    AddCircle(i32, i32),
    SelectCircle(Option<usize>),
    ShowDiameterDialog,
    HideDiameterDialog,
    SetDiameter(f64),
    Undo,
    Redo,
    MouseMove(i32, i32),
}

#[derive(Default)]
pub struct CircleDrawer {
    main_box: efltk::Box,
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
            Msg::AddCircle(x, y) => {
                if !model.show_diameter_dialog {
                    model.add_circle(x, y);
                }
            }
            Msg::SelectCircle(index) => {
                model.select_circle(index);
            }
            Msg::ShowDiameterDialog => {
                if !model.show_diameter_dialog {
                    model.show_diameter_dialog();
                }
            }
            Msg::HideDiameterDialog => {
                model.hide_diameter_dialog();
            }
            Msg::SetDiameter(value) => {
                model.set_diameter(value as i32);
            }
            Msg::Undo => {
                if model.can_undo() {
                    model.undo();
                }
            }
            Msg::Redo => {
                if model.can_redo() {
                    model.redo();
                }
            }
            Msg::MouseMove(x, y) => {
                model.update_mouse(x, y);
            }
        }
        true
    }

    fn update(&self, model: &Self::State) {
        // Update button states
        self.undo_btn.set_disabled(!model.can_undo());
        self.redo_btn.set_disabled(!model.can_redo());

        // Update diameter slider
        if let Some(index) = model.selected {
            let diameter = model.circles[index].diameter;
            if (diameter as f64) != self.diameter_slider.value() {
                self.diameter_slider.set_value(diameter as f64);
            }
        }

        // Redraw canvas
        self.canvas.clear();
        
        // Draw all circles
        for (idx, circle) in model.circles.iter().enumerate() {
            let is_selected = model.selected == Some(idx);
            
            // Draw unfilled circle (outline)
            self.canvas.draw_circle(
                circle.x,
                circle.y,
                circle.radius(),
                false,
                (0, 0, 0, 255) // Black outline
            );
            
            // Draw filled circle if selected
            if is_selected {
                self.canvas.draw_circle(
                    circle.x,
                    circle.y,
                    circle.radius(),
                    true,
                    (128, 128, 128, 255) // Gray fill
                );
            }
        }

        // Show/hide diameter dialog
        if model.show_diameter_dialog {
            self.diameter_popup.show();
        } else {
            self.diameter_popup.hide();
        }
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
                move |_| sender.send(Msg::Undo).unwrap()
            });

        self.redo_btn = efltk::Button::new(&btn_box)
            .with_text("Redo")
            .with_callback({
                let sender = sender.clone();
                move |_| sender.send(Msg::Redo).unwrap()
            });

        // Canvas area
        self.canvas = efltk::Canvas::new(&self.main_box)
            .with_min_size(400, 400)
            .with_weight(true, true)
            .with_mouse_down_left({
                let sender = sender.clone();
                move |wgt: efltk::Canvas| {
                    let mut x = 0i32;
                    let mut y = 0i32;
                    unsafe {
                        evas_pointer_canvas_xy_get(
                            evas_object_evas_get(wgt.as_raw()),
                            &mut x,
                            &mut y
                        );
                        let (cx, cy, _, _) = wgt.geometry();
                        sender.send(Msg::AddCircle(x - cx, y - cy)).unwrap();
                    }
                }
            })
            .with_mouse_move({
                let sender = sender.clone();
                move |wgt: efltk::Canvas| {
                    let mut x = 0i32;
                    let mut y = 0i32;
                    unsafe {
                        evas_pointer_canvas_xy_get(
                            evas_object_evas_get(wgt.as_raw()),
                            &mut x,
                            &mut y
                        );
                        let (cx, cy, _, _) = wgt.geometry();
                        sender.send(Msg::MouseMove(x - cx, y - cy)).unwrap();
                    }
                }
            })
            .with_mouse_down_right({
                let sender = sender.clone();
                move |wgt: efltk::Canvas| {
                    let mut x = 0i32;
                    let mut y = 0i32;
                    unsafe {
                        evas_pointer_canvas_xy_get(
                            evas_object_evas_get(wgt.as_raw()),
                            &mut x,
                            &mut y
                        );
                        let (cx, cy, _, _) = wgt.geometry();
                        let pos_x = x - cx;
                        let pos_y = y - cy;
                        
                        // Find circle under mouse and show diameter dialog
                        sender.send(Msg::MouseMove(pos_x, pos_y)).unwrap();
                        // The actual selection will be handled in the update
                        // For now, just show the dialog if there's a selected circle
                        if let Some(idx) = sender.send(Msg::FindCircleAt(pos_x, pos_y)) {
                            // This won't work - need different approach
                        }
                    }
                    // For now, just show dialog if there's a selected circle
                    if model.selected.is_some() {
                        sender.send(Msg::ShowDiameterDialog).unwrap();
                    }
                }
            });

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
            .with_value(24.0)
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
