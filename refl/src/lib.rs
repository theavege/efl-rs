pub mod prelude;

use {
    prelude::*,
    std::{cell::RefCell, rc::Rc},
};

#[derive(Default)]
pub enum Align {
    #[default]
    Fill,
    Left,
    Center,
    Right,
}

impl Align {
    fn to_f64(&self) -> f64 {
        match self {
            Align::Fill => -1.0,
            Align::Left => 0.0,
            Align::Center => 0.5,
            Align::Right => 1.0,
        }
    }
}

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

impl EvasObject for WidgetItem {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for WidgetItem {}

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
    fn add<F: FnMut(Self) + 'static>(&self, icon: &str, label: &str, func: F) -> WidgetItem {
        WidgetItem::from_raw(self.append(icon, label, func))
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

impl EvasObject for Clock {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Clock {}
impl ClockExt for Clock {}

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
impl SelectorExt for Ctxpopup {
    fn add<F: FnMut(Self) + 'static>(&self, icon: &str, label: &str, func: F) -> WidgetItem {
        WidgetItem::from_raw(self.append(icon, label, func))
    }
}
impl ElmObject for Ctxpopup {}
impl OnDismissed for Ctxpopup {}
impl CtxpopupExt for Ctxpopup {}

#[derive(Default)]
pub struct Entry(Option<*mut Evas_Object>);

impl EvasObject for Entry {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Entry {}
impl SelectorExt for Entry {
    fn add<F: FnMut(Self) + 'static>(&self, icon: &str, label: &str, func: F) -> WidgetItem {
        self.append(icon, label, func);
        WidgetItem::default()
    }
}
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
impl FlipSelectorExt for FlipSelector {}

#[derive(Default)]
pub struct Frame(Option<*mut Evas_Object>);

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
impl FrameExt for Frame {}

#[derive(Default)]
pub struct HoverSel(Option<*mut Evas_Object>);

impl EvasObject for HoverSel {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for HoverSel {}
impl SelectorExt for HoverSel {
    fn add<F: FnMut(Self) + 'static>(&self, icon: &str, label: &str, func: F) -> WidgetItem {
        WidgetItem::from_raw(self.append(icon, label, func))
    }
}
impl OnSelected for HoverSel {}
impl HoverSelExt for HoverSel {}

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

impl EvasObject for Label {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Label {}
impl LabelExt for Label {}

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

impl EvasObject for List {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl SelectorExt for List {
    fn add<F: FnMut(Self) + 'static>(&self, icon: &str, label: &str, func: F) -> WidgetItem {
        WidgetItem::from_raw(self.append(icon, label, func))
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
        let item = WidgetItem::from_raw(self.push(child));
        self.lst.borrow_mut().push(item);
        child.show();
    }
}
impl ElmObject for Naviframe {}
impl NaviframeExt for Naviframe {}

#[derive(Default)]
pub struct Notify(Option<*mut Evas_Object>);

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
impl NotifyExt for Notify {}

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
impl SliderExt for Slider {}

#[derive(Default)]
pub struct Spinner(Option<*mut Evas_Object>);

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
impl SpinnerExt for Spinner {}

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
impl SelectorExt for ToolBar {
    fn add<F: FnMut(Self) + 'static>(&self, icon: &str, label: &str, func: F) -> WidgetItem {
        WidgetItem::from_raw(self.append(icon, label, func))
    }
}
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
pub struct Bubble(Option<*mut Evas_Object>);

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
impl BubbleExt for Bubble {}

#[derive(Default)]
pub struct Button(Option<*mut Evas_Object>);

impl EvasObject for Button {
    fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!")
    }
    fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(Some(obj))
    }
}
impl ElmObject for Button {}
impl ButtonExt for Button {}
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
impl FileSelectorExt for FileSelector {}

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
