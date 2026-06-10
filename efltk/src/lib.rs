#![doc = include_str!("../README.md")]

pub mod prelude;

use {
    efltk_sys::*,
    prelude::*,
    std::{cell::RefCell, ptr::NonNull, rc::Rc},
};

pub enum ListMode {
    Compress = 0,
    Scroll,
    Limit,
    Expand,
}

pub enum ScrollerPolicy {
    Auto = 0,
    On,
    Off,
    Last,
}

pub enum Shrink {
    None = 0,
    Hide,
    Scroll,
    Menu,
    Expand,
    Last,
}

pub enum FileSelectorMode {
    List = 0,
    Grid,
    Last,
}

#[derive(Default)]
pub enum ImageOrient {
    #[default]
    None = 0,
    Rotate90 = 1,
    Rotate180 = 2,
    Rotate270 = 3,
    FlipHorizontal = 4,
    FlipVertical = 5,
    FlipTranspose = 6,
    FlipTransverse = 7,
}

#[derive(Default)]
pub enum PrefsResetMode {
    #[default]
    Defaults = 0,
    Last = 1,
}

#[derive(Default)]
pub struct EventHandler(Option<NonNull<Ecore_Event_Handler>>);

#[derive(Default)]
pub struct Timer(Option<NonNull<Ecore_Timer>>);

#[derive(Default)]
pub struct WidgetItem(Option<NonNull<Evas_Object>>);

#[derive(Default)]
pub struct Menu(Option<NonNull<Evas_Object>>);

impl Menu {
    pub fn selected(&self) -> WidgetItem {
        WidgetItem::from_raw(unsafe { elm_menu_selected_item_get(self.as_raw()) })
    }
    fn first(&self) -> WidgetItem {
        WidgetItem::from_raw(unsafe { elm_menu_first_item_get(self.as_raw()) })
    }
}

impl WidgetExt for Menu {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
}

impl SelectorExt for Menu {
    fn add(&self, label: &str) -> WidgetItem {
        self.append(label, label, |wgt| {
            wgt.call_signal(SelectorSignal::Selected)
        })
    }
    fn set_value(&self, value: u32) {
        let mut temp = self.first().as_raw();
        for _idx in 0..value {
            temp = unsafe { elm_menu_item_next_get(temp) }
        }
        unsafe { elm_menu_item_selected_set(temp, true as Eina_Bool) };
    }
    fn value(&self) -> u32 {
        unsafe { elm_menu_item_index_get(self.selected().as_raw()) as u32 }
    }
    fn lenght(&self) -> u32 {
        let mut count = 0;
        let mut temp = self.first();
        while temp.0.is_some() {
            count += 1;
            temp = WidgetItem::from_raw(unsafe { elm_menu_item_next_get(temp.as_raw()) });
        }
        count
    }
    fn find(&self, item: WidgetItem) -> u32 {
        let mut count = 0;
        let mut temp = self.first().as_raw();
        while temp != item.as_raw() {
            temp = unsafe { elm_menu_item_next_get(temp) };
            count += 1;
        }
        count
    }
    fn clear(&self) {
        let mut temp = self.first();
        while temp.0.is_some() {
            let next = WidgetItem::from_raw(unsafe { elm_menu_item_next_get(temp.as_raw()) });
            temp.del();
            temp = next;
        }
    }
}
impl MenuExt for Menu {}

#[derive(Default)]
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
    pub gmtoff: i64,
    pub zone: String,
}

impl Tm {
    pub fn to_tm(&self) -> tm {
        let zone = std::ffi::CString::new(self.zone.clone()).unwrap();
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
            tm_gmtoff: self.gmtoff,
            tm_zone: zone.as_ptr(),
        }
    }
    pub fn from_tm(value: tm) -> Self {
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
            gmtoff: value.tm_gmtoff,
            zone,
        }
    }
}

#[derive(Default)]
pub struct Ctxpopup(Option<NonNull<Evas_Object>>);

impl Ctxpopup {
    fn first(&self) -> WidgetItem {
        WidgetItem::from_raw(unsafe { elm_ctxpopup_first_item_get(self.as_raw()) })
    }
    fn last(&self) -> WidgetItem {
        WidgetItem::from_raw(unsafe { elm_ctxpopup_last_item_get(self.as_raw()) })
    }
    fn selected(&self) -> WidgetItem {
        WidgetItem::from_raw(unsafe { elm_ctxpopup_selected_item_get(self.as_raw()) })
    }
}

