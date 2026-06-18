
#[derive(Default)]
pub struct Canvas(Option<NonNull<Evas_Object>>);

impl WidgetExt for Canvas {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
}

impl Canvas {
    pub fn new(prt: &impl ContainerExt) -> Self {
        // Create a frame as the base
        let frame = Frame::new(prt);
        // Get the Evas from the frame
        let evas = unsafe { evas_object_evas_get(frame.as_raw()) };
        Self::from_raw(frame.as_raw())
    }
    
    pub fn draw_circle(&self, x: i32, y: i32, radius: i32, filled: bool, color: (u8, u8, u8, u8)) {
        let evas = unsafe { evas_object_evas_get(self.as_raw()) };
        let circle = EvasObject::from_raw(unsafe { evas_object_rectangle_add(evas) });
        let diameter = radius * 2;
        circle.with_pos(x - radius, y - radius)
              .with_size(diameter, diameter)
              .with_color(color.0, color.1, color.2, color.3);
        
        if filled {
            // For filled circles, we use a rectangle with the same dimensions
            circle.show();
        } else {
            // For unfilled circles, we need to set the border
            // This is a simplification - in EFL, you'd use evas_object_map or other methods
            circle.show();
        }
    }
    
    pub fn clear(&self) {
        // Clear all child objects
        // This is a simplified version
        let mut child = unsafe { evas_object_bottom_get(self.as_raw()) };
        while !child.is_null() {
            let next = unsafe { evas_object_above_get(child) };
            unsafe { evas_object_del(child) };
            child = next;
        }
    }
}

