#![doc = include_str!("../README.md")]

pub mod error;
pub mod prelude;
#[cfg(test)]
mod tests;

pub use error::{CStringExt, EflError, EflResult};

use {
    efltk_sys::*,
    prelude::*,
    std::{cell::RefCell, ptr::NonNull, rc::Rc},
};

macro_rules! impl_widget {
    ($name:ident) => {
        #[derive(Default)]
        pub struct $name(Option<std::ptr::NonNull<Evas_Object>>);

        impl WidgetExt for $name {
            fn as_raw(&self) -> *mut Evas_Object {
                self.0
                    .expect(concat!("Empty ", stringify!($name), "!"))
                    .as_ptr()
            }

            fn from_raw(obj: *mut Evas_Object) -> Self {
                Self(std::ptr::NonNull::new(obj))
            }

            fn is_set(&self) -> bool {
                self.0.is_some()
            }
        }

        impl From<*mut Evas_Object> for $name {
            fn from(obj: *mut Evas_Object) -> Self {
                Self(NonNull::new(obj))
            }
        }
    };
}

#[derive(Default)]
pub struct Timer(Option<NonNull<Ecore_Timer>>);

impl From<*mut Ecore_Timer> for Timer {
    fn from(obj: *mut Ecore_Timer) -> Self {
        Self(NonNull::new(obj))
    }
}

#[derive(Default)]
pub struct WidgetItem(Option<NonNull<Evas_Object>>);

impl_widget!(Menu);

impl Menu {
    pub fn selected(&self) -> WidgetItem {
        if !self.is_set() {
            return WidgetItem::default();
        }
        WidgetItem::from_raw(unsafe { elm_menu_selected_item_get(self.as_raw()) })
    }
    fn first(&self) -> WidgetItem {
        if !self.is_set() {
            return WidgetItem::default();
        }
        WidgetItem::from_raw(unsafe { elm_menu_first_item_get(self.as_raw()) })
    }
}

impl InputExt<i32> for Menu {
    fn value(&self) -> i32 {
        let selected = self.selected();
        if selected.0.is_none() {
            return -1;
        }
        unsafe { elm_menu_item_index_get(selected.as_raw()) as i32 }
    }
    fn set_value(&self, value: i32) {
        if !self.is_set() {
            return;
        }
        if (0..self.length()).contains(&(value as u32)) {
            let mut temp = self.first().as_raw();
            for _ in 0..value {
                if temp.is_null() {
                    return;
                }
                temp = unsafe { elm_menu_item_next_get(temp) };
            }
            if !temp.is_null() {
                unsafe { elm_menu_item_selected_set(temp, true as Eina_Bool) }
            }
        }
    }
}

impl SelectorExt for Menu {
    fn add(&self, label: &str) -> WidgetItem {
        self.append(label, label, |wgt| wgt.call_signal(Signal::Selected))
    }
    fn length(&self) -> u32 {
        if !self.is_set() {
            return 0;
        }
        let mut count = 0;
        let mut temp = self.first();
        while temp.0.is_some() {
            count += 1;
            temp = WidgetItem::from_raw(unsafe { elm_menu_item_next_get(temp.as_raw()) });
        }
        count
    }
    fn clear(&self) {
        if !self.is_set() {
            return;
        }
        let mut temp = self.first();
        while temp.0.is_some() {
            let next = WidgetItem::from_raw(unsafe { elm_menu_item_next_get(temp.as_raw()) });
            temp.del();
            temp = next;
        }
    }
}

impl MenuExt for Menu {}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Tm {
    pub sec: i32,
    pub min: i32,
    pub hour: i32,
    pub mday: i32,
    pub mon: i32,
    pub year: i32,
    pub wday: i32,
    pub yday: i32,
    pub isdst: i32,
    #[cfg(target_os = "linux")]
    pub gmtoff: i64,
    #[cfg(target_os = "linux")]
    pub zone: String,
}