impl WidgetExt for Ctxpopup {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
}
impl CtxpopupExt for Ctxpopup {}
impl SelectorExt for Ctxpopup {
    fn add(&self, label: &str) -> WidgetItem {
        self.append(label, label, |_| {})
    }
    fn set_value(&self, value: u32) {
        let mut temp = self.first().as_raw();
        for _ in 0..value {
            temp = unsafe { elm_ctxpopup_item_next_get(temp) };
        }
        unsafe { elm_ctxpopup_item_selected_set(temp, true as Eina_Bool) }
    }
    fn value(&self) -> u32 {
        self.find(self.selected())
    }
    fn find(&self, item: WidgetItem) -> u32 {
        let mut count = 0;
        let mut temp = self.first().as_raw();
        while temp != item.as_raw() {
            temp = unsafe { elm_ctxpopup_item_next_get(temp) };
            count += 1;
        }
        count
    }
    fn clear(&self) {
        let mut temp = self.first();
        while temp.0.is_some() {
            let next = WidgetItem::from_raw(unsafe { elm_ctxpopup_item_next_get(temp.as_raw()) });
            temp.del();
            temp = next;
        }
    }
    fn lenght(&self) -> u32 {
        self.find(self.last())
    }
}

#[derive(Default)]
pub struct Entry(Option<NonNull<Evas_Object>>);

impl WidgetExt for Entry {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
}
impl EntryExt for Entry {}

#[derive(Default)]
pub struct Frame(Option<NonNull<Evas_Object>>);

impl WidgetExt for Frame {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
}
impl ContainerExt for Frame {}
impl FrameExt for Frame {}

#[derive(Default)]
pub struct Icon(Option<NonNull<Evas_Object>>);

impl WidgetExt for Icon {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
}
impl IconExt for Icon {}

#[derive(Default)]
pub struct Label(Option<NonNull<Evas_Object>>);

impl WidgetExt for Label {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
}
impl LabelExt for Label {}

#[derive(Default)]
pub struct Separator(Option<NonNull<Evas_Object>>);

impl WidgetExt for Separator {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
}
impl SeparatorExt for Separator {}

#[derive(Default)]
pub struct List(Option<NonNull<Evas_Object>>);
impl List {
    pub fn selected(&self) -> WidgetItem {
        WidgetItem::from_raw(unsafe { elm_list_selected_item_get(self.as_raw()) })
    }
    fn first(&self) -> WidgetItem {
        WidgetItem::from_raw(unsafe { elm_list_first_item_get(self.as_raw()) })
    }
}

