
}

pub trait CanvasExt: WidgetExt {
    fn with_mouse_down_left<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.set_callback(MouseSignal::DownLeft, func);
        self
    }
    fn with_mouse_down_right<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.set_callback(MouseSignal::DownRight, func);
        self
    }
    fn with_mouse_up_left<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.set_callback(MouseSignal::UpLeft, func);
        self
    }
    fn with_mouse_move<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.set_callback(MouseSignal::Move, func);
        self
    }
}

impl CanvasExt for super::Canvas {}