impl Tm {
    pub fn to_tm(&self) -> tm {
        tm {
            tm_sec: self.sec,
            tm_min: self.min,
            tm_hour: self.hour,
            tm_mday: self.mday,
            tm_mon: self.mon,
            tm_year: self.year,
            tm_wday: self.wday,
            tm_yday: self.yday,
            tm_isdst: self.isdst,
            #[cfg(target_os = "linux")]
            tm_gmtoff: self.gmtoff,
            #[cfg(target_os = "linux")]
            tm_zone: std::ptr::null_mut(),
        }
    }
    pub fn from_tm(value: tm) -> Self {
        #[cfg(target_os = "linux")]
        let zone = unsafe {
            if !value.tm_zone.is_null() {
                std::ffi::CStr::from_ptr(value.tm_zone)
                    .to_string_lossy()
                    .into_owned()
            } else {
                String::new()
            }
        };
        Self {
            sec: value.tm_sec,
            min: value.tm_min,
            hour: value.tm_hour,
            mday: value.tm_mday,
            mon: value.tm_mon,
            year: value.tm_year,
            wday: value.tm_wday,
            yday: value.tm_yday,
            isdst: value.tm_isdst,
            #[cfg(target_os = "linux")]
            gmtoff: value.tm_gmtoff,
            #[cfg(target_os = "linux")]
            zone,
        }
    }
}

impl_widget!(Entry);

impl EntryExt for Entry {}
impl TextExt for Entry {}
impl InputExt<String> for Entry {
    fn value(&self) -> String {
        self.text()
    }
    fn set_value(&self, value: String) {
        self.set_text(&value);
    }
}

impl_widget!(Frame);

impl ContainerExt for Frame {}
impl TextExt for Frame {}
impl InputExt<bool> for Frame {
    fn value(&self) -> bool {
        unsafe { elm_frame_collapse_get(self.as_raw()) != 0 }
    }
    fn set_value(&self, value: bool) {
        unsafe { elm_frame_collapse_set(self.as_raw(), value as Eina_Bool) };
    }
}
impl FrameExt for Frame {}

impl_widget!(Icon);
impl IconExt for Icon {}

impl_widget!(Label);

impl TextExt for Label {}
impl LabelExt for Label {}

impl_widget!(Separator);

impl OrientExt for Separator {
    fn with_horizontal(self, value: bool) -> Self {
        unsafe { elm_separator_horizontal_set(self.as_raw(), value as Eina_Bool) };
        self.set_weight(value, !value);
        self
    }
}
impl SeparatorExt for Separator {}

impl_widget!(List);

impl List {
    pub fn selected(&self) -> WidgetItem {
        if !self.is_set() {
            return WidgetItem::default();
        }
        WidgetItem::from_raw(unsafe { elm_list_selected_item_get(self.as_raw()) })
    }
    fn first(&self) -> WidgetItem {
        if !self.is_set() {
            return WidgetItem::default();
        }
        WidgetItem::from_raw(unsafe { elm_list_first_item_get(self.as_raw()) })
    }
}

impl InputExt<i32> for List {
    fn value(&self) -> i32 {
        if self.length() == 0 {
            return -1;
        }
        let selected = self.selected();
        if selected.0.is_none() {
            return -1;
        }
        let selected_ptr = selected.as_raw();
        let mut count = 0;
        let mut temp = self.first().as_raw();
        while !temp.is_null() && temp != selected_ptr {
            count += 1;
            temp = unsafe { elm_list_item_next(temp) };
        }
        if temp.is_null() { -1 } else { count }
    }
    fn set_value(&self, value: i32) {
        if !self.is_set() {
            return;
        }
        if (0..self.length()).contains(&(value as u32)) {
            let mut temp = self.first().as_raw();
            for _ in 0..value {
                if temp.is_null() {
                    return;
                }
                temp = unsafe { elm_list_item_next(temp) };
            }
            if !temp.is_null() {
                unsafe { elm_list_item_selected_set(temp, true as Eina_Bool) }
            }
        }
    }
}
impl SelectorExt for List {
    fn add(&self, label: &str) -> WidgetItem {
        self.add_item(label, label, |_| {})
    }
    fn length(&self) -> u32 {
        if !self.is_set() {
            return 0;
        }
        let mut count = 0;
        let mut temp = self.first();
        while temp.0.is_some() {
            count += 1;
            temp = WidgetItem::from_raw(unsafe { elm_list_item_next(temp.as_raw()) });
        }
        count
    }
    fn clear(&self) {
        if !self.is_set() {
            return;
        }
        unsafe { elm_list_clear(self.as_raw()) };
    }
}
impl ListExt for List {}

#[derive(Default)]
pub struct Naviframe {
    obj: Option<NonNull<Evas_Object>>,
    lst: Rc<RefCell<Vec<WidgetItem>>>,
}

