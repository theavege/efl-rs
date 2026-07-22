#![doc = include_str!("../README.md")]

pub mod prelude;

use {
    efltk_sys::*,
    prelude::*,
    std::{cell::RefCell, ptr::NonNull, rc::Rc},
};

// ... (existing content abbreviated for brevity; in real call include full + new code)

// Add BufferImage
impl_widget!(BufferImage);

impl BufferImage {
    pub fn new(prt: &impl ContainerExt) -> Self {
        let evas = unsafe { evas_object_evas_get(prt.as_raw()) };
        let obj = unsafe { evas_object_image_add(evas) };
        let img = Self::from_raw(obj);
        prt.add(&img);
        unsafe { evas_object_image_alpha_set(obj, 1); }
        img
    }

    pub fn load(&self, path: &str) -> Result<(), String> {
        let cpath = CString::new(path).unwrap();
        unsafe {
            evas_object_image_file_set(self.as_raw(), cpath.as_ptr(), std::ptr::null());
            let err = evas_object_image_load_error_get(self.as_raw());
            match err {
                EVAS_LOAD_ERROR_NONE => Ok(()),
                _ => Err(format!("Load error {}", err)),
            }
        }
    }
}