impl WidgetExt for List {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
}
impl SelectorExt for List {
    fn add(&self, label: &str) -> WidgetItem {
        self.add_item(label, label, |_| {})
    }
    fn set_value(&self, value: u32) {
        let mut temp = self.first().as_raw();
        for _ in 0..value {
            temp = unsafe { elm_list_item_next(temp) };
        }
        unsafe { elm_list_item_selected_set(temp, true as Eina_Bool) }
    }
    fn value(&self) -> u32 {
        self.find(self.selected())
    }
    fn find(&self, item: WidgetItem) -> u32 {
        let mut count = 0;
        let mut temp = self.first().as_raw();
        while temp != item.as_raw() {
            count += 1;
            temp = unsafe { elm_list_item_next(temp) };
        }
        count
    }
    fn clear(&self) {
        unsafe { elm_list_clear(self.as_raw()) };
    }
    fn lenght(&self) -> u32 {
        let mut count = 0;
        let mut temp = self.first();
        while temp.0.is_some() {
            count += 1;
            temp = WidgetItem::from_raw(unsafe { elm_list_item_next(temp.as_raw()) });
        }
        count
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
        if self.lst.borrow_mut().len() > value {
            self.to_top(&self.lst.borrow()[value]);
        };
    }
    pub fn promote(&self) {
        self.to_top(&self.bottom())
    }
    fn bottom(&self) -> WidgetItem {
        WidgetItem::from_raw(unsafe { elm_naviframe_bottom_item_get(self.as_raw()) })
    }
    fn to_top(&self, item: &WidgetItem) {
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
}
impl ContainerExt for Naviframe {
    fn add(&self, child: &impl WidgetExt) {
        self.lst.borrow_mut().push(self.push(child));
        child.show();
    }
}
impl NaviframeExt for Naviframe {}

#[derive(Default)]
pub struct Panes(Option<NonNull<Evas_Object>>);

impl WidgetExt for Panes {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
}
impl ContainerExt for Panes {
    fn add(&self, child: &impl WidgetExt) {
        match self.content("left") {
            None => self.set_content(child, "left"),
            _ => self.set_content(child, "right"),
        }
        child.show();
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
}
impl PopupExt for Popup {}
impl ContainerExt for Popup {}

#[derive(Default)]
pub struct ProgressBar(Option<NonNull<Evas_Object>>);

impl WidgetExt for ProgressBar {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
}
impl ProgressBarExt for ProgressBar {}

#[derive(Default)]
pub struct Radio(Option<NonNull<Evas_Object>>);

impl WidgetExt for Radio {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
}
impl RadioExt for Radio {}

#[derive(Default)]
pub struct Prefs(Option<NonNull<Evas_Object>>);

impl WidgetExt for Prefs {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
}
impl PrefsExt for Prefs {}

#[derive(Default)]
pub struct Video(Option<NonNull<Evas_Object>>);

impl WidgetExt for Video {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
}
impl LayoutExt for Video {}
impl VideoExt for Video {}

#[derive(Default)]
pub struct Player(Option<NonNull<Evas_Object>>);

impl WidgetExt for Player {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
}
impl ContainerExt for Player {
    fn add(&self, child: &impl WidgetExt) {
        self.set_content(child, "video");
        child.show();
    }
}
impl LayoutExt for Player {}
impl PlayerExt for Player {}

#[derive(Default)]
pub struct SegmentControl(Option<NonNull<Evas_Object>>);

impl SegmentControl {
    fn selected(&self) -> WidgetItem {
        WidgetItem::from_raw(unsafe { elm_segment_control_item_selected_get(self.as_raw()) })
    }
}

impl WidgetExt for SegmentControl {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
}
// impl EvasObject for SegmentControl {}
impl SegmentControlExt for SegmentControl {}
impl SelectorExt for SegmentControl {
    fn add(&self, label: &str) -> WidgetItem {
        WidgetItem::from_raw(unsafe {
            elm_segment_control_item_add(
                self.as_raw(),
                Icon::new(self).with_standard(label).as_raw(),
                std::ffi::CString::new(label).unwrap().as_ptr(),
            )
        })
    }
    fn find(&self, item: WidgetItem) -> u32 {
        unsafe { elm_segment_control_item_index_get(item.as_raw()) as u32 }
    }
    fn value(&self) -> u32 {
        unsafe { elm_segment_control_item_index_get(self.selected().as_raw()) as u32 }
    }
    fn lenght(&self) -> u32 {
        unsafe { elm_segment_control_item_count_get(self.as_raw()) as u32 }
    }
    fn set_value(&self, value: u32) {
        unsafe {
            elm_segment_control_item_selected_set(
                elm_segment_control_item_get(self.as_raw(), value as i32),
                true as Eina_Bool,
            )
        };
    }
    fn clear(&self) {
        unsafe { elm_diskselector_clear(self.as_raw()) };
    }
}

#[derive(Default)]
pub struct Slider(Option<NonNull<Evas_Object>>);

impl WidgetExt for Slider {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
}
impl SliderExt for Slider {}
impl RangerExt for Slider {
    fn value(&self) -> f64 {
        unsafe { elm_slider_value_get(self.as_raw()) }
    }
    fn set_range(&self, min: f64, max: f64) {
        unsafe { elm_slider_min_max_set(self.as_raw(), min, max) };
    }
    fn set_value(&self, value: f64) {
        unsafe { elm_slider_value_set(self.as_raw(), value) };
    }
    fn set_step(&self, value: f64) {
        unsafe { elm_slider_step_set(self.as_raw(), value) };
    }
    fn set_format(&self, value: &str) {
        let ctext = std::ffi::CString::new(value).unwrap();
        unsafe { elm_slider_unit_format_set(self.as_raw(), ctext.as_ptr()) };
    }
}

#[derive(Default)]
pub struct Spinner(Option<NonNull<Evas_Object>>);

impl WidgetExt for Spinner {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
}
impl SpinnerExt for Spinner {}
impl RangerExt for Spinner {
    fn set_format(&self, format: &str) {
        let cformat = std::ffi::CString::new(format).unwrap();
        unsafe { elm_spinner_label_format_set(self.as_raw(), cformat.as_ptr()) };
    }
    fn set_range(&self, min: f64, max: f64) {
        unsafe { elm_spinner_min_max_set(self.as_raw(), min, max) };
    }
    fn set_step(&self, step: f64) {
        unsafe { elm_spinner_step_set(self.as_raw(), step) };
    }
    fn set_value(&self, value: f64) {
        unsafe { elm_spinner_value_set(self.as_raw(), value) };
    }
    fn value(&self) -> f64 {
        unsafe { elm_spinner_value_get(self.as_raw()) }
    }
}

#[derive(Default)]
pub struct Window(Option<NonNull<Evas_Object>>);

impl WidgetExt for Window {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
}
impl ContainerExt for Window {
    fn add(&self, child: &impl WidgetExt) {
        unsafe { elm_win_resize_object_add(self.as_raw(), child.as_raw()) };
        child.show();
    }
}
impl WindowExt for Window {}

#[derive(Default)]
pub struct Box(Option<NonNull<Evas_Object>>);

impl WidgetExt for Box {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
}
impl ContainerExt for Box {
    fn add(&self, child: &impl WidgetExt) {
        self.add_item(child);
        child.show();
    }
}
impl BoxExt for Box {}

#[derive(Default)]
pub struct Bubble(Option<NonNull<Evas_Object>>);

impl WidgetExt for Bubble {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
}
impl ContainerExt for Bubble {}
impl BubbleExt for Bubble {}

#[derive(Default)]
pub struct Button(Option<NonNull<Evas_Object>>);

impl WidgetExt for Button {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
}
impl ButtonExt for Button {}

#[derive(Default)]
pub struct Check(Option<NonNull<Evas_Object>>);

impl WidgetExt for Check {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
}
impl CheckExt for Check {}

#[derive(Default)]
pub struct FileEntry(Option<NonNull<Evas_Object>>);

impl WidgetExt for FileEntry {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
}
impl FileEntryExt for FileEntry {}