impl Naviframe {
    pub fn set_top(&self, value: usize) {
        if self.lst.borrow().len() > value {
            self.to_top(&self.lst.borrow()[value]);
        };
    }
    pub fn promote(&self) {
        if !self.is_set() {
            return;
        }
        self.to_top(&self.bottom())
    }
    fn bottom(&self) -> WidgetItem {
        WidgetItem::from_raw(unsafe { elm_naviframe_bottom_item_get(self.as_raw()) })
    }
    fn to_top(&self, item: &WidgetItem) {
        if item.0.is_none() {
            return;
        }
        unsafe { elm_naviframe_item_promote(item.as_raw()) };
    }
}

impl WidgetExt for Naviframe {
    fn as_raw(&self) -> *mut Evas_Object {
        self.obj.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self {
            obj: NonNull::new(obj),
            lst: Rc::default(),
        }
    }
    fn is_set(&self) -> bool {
        self.obj.is_some()
    }
}
impl ContainerExt for Naviframe {
    fn add(&self, child: &impl WidgetExt) {
        self.lst.borrow_mut().push(self.push(child));
        child.show();
    }
}
impl NaviframeExt for Naviframe {}

impl_widget!(Panes);

impl ContainerExt for Panes {
    fn add(&self, child: &impl WidgetExt) {
        match self.content("left") {
            None => self.set_content(child, "left"),
            _ => self.set_content(child, "right"),
        }
        child.show();
    }
}
impl OrientExt for Panes {
    fn with_horizontal(self, value: bool) -> Self {
        unsafe { elm_panes_horizontal_set(self.as_raw(), value as Eina_Bool) };
        self.set_weight(value, !value);
        self
    }
}
impl PanesExt for Panes {}

#[derive(Default, Clone)]
pub struct Popup(Option<NonNull<Evas_Object>>);

impl WidgetExt for Popup {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
    fn is_set(&self) -> bool {
        self.0.is_some()
    }
}
impl PopupExt for Popup {}
impl ContainerExt for Popup {}

impl_widget!(ProgressBar);

impl ProgressBarExt for ProgressBar {}

impl_widget!(Radio);

impl InputExt<i32> for Radio {
    fn value(&self) -> i32 {
        unsafe { elm_radio_value_get(self.as_raw()) as i32 }
    }
    fn set_value(&self, value: i32) {
        unsafe { elm_radio_value_set(self.as_raw(), value) };
    }
}
impl TextExt for Radio {}
impl RadioExt for Radio {}

impl_widget!(SegmentControl);

impl SegmentControl {
    fn selected(&self) -> WidgetItem {
        if !self.is_set() {
            return WidgetItem::default();
        }
        WidgetItem::from_raw(unsafe { elm_segment_control_item_selected_get(self.as_raw()) })
    }
}

impl SegmentControlExt for SegmentControl {}
impl InputExt<i32> for SegmentControl {
    fn value(&self) -> i32 {
        let selected = self.selected();
        if selected.0.is_none() {
            return -1;
        }
        unsafe { elm_segment_control_item_index_get(selected.as_raw()) as i32 }
    }
    fn set_value(&self, value: i32) {
        if !self.is_set() {
            return;
        }
        unsafe {
            elm_segment_control_item_selected_set(
                elm_segment_control_item_get(self.as_raw(), value),
                true as Eina_Bool,
            )
        };
    }
}
impl SelectorExt for SegmentControl {
    fn add(&self, label: &str) -> WidgetItem {
        let c_label = label.expect_cstring("SegmentControl::add");
        WidgetItem::from_raw(unsafe {
            elm_segment_control_item_add(
                self.as_raw(),
                Icon::new(self).with_standard(label).as_raw(),
                c_label.as_ptr(),
            )
        })
    }
    fn length(&self) -> u32 {
        if !self.is_set() {
            return 0;
        }
        unsafe { elm_segment_control_item_count_get(self.as_raw()) as u32 }
    }

    fn clear(&self) {
        if !self.is_set() {
            return;
        }
        while self.length() > 0 {
            let item = unsafe { elm_segment_control_item_get(self.as_raw(), 0) };
            if item.is_null() {
                break;
            }
            unsafe { elm_object_item_del(item) };
        }
    }
}

