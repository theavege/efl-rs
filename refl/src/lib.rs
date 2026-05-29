pub mod prelude;

use {
    prelude::*,
    refl_sys::*,
    std::{cell::RefCell, rc::Rc},
};

pub enum ActionSliderPos {
    None = 0,
    Left,
    Center,
    Right,
}

pub enum ListMode {
    Compress = 0,
    Scroll,
    Limit,
    Expand,
}

pub enum PanelOrient {
    Top = 0,
    Bottom,
    Left,
    Right,
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

pub enum WinType {
    Basic = 0,
    Dialog,
    Desktop,
    Dock,
}

pub struct EventHandler(*mut Ecore_Event_Handler);

impl EcoreEventExt for EventHandler {
    fn as_raw(&self) -> *mut Ecore_Event_Handler {
        self.0
    }
    fn from_raw(obj: *mut Ecore_Event_Handler) -> Self {
        Self(obj)
    }
}
impl EventHandlerExt for EventHandler {}

#[derive(Default)]
pub struct WidgetItem(Option<*mut Evas_Object>);

impl EvasObjectItemExt for WidgetItem {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}

impl AsMut<Evas_Object> for WidgetItem {
    fn as_mut(&mut self) -> &mut Evas_Object {
        unsafe { &mut *self.0.unwrap() }
    }
}

#[derive(Default)]
pub struct Menu(Option<*mut Evas_Object>);

impl EvasObject for Menu {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}

impl ElmObject for Menu {}

impl SelectorExt for Menu {
    fn selected(&self) -> WidgetItem {
        WidgetItem::from_raw(unsafe { elm_menu_selected_item_get(self.as_raw()) })
    }
    fn add<F: FnMut(Self) + 'static>(&self, label: &str, func: F) -> WidgetItem {
        self.append(label, label, func)
    }
    fn first(&self) -> WidgetItem {
        self.first_item()
    }
    fn last(&self) -> WidgetItem {
        self.last_item()
    }
    fn set_value(&self, value: u32) {
        self.set_index(value)
    }
    fn value(&self) -> u32 {
        self.index()
    }
    fn clear(&self) {
        self.clear_items();
    }
}
impl OnChanged for Menu {}
impl OnClicked for Menu {}
impl OnDismissed for Menu {}
impl MenuExt for Menu {}

#[derive(Default)]
pub struct Timer(*mut Ecore_Timer);

impl EcoreTimerExt for Timer {
    fn as_raw(&self) -> *mut Ecore_Timer {
        self.0
    }
    fn from_raw(obj: *mut Ecore_Timer) -> Self {
        Self(obj)
    }
}
impl TimerExt for Timer {}

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
pub struct Calendar(Option<*mut Evas_Object>);

impl EvasObject for Calendar {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Calendar {}
impl OnChanged for Calendar {}
impl CalendarExt for Calendar {}

#[derive(Default)]
pub struct Clock(Option<*mut Evas_Object>);

impl Clock {
    pub fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_clock_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
    pub fn time(&self) -> (i32, i32, i32) {
        let mut hrs: i32 = 0;
        let mut min: i32 = 0;
        let mut sec: i32 = 0;
        unsafe { elm_clock_time_get(self.as_raw(), &mut hrs, &mut min, &mut sec) };
        (hrs, min, sec)
    }
    pub fn set_time(&self, hrs: i32, min: i32, sec: i32) {
        unsafe { elm_clock_time_set(self.as_raw(), hrs, min, sec) };
    }
}

impl EvasObject for Clock {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Clock {}

#[derive(Default)]
pub struct Ctxpopup(Option<*mut Evas_Object>);

impl EvasObject for Ctxpopup {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Ctxpopup {}
impl OnDismissed for Ctxpopup {}
impl CtxpopupExt for Ctxpopup {}

#[derive(Default)]
pub struct Entry(Option<*mut Evas_Object>);
impl Entry {
    pub fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_entry_add(prt.as_raw()) });
        elm.conf();
        elm.set_single_line(true);
        elm.set_scrollable(true);
        elm.set_menu(true);
        prt.add(&elm);
        elm
    }
}
impl EvasObject for Entry {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Entry {}
impl ContainerExt for Entry {}
impl OnClicked for Entry {}
impl OnChanged for Entry {}
impl EntryExt for Entry {}

#[derive(Default)]
pub struct FlipSelector(Option<*mut Evas_Object>);

impl EvasObject for FlipSelector {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for FlipSelector {}
impl OnChanged for FlipSelector {}
impl SelectorExt for FlipSelector {
    fn selected(&self) -> WidgetItem {
        WidgetItem::from_raw(unsafe { elm_flipselector_selected_item_get(self.as_raw()) })
    }
    fn add<F: FnMut(Self) + 'static>(&self, label: &str, func: F) -> WidgetItem {
        self.append(label, func)
    }
    fn first(&self) -> WidgetItem {
        self.first_item()
    }
    fn last(&self) -> WidgetItem {
        self.last_item()
    }
    fn value(&self) -> u32 {
        self.index()
    }
    fn set_value(&self, value: u32) {
        self.set_index(value)
    }
    fn clear(&self) {
        self.clear_items();
    }
}
impl FlipSelExt for FlipSelector {}

#[derive(Default)]
pub struct Frame(Option<*mut Evas_Object>);
impl Frame {
    pub fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_frame_add(parent.as_raw()) });
        elm.conf();
        elm.set_autocollapse(true);
        parent.add(&elm);
        elm
    }
    pub fn with_collapse(self, value: bool) -> Self {
        self.set_collapse(value);
        self
    }
    pub fn with_autocollapse(self, value: bool) -> Self {
        self.set_autocollapse(value);
        self
    }
    pub fn set_autocollapse(&self, value: bool) {
        unsafe { elm_frame_autocollapse_set(self.as_raw(), value as Eina_Bool) };
    }
    pub fn set_collapse(&self, value: bool) {
        unsafe { elm_frame_collapse_set(self.as_raw(), value as Eina_Bool) };
    }
    pub fn collapse(&self) -> bool {
        unsafe { elm_frame_collapse_get(self.as_raw()) != 0 }
    }
}
impl EvasObject for Frame {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Frame {}
impl ContainerExt for Frame {
    fn add(&self, child: &impl ElmObject) {
        self.set_content(child, "default");
        child.show();
    }
}
impl OnClicked for Frame {}

#[derive(Default)]
pub struct Icon(Option<*mut Evas_Object>);

impl EvasObject for Icon {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Icon {}
impl IconExt for Icon {}

#[derive(Default)]
pub struct Label(Option<*mut Evas_Object>);

impl Label {
    pub fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_label_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
}

impl EvasObject for Label {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Label {}

#[derive(Default)]
pub struct Layout(Option<*mut Evas_Object>);

impl Layout {
    pub fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_layout_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
}

impl EvasObject for Layout {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Layout {}
impl ContainerExt for Layout {
    fn add(&self, child: &impl ElmObject) {
        self.set_content(child, "default");
        child.show();
    }
}
impl LayoutExt for Layout {}

#[derive(Default)]
pub struct Separator(Option<*mut Evas_Object>);

impl EvasObject for Separator {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Separator {}
impl SeparatorExt for Separator {}

#[derive(Default)]
pub struct List(Option<*mut Evas_Object>);
impl List {
    pub fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_list_add(prt.as_raw()) });
        elm.conf();
        elm.set_mode(ListMode::Expand);
        elm.go();
        prt.add(&elm);
        elm
    }
}

impl EvasObject for List {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl SelectorExt for List {
    fn add<F: FnMut(Self) + 'static>(&self, label: &str, func: F) -> WidgetItem {
        self.append(label, label, func)
    }
    fn selected(&self) -> WidgetItem {
        WidgetItem::from_raw(unsafe { elm_list_selected_item_get(self.as_raw()) })
    }
    fn first(&self) -> WidgetItem {
        WidgetItem::from_raw(unsafe { elm_list_first_item_get(self.as_raw()) })
    }
    fn last(&self) -> WidgetItem {
        WidgetItem::from_raw(unsafe { elm_list_last_item_get(self.as_raw()) })
    }
    fn set_value(&self, value: u32) {
        self.set_index(value)
    }
    fn value(&self) -> u32 {
        self.index()
    }
    fn clear(&self) {
        unsafe { elm_list_clear(self.as_raw()) };
    }
}
impl ElmObject for List {}
impl ListExt for List {}

#[derive(Default)]
pub struct Naviframe {
    obj: Option<*mut Evas_Object>,
    lst: Rc<RefCell<Vec<WidgetItem>>>,
}

impl Naviframe {
    pub fn set_top(&self, value: usize) {
        if self.lst.borrow_mut().len() > value {
            self.to_top(&self.lst.borrow()[value]);
        };
    }
}

impl EvasObject for Naviframe {
    fn as_raw(&self) -> *mut Evas_Object {
        self.obj.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self {
            obj: Some(obj),
            lst: Rc::default(),
        }
    }
}
impl ContainerExt for Naviframe {
    fn add(&self, child: &impl ElmObject) {
        let item = self.push(child);
        self.lst.borrow_mut().push(item);
        child.show();
    }
}
impl ElmObject for Naviframe {}
impl NaviframeExt for Naviframe {}

#[derive(Default)]
pub struct Notify(Option<*mut Evas_Object>);

impl Notify {
    pub fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_notify_add(prt.window().as_raw()) });
        elm.conf();
        prt.add(&elm);
        elm
    }
    pub fn set_timeout(&self, value: f64) {
        unsafe { elm_notify_timeout_set(self.as_raw(), value) };
    }
}