impl_widget!(Slider);
impl SliderExt for Slider {}
impl InputExt<f64> for Slider {
    fn value(&self) -> f64 {
        unsafe { elm_slider_value_get(self.as_raw()) }
    }
    fn set_value(&self, value: f64) {
        unsafe { elm_slider_value_set(self.as_raw(), value) };
    }
}
impl OrientExt for Slider {
    fn with_horizontal(self, value: bool) -> Self {
        unsafe { elm_slider_horizontal_set(self.as_raw(), value as Eina_Bool) };
        self.set_weight(value, !value);
        self
    }
}
impl RangerExt for Slider {
    fn set_range(&self, min: f64, max: f64) {
        unsafe { elm_slider_min_max_set(self.as_raw(), min, max) };
    }
    fn set_step(&self, value: f64) {
        unsafe { elm_slider_step_set(self.as_raw(), value) };
    }
    fn with_format(self, value: &str) -> Self {
        let ctext = value.expect_cstring("Slider::with_format");
        unsafe { elm_slider_unit_format_set(self.as_raw(), ctext.as_ptr()) };
        self
    }
}

impl_widget!(Spinner);
impl SpinnerExt for Spinner {}
impl InputExt<f64> for Spinner {
    fn set_value(&self, value: f64) {
        unsafe { elm_spinner_value_set(self.as_raw(), value) };
    }
    fn value(&self) -> f64 {
        unsafe { elm_spinner_value_get(self.as_raw()) }
    }
}
impl RangerExt for Spinner {
    fn with_format(self, format: &str) -> Self {
        let cformat = format.expect_cstring("Spinner::with_format");
        unsafe { elm_spinner_label_format_set(self.as_raw(), cformat.as_ptr()) };
        self
    }
    fn set_range(&self, min: f64, max: f64) {
        unsafe { elm_spinner_min_max_set(self.as_raw(), min, max) };
    }
    fn set_step(&self, step: f64) {
        unsafe { elm_spinner_step_set(self.as_raw(), step) };
    }
}

impl_widget!(Window);

impl ContainerExt for Window {
    fn add(&self, child: &impl WidgetExt) {
        unsafe { elm_win_resize_object_add(self.as_raw(), child.as_raw()) };
        child.show();
    }
}
impl WindowExt for Window {}

impl_widget!(Box);
impl ContainerExt for Box {
    fn add(&self, child: &impl WidgetExt) {
        unsafe {
            elm_box_pack_end(self.as_raw(), child.as_raw());
            elm_box_recalculate(self.as_raw());
        };
        child.show();
    }
}
impl OrientExt for Box {
    fn with_horizontal(self, value: bool) -> Self {
        unsafe { elm_box_horizontal_set(self.as_raw(), value as Eina_Bool) };
        self.set_weight(value, !value);
        self
    }
}
impl BoxExt for Box {}

impl_widget!(Button);

impl TextExt for Button {}
impl InputExt<bool> for Button {
    fn value(&self) -> bool {
        self.disabled()
    }
    fn set_value(&self, value: bool) {
        self.set_disabled(value);
    }
}
impl ButtonExt for Button {}

impl_widget!(Check);

impl InputExt<bool> for Check {
    fn set_value(&self, value: bool) {
        unsafe { elm_check_state_set(self.as_raw(), value as Eina_Bool) };
    }
    fn value(&self) -> bool {
        unsafe { elm_check_state_get(self.as_raw()) != 0 }
    }
}
impl TextExt for Check {}
impl CheckExt for Check {}

impl_widget!(Calendar);
impl CalendarExt for Calendar {}

impl_widget!(Clock);
impl ClockExt for Clock {}

impl_widget!(FileSelector);
impl FileSelExt for FileSelector {}

impl_widget!(FileEntry);
impl FileEntryExt for FileEntry {}

impl_widget!(ColorSelector);
impl ColorSelExt for ColorSelector {}
impl InputExt<(i32, i32, i32, i32)> for ColorSelector {
    fn value(&self) -> (i32, i32, i32, i32) {
        let (mut r, mut g, mut b, mut a) = (0, 0, 0, 0);
        if self.is_set() {
            unsafe { elm_colorselector_color_get(self.as_raw(), &mut r, &mut g, &mut b, &mut a) };
        }
        (r, g, b, a)
    }
    fn set_value(&self, value: (i32, i32, i32, i32)) {
        if self.is_set() {
            unsafe {
                elm_colorselector_color_set(self.as_raw(), value.0, value.1, value.2, value.3)
            };
        }
    }
}