impl EvasObject for Notify {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Notify {}
impl ContainerExt for Notify {
    fn add(&self, child: &impl ElmObject) {
        self.set_content(child, "default");
        child.show();
    }
}

#[derive(Default)]
pub struct Panel(Option<*mut Evas_Object>);

impl EvasObject for Panel {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Panel {}
impl ContainerExt for Panel {
    fn add(&self, child: &impl ElmObject) {
        self.set_content(child, "default");
        child.show();
    }
}
impl OnToggled for Panel {}
impl PanelExt for Panel {}

#[derive(Default)]
pub struct Panes(Option<*mut Evas_Object>);

impl EvasObject for Panes {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Panes {}
impl ContainerExt for Panes {
    fn add(&self, child: &impl ElmObject) {
        match self.content("left") {
            None => self.set_content(child, "left"),
            _ => self.set_content(child, "right"),
        }
        child.show();
    }
}
impl OnClickedDouble for Panes {}
impl OnPressed for Panes {}
impl PanesExt for Panes {}

#[derive(Default)]
pub struct Popup(Option<*mut Evas_Object>);

impl EvasObject for Popup {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ContainerExt for Popup {}
impl ElmObject for Popup {}
impl PopupExt for Popup {}

#[derive(Default)]
pub struct ProgressBar(Option<*mut Evas_Object>);

impl EvasObject for ProgressBar {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for ProgressBar {}
impl OnClicked for ProgressBar {}
impl OnChanged for ProgressBar {}
impl ProgressBarExt for ProgressBar {}

#[derive(Default)]
pub struct Radio(Option<*mut Evas_Object>);

impl EvasObject for Radio {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Radio {}
impl OnChanged for Radio {}
impl RadioExt for Radio {}

#[derive(Default)]
pub struct Scroller(Option<*mut Evas_Object>);

impl EvasObject for Scroller {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Scroller {}
impl OnChanged for Scroller {}
impl ScrollerExt for Scroller {}

#[derive(Default)]
pub struct SegmentControl(Option<*mut Evas_Object>);

impl EvasObject for SegmentControl {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for SegmentControl {}
impl OnChanged for SegmentControl {}
impl SegmentControlExt for SegmentControl {}

#[derive(Default)]
pub struct Slider(Option<*mut Evas_Object>);

impl Slider {
    pub fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_slider_add(prt.as_raw()) });
        elm.conf();
        prt.add(&elm);
        elm
    }
    pub fn set_horizontal(&self, value: bool) {
        unsafe { elm_slider_horizontal_set(self.as_raw(), value as Eina_Bool) };
    }
}

impl EvasObject for Slider {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Slider {}
impl OnChanged for Slider {}
impl OnChangedDelay for Slider {}
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
pub struct Spinner(Option<*mut Evas_Object>);
impl Spinner {
    pub fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_spinner_add(prt.as_raw()) });
        elm.conf();
        prt.add(&elm);
        elm
    }
}
impl EvasObject for Spinner {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Spinner {}
impl OnChanged for Spinner {}
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
pub struct ToolBar(Option<*mut Evas_Object>);

impl EvasObject for ToolBar {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
//~ impl SelectorExt for ToolBar {
//~ fn add<F: FnMut(Self) + 'static>(&self, icon: &str, label: &str, func: F) -> WidgetItem {
//~ WidgetItem::from_raw(self.append(icon, label, func))
//~ }
//~ }
impl ElmObject for ToolBar {}
impl OnClicked for ToolBar {}
impl ToolBarExt for ToolBar {}

#[derive(Default)]
pub struct Window(Option<*mut Evas_Object>);

impl EvasObject for Window {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ContainerExt for Window {
    fn add(&self, child: &impl ElmObject) {
        self.add_object(child);
        child.show();
    }
}
impl ElmObject for Window {}
impl OnDeleteRequest for Window {}
impl WindowExt for Window {}

#[derive(Default)]
pub struct ActionSlider(Option<*mut Evas_Object>);

impl EvasObject for ActionSlider {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for ActionSlider {}
impl ActionSliderExt for ActionSlider {}
impl OnSelected for ActionSlider {}
impl OnPosChanged for ActionSlider {}

#[derive(Default)]
pub struct Box(Option<*mut Evas_Object>);

impl EvasObject for Box {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Box {}
impl ContainerExt for Box {
    fn add(&self, child: &impl ElmObject) {
        self.pack_end(child);
        self.recalculate();
        child.show();
    }
}
impl BoxExt for Box {}

#[derive(Default)]
pub struct Grid(Option<*mut Evas_Object>);

impl Grid {
    pub fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_grid_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
}

impl EvasObject for Grid {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Grid {}
impl ContainerExt for Grid {
    fn add(&self, child: &impl ElmObject) {
        self.pack(child, 0, 0, 1, 1);
        child.show();
    }
}
impl GridExt for Grid {}

#[derive(Default)]
pub struct Table(Option<*mut Evas_Object>);

impl Table {
    pub fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_table_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
}

impl EvasObject for Table {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Table {}
impl ContainerExt for Table {
    fn add(&self, child: &impl ElmObject) {
        self.pack(child, 0, 0, 1, 1);
        child.show();
    }
}
impl TableExt for Table {}

#[derive(Default)]
pub struct Bubble(Option<*mut Evas_Object>);
impl Bubble {
    pub fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_bubble_add(prt.as_raw()) })
            .with_conf()
            .with_pos(1);
        prt.add(&elm);
        elm
    }
    pub fn set_pos(&self, value: i32) {
        unsafe { elm_bubble_pos_set(self.as_raw(), value) };
    }
    pub fn with_pos(self, value: i32) -> Self {
        self.set_pos(value);
        self
    }
}
impl EvasObject for Bubble {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Bubble {}
impl ContainerExt for Bubble {
    fn add(&self, child: &impl prelude::ElmObject) {
        self.set_content(child, "default");
        child.show();
    }
}

#[derive(Default)]
pub struct Button(Option<*mut Evas_Object>);

impl Button {
    pub fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_button_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
}

impl EvasObject for Button {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Button {}
impl ContainerExt for Button {}
impl OnClicked for Button {}

#[derive(Default)]
pub struct Combobox(Option<*mut Evas_Object>);

impl EvasObject for Combobox {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Combobox {}
impl OnChanged for Combobox {}
impl OnSelected for Combobox {}
impl OnExpanded for Combobox {}
impl ComboboxExt for Combobox {}

#[derive(Default)]
pub struct Check(Option<*mut Evas_Object>);

impl EvasObject for Check {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Check {}
impl OnChanged for Check {}
impl CheckExt for Check {}

#[derive(Default)]
pub struct Conformant(Option<*mut Evas_Object>);

impl EvasObject for Conformant {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Conformant {}
impl ContainerExt for Conformant {
    fn add(&self, child: &impl ElmObject) {
        self.set_content(child, "default");
        child.show();
    }
}
impl ConformantExt for Conformant {}

#[derive(Default)]
pub struct Diskselector(Option<*mut Evas_Object>);

impl EvasObject for Diskselector {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Diskselector {}
impl SelectorExt for Diskselector {
    fn add<F: FnMut(Self) + 'static>(&self, label: &str, func: F) -> WidgetItem {
        self.append(label, func)
    }
    fn selected(&self) -> WidgetItem {
        WidgetItem::from_raw(unsafe { elm_diskselector_selected_item_get(self.as_raw()) })
    }
    fn first(&self) -> WidgetItem {
        WidgetItem::from_raw(unsafe { elm_diskselector_first_item_get(self.as_raw()) })
    }
    fn last(&self) -> WidgetItem {
        WidgetItem::from_raw(unsafe { elm_diskselector_last_item_get(self.as_raw()) })
    }
    fn value(&self) -> u32 {
        //~ unsafe { elm_diskselector_selected_index_get(self.as_raw()) as u32 }
        0
    }
    fn set_value(&self, value: u32) {
        //~ unsafe { elm_diskselector_selected_index_set(self.as_raw(), value as i32) };
    }
    fn clear(&self) {
        unsafe { elm_diskselector_clear(self.as_raw()) };
    }
}
impl OnChanged for Diskselector {}
impl DiskselectorExt for Diskselector {}

#[derive(Default)]
pub struct Colorselector(Option<*mut Evas_Object>);

impl EvasObject for Colorselector {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Colorselector {}
impl OnChanged for Colorselector {}
impl ColorselectorExt for Colorselector {}
#[derive(Default)]
pub struct FileSelector(Option<*mut Evas_Object>);

impl EvasObject for FileSelector {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for FileSelector {}
impl ContainerExt for FileSelector {}
impl OnActivated for FileSelector {}
impl OnSelected for FileSelector {}
impl OnDone for FileSelector {}
impl FileSelExt for FileSelector {}

#[derive(Default)]
pub struct Gengrid(Option<*mut Evas_Object>);

impl EvasObject for Gengrid {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Gengrid {}
impl OnSelected for Gengrid {}
impl OnUnselected for Gengrid {}
impl OnActivated for Gengrid {}
impl OnClickedDouble for Gengrid {}
impl GengridExt for Gengrid {}

#[derive(Default)]
pub struct Genlist(Option<*mut Evas_Object>);

impl EvasObject for Genlist {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Genlist {}
impl OnSelected for Genlist {}
impl OnUnselected for Genlist {}
impl OnActivated for Genlist {}
impl OnClickedDouble for Genlist {}
impl GenlistExt for Genlist {}
