pub use std::sync::mpsc::Sender;
use {
    efltk_sys::*,
    std::{
        ffi::{CStr, CString, c_void},
        sync::mpsc::channel,
    },
};

pub trait SignalExt {
    fn to_str(&self) -> &str;
}

#[derive(Default)]
pub enum TriggerSignal {
    #[default]
    Clicked,
}

impl SignalExt for TriggerSignal {
    fn to_str(&self) -> &str {
        match self {
            Self::Clicked => "clicked",
        }
    }
}

#[derive(Default)]
pub enum SelectorSignal {
    #[default]
    Selected,
    Changed,
}

impl SignalExt for SelectorSignal {
    fn to_str(&self) -> &str {
        match self {
            Self::Selected => "selected",
            Self::Changed => "changed",
        }
    }
}

#[derive(Default)]
pub enum CheckSignal {
    #[default]
    Changed,
}
impl SignalExt for CheckSignal {
    fn to_str(&self) -> &str {
        match self {
            Self::Changed => "changed",
        }
    }
}

#[derive(Default)]
pub enum RangerSignal {
    #[default]
    Changed,
}
impl SignalExt for RangerSignal {
    fn to_str(&self) -> &str {
        match self {
            Self::Changed => "changed",
        }
    }
}

#[derive(Default)]
pub enum EntrySignal {
    #[default]
    Changed,
}
impl SignalExt for EntrySignal {
    fn to_str(&self) -> &str {
        match self {
            Self::Changed => "changed",
        }
    }
}

#[derive(Default)]
pub enum RadioSignal {
    #[default]
    Changed,
}
impl SignalExt for RadioSignal {
    fn to_str(&self) -> &str {
        match self {
            Self::Changed => "changed",
        }
    }
}

#[derive(Default)]
pub enum ProgressBarSignal {
    #[default]
    Changed,
}
impl SignalExt for ProgressBarSignal {
    fn to_str(&self) -> &str {
        match self {
            Self::Changed => "changed",
        }
    }
}

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
            Self::Fill => -1.0,
            Self::Left => 0.0,
            Self::Center => 0.5,
            Self::Right => 1.0,
        }
    }
}

#[derive(Default)]
pub enum PanelOrient {
    #[default]
    Top = 0,
    Bottom,
    Left,
    Right,
}

#[derive(Default)]
pub enum Cursor {
    #[default]
    Hand2,
    Hand1,
    Hand3,
    Bogocity,
    Xterm,
}
impl Cursor {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Hand1 => "hand1",
            Self::Hand2 => "hand2",
            Self::Hand3 => "hand3",
            Self::Bogocity => "bogocity",
            Self::Xterm => "xterm",
        }
    }
}

pub fn run(func: impl Fn() -> super::Window) {
    let c_args = std::env::args()
        .map(|arg| CString::new(arg).unwrap())
        .map(|arg| arg.as_ptr())
        .collect::<Vec<*const i8>>();
    unsafe {
        elm_init(c_args.len() as i32, c_args.as_ptr() as *mut *mut i8);
        elm_policy_set(
            Elm_Policy_ELM_POLICY_QUIT,
            Elm_Policy_Quit_ELM_POLICY_QUIT_LAST_WINDOW_CLOSED as i32,
        );
        func().show();
        elm_run();
        elm_shutdown();
    }
}

pub fn exit() {
    unsafe {
        efl_exit(0);
    };
}

pub trait EcoreTimerExt: Sized {
    fn as_ptr(&self) -> *mut Ecore_Timer;
}

pub trait TimerExt: EcoreTimerExt + From<*mut Ecore_Timer> {
    fn new<F: FnMut() -> bool + 'static>(timeout: f64, func: F) -> Self {
        let raw_ptr: *mut Box<EcoreCb> = Box::into_raw(Box::new(Box::new(func)));
        Self::from(unsafe { ecore_timer_add(timeout, Some(ecore_task_cb), raw_ptr as *mut c_void) })
    }
    fn del(&self) {
        unsafe { ecore_timer_del(self.as_ptr()) };
    }
    fn set_freeze(&self) {
        unsafe { ecore_timer_freeze(self.as_ptr()) };
    }
    fn set_delay(&self, value: f64) {
        unsafe { ecore_timer_delay(self.as_ptr(), value) };
    }
}

pub trait EcoreEventExt: Sized {
    fn as_ptr(&self) -> *mut Ecore_Event_Handler;
}

pub trait EventHandlerExt: EcoreEventExt + From<*mut Ecore_Event_Handler> {
    fn new<F: FnMut() -> bool + 'static>(type_: i32, func: F) -> Self {
        let raw_ptr: *mut Box<EcoreCb> = Box::into_raw(Box::new(Box::new(func)));
        Self::from(unsafe {
            ecore_event_handler_add(type_, Some(ecore_event_handler_cb), raw_ptr as *mut c_void)
        })
    }
    fn del(&self) {
        unsafe { ecore_event_handler_del(self.as_ptr()) };
    }
}

type EcoreCb = dyn FnMut() -> bool;

pub(crate) unsafe extern "C" fn smart_cb<T: ElmObject>(
    data: *mut c_void,
    object: *mut Evas_Object,
    _event_info: *mut c_void,
) {
    unsafe {
        let func: &mut Box<dyn FnMut(T)> = &mut *(data as *mut Box<dyn FnMut(T)>);
        func(T::from_raw(object));
    }
}

pub(crate) unsafe extern "C" fn ecore_task_cb(data: *mut c_void) -> Eina_Bool {
    unsafe {
        let func: &mut Box<EcoreCb> = &mut *(data as *mut Box<EcoreCb>);
        func() as Eina_Bool
    }
}

pub(crate) unsafe extern "C" fn ecore_event_handler_cb(
    data: *mut c_void,
    _type_: i32,
    _event: *mut c_void,
) -> Eina_Bool {
    unsafe {
        let func: &mut Box<EcoreCb> = &mut *(data as *mut Box<EcoreCb>);
        func() as Eina_Bool
    }
}

pub trait EvasObjectItemExt: Sized {
    fn as_raw(&self) -> *mut Evas_Object;
    fn from_raw(obj: *mut Evas_Object) -> Self;
    fn text(&self) -> String {
        unsafe {
            let ptr = elm_object_item_part_text_get(self.as_raw(), std::ptr::null());
            if !ptr.is_null() {
                CStr::from_ptr(ptr).to_string_lossy().into_owned()
            } else {
                String::new()
            }
        }
    }
    fn del(&self) {
        unsafe { elm_object_item_del(self.as_raw()) };
    }
}
pub trait EvasObject: Sized {
    fn as_raw(&self) -> *mut Evas_Object;
    fn from_raw(obj: *mut Evas_Object) -> Self;
    fn show(&self) {
        unsafe {
            evas_object_show(self.as_raw());
        };
    }
    fn del(&self) {
        unsafe {
            evas_object_del(self.as_raw());
        };
    }
    fn parent<T: ElmObject>(&self) -> T {
        T::from_raw(unsafe { efl_parent_get(self.as_raw()) })
    }
    fn set_weight(&self, x: bool, y: bool) {
        unsafe {
            evas_object_size_hint_weight_set(self.as_raw(), x as u8 as f64, y as u8 as f64);
        };
    }
    fn set_align(&self, x: Align, y: Align) {
        unsafe {
            evas_object_size_hint_align_set(self.as_raw(), x.to_f64(), y.to_f64());
        };
    }
    fn set_min_size(&self, w: i32, h: i32) {
        unsafe {
            evas_object_size_hint_min_set(self.as_raw(), w, h);
            evas_object_resize(self.as_raw(), w, h);
        };
    }
    fn set_size(&self, w: i32, h: i32) {
        unsafe {
            evas_object_size_hint_max_set(self.as_raw(), w, h);
            evas_object_resize(self.as_raw(), w, h);
        };
        self.set_min_size(w, h);
        self.set_weight(w == -1, h == -1);
    }
    fn geometry(&self) -> (i32, i32, i32, i32) {
        let (mut x, mut y, mut w, mut h) = (0, 0, 0, 0);
        unsafe { evas_object_geometry_get(self.as_raw(), &mut x, &mut y, &mut w, &mut h) };
        (x, y, w, h)
    }
    fn with_weight(self, x: bool, y: bool) -> Self {
        self.set_weight(x, y);
        self
    }
    fn with_align(self, x: Align, y: Align) -> Self {
        self.set_align(x, y);
        self
    }
    fn with_size(self, w: i32, h: i32) -> Self {
        self.set_size(w, h);
        self
    }
    fn with_min_size(self, w: i32, h: i32) -> Self {
        self.set_min_size(w, h);
        self
    }
}

pub trait ElmObject: EvasObject {
    fn with_text(self, text: &str) -> Self {
        self.set_text(text);
        self
    }
    fn with_style(self, value: &str) -> Self {
        self.set_style(value);
        self
    }
    fn with_icon(self, value: &str) -> Self {
        self.set_icon(value);
        self
    }
    fn with_conf(self) -> Self {
        self.set_align(Align::Fill, Align::Fill);
        self.set_weight(true, true);
        self.set_focus(false);
        self
    }
    fn set_icon(&self, value: &str) {
        super::Icon::new(self).with_standard(value);
    }
    fn set_text(&self, text: &str) {
        self.set_part("default", text);
    }
    fn set_part(&self, part: &str, text: &str) {
        let c_part = CString::new(part).unwrap();
        let c_text = CString::new(text).unwrap();
        let c_part_ptr = match part.is_empty() {
            true => std::ptr::null(),
            false => c_part.as_ptr(),
        };
        let c_text_ptr = match text.is_empty() {
            true => std::ptr::null(),
            false => c_text.as_ptr(),
        };
        unsafe { elm_object_part_text_set(self.as_raw(), c_part_ptr, c_text_ptr) };
    }
    fn set_style(&self, value: &str) -> bool {
        let ctext = CString::new(value).unwrap();
        unsafe { elm_object_style_set(self.as_raw(), ctext.as_ptr()) != 0 }
    }
    fn with_tooltip(self, value: &str) -> Self {
        self.set_tooltip(value);
        self
    }
    fn with_part(self, part: &str, text: &str) -> Self {
        self.set_part(part, text);
        self
    }
    fn with_cursor(self, cursor: Cursor) -> Self {
        self.set_cursor(cursor);
        self
    }
    fn set_tooltip(&self, value: &str) {
        let ctext = CString::new(value).unwrap();
        unsafe { elm_object_tooltip_text_set(self.as_raw(), ctext.as_ptr()) }
    }
    fn set_cursor(&self, cursor: Cursor) -> bool {
        unsafe {
            elm_object_cursor_set(
                self.as_raw(),
                CString::new(cursor.to_str()).unwrap().as_ptr(),
            ) != 0
        }
    }
    fn disabled(&self) -> bool {
        unsafe { elm_object_disabled_get(self.as_raw()) != 0 }
    }
    fn focus(&self) -> bool {
        unsafe { elm_object_focus_get(self.as_raw()) != 0 }
    }
    fn with_disabled(self, disabled: bool) -> Self {
        self.set_disabled(disabled);
        self
    }
    fn set_disabled(&self, disabled: bool) {
        unsafe { elm_object_disabled_set(self.as_raw(), disabled as Eina_Bool) }
    }
    fn set_focus(&self, value: bool) {
        unsafe { elm_object_focus_set(self.as_raw(), value as Eina_Bool) }
    }
    fn with_content(self, obj: &impl ElmObject, value: &str) -> Self {
        self.set_content(obj, value);
        self
    }
    fn set_content(&self, obj: &impl ElmObject, value: &str) {
        let ctext = CString::new(value).unwrap();
        unsafe { elm_object_part_content_set(self.as_raw(), ctext.as_ptr(), obj.as_raw()) }
        obj.show();
    }
    fn content(&self, value: &str) -> Option<super::WidgetItem> {
        let ctext = CString::new(value).unwrap();
        let ptr = unsafe { elm_object_part_content_get(self.as_raw(), ctext.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(super::WidgetItem::from_raw(ptr))
        }
    }
    fn text(&self) -> String {
        unsafe {
            let ptr = elm_object_part_text_get(self.as_raw(), std::ptr::null());
            match !ptr.is_null() {
                true => CStr::from_ptr(ptr).to_string_lossy().into_owned(),
                false => String::new(),
            }
        }
    }
    fn window(&self) -> super::Window {
        super::Window::from_raw(unsafe { elm_win_get(self.as_raw()) })
    }
    fn set_callback<F: FnMut(Self) + 'static>(&self, sign: impl SignalExt, func: F) {
        let raw_ptr: *mut Box<dyn FnMut(Self)> = Box::into_raw(Box::new(Box::new(func)));
        unsafe {
            evas_object_smart_callback_add(
                self.as_raw(),
                CString::new(sign.to_str()).unwrap().as_ptr(),
                Some(smart_cb::<Self>),
                raw_ptr as *mut c_void,
            );
        }
    }
    fn do_callback(&self, sign: impl SignalExt) {
        unsafe {
            evas_object_smart_callback_call(
                self.as_raw(),
                CString::new(sign.to_str()).unwrap().as_ptr(),
                std::ptr::null_mut(),
            );
        }
    }
}

pub trait BubbleExt: ContainerExt {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_bubble_add(prt.as_raw()) })
            .with_conf()
            .with_pos(1);
        prt.add(&elm);
        elm
    }
    fn with_info(self, info: &str) -> Self {
        self.set_part("info", info);
        self
    }
    fn with_pos(self, value: i32) -> Self {
        unsafe { elm_bubble_pos_set(self.as_raw(), value) };
        self
    }
}

pub trait LabelExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe {
            let ptr = elm_label_add(prt.as_raw());
            elm_label_line_wrap_set(ptr, Elm_Wrap_Type_ELM_WRAP_WORD);
            ptr
        })
        .with_conf();
        prt.add(&elm);
        elm
    }
}
pub trait SpinnerExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_spinner_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
}
pub trait ClockExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_clock_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
    fn time(&self) -> (i32, i32, i32) {
        let (mut hrs, mut min, mut sec) = (0, 0, 0);
        unsafe { elm_clock_time_get(self.as_raw(), &mut hrs, &mut min, &mut sec) };
        (hrs, min, sec)
    }
    fn set_time(&self, hrs: i32, min: i32, sec: i32) {
        unsafe { elm_clock_time_set(self.as_raw(), hrs, min, sec) };
    }
}

pub trait ButtonExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_button_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
    fn with_callback<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.set_callback(TriggerSignal::Clicked, func);
        self
    }
    fn with_signal<F: FnMut(Self) + 'static>(self, sign: TriggerSignal, func: F) -> Self {
        self.set_callback(sign, func);
        self
    }
}

pub trait ContainerExt: ElmObject {
    fn add(&self, child: &impl ElmObject) {
        child.show();
    }
    fn inside(&self, mut func: impl FnMut(&Self)) {
        func(self);
    }
}

pub trait ConformantExt: ContainerExt {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_conformant_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
}

pub trait ColorselectorExt: Sized + ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_colorselector_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
    fn color(&self) -> (i32, i32, i32, i32) {
        let (mut r, mut g, mut b, mut a) = (0, 0, 0, 0);
        unsafe { elm_colorselector_color_get(self.as_raw(), &mut r, &mut g, &mut b, &mut a) };
        (r, g, b, a)
    }
    fn set_color(&self, r: i32, g: i32, b: i32, a: i32) {
        unsafe { elm_colorselector_color_set(self.as_raw(), r, g, b, a) };
    }
}

pub trait DayselectorExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_dayselector_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
}

pub trait DiskselectorExt: Sized + ElmObject {
    #[deprecated = "use efltk::FlipSelector::new(&parent) instead"]
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_diskselector_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
    fn append<F: FnMut(Self) + 'static>(&self, label_: &str, func: F) -> super::WidgetItem {
        let raw_ptr: *mut Box<dyn FnMut(Self)> = Box::into_raw(Box::new(Box::new(func)));
        super::WidgetItem::from_raw(unsafe {
            elm_diskselector_item_append(
                self.as_raw(),
                CString::new(label_).unwrap().as_ptr(),
                super::Icon::new(self).with_standard(label_).as_raw(),
                Some(smart_cb::<Self>),
                raw_ptr as *mut c_void,
            )
        })
    }
}
pub trait BoxExt: ContainerExt {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_box_add(prt.as_raw()) })
            .with_homogeneous(false)
            .with_horizontal(false)
            .with_conf();
        prt.add(&elm);
        elm
    }
    fn with_horizontal(self, value: bool) -> Self {
        self.set_horizontal(value);
        self
    }
    fn with_homogeneous(self, value: bool) -> Self {
        unsafe { elm_box_homogeneous_set(self.as_raw(), value as Eina_Bool) };
        self
    }
    fn with_padding(self, horizontal: i32, vertical: i32) -> Self {
        unsafe { elm_box_padding_set(self.as_raw(), horizontal, vertical) };
        self
    }
    fn add_item(&self, item: &impl super::ElmObject) {
        unsafe {
            elm_box_pack_end(self.as_raw(), item.as_raw());
            elm_box_recalculate(self.as_raw());
        };
    }
    fn set_horizontal(&self, value: bool) {
        unsafe { elm_box_horizontal_set(self.as_raw(), value as Eina_Bool) };
        self.set_weight(value, !value);
    }
}

pub trait MenuExt: SelectorExt {
    fn popup(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_menu_add(prt.window().as_raw()) }).with_conf();
        elm.close();
        elm
    }
    fn main(win: &impl ContainerExt) -> Self {
        Self::from_raw(unsafe { elm_win_main_menu_get(win.as_raw()) }).with_conf()
    }
    fn with_appends<F: FnMut(Self) + 'static + Clone>(self, items: &[&str], func: F) -> Self {
        for item in items {
            self.append(item, item, func.clone());
        }
        self
    }
    fn append<F: FnMut(Self) + 'static>(
        &self,
        icon: &str,
        label: &str,
        func: F,
    ) -> super::WidgetItem {
        let raw_ptr: *mut Box<dyn FnMut(Self)> = Box::into_raw(Box::new(Box::new(func)));
        super::WidgetItem::from_raw(unsafe {
            elm_menu_item_add(
                self.as_raw(),
                std::ptr::null_mut(),
                CString::new(icon).unwrap().as_ptr(),
                CString::new(label).unwrap().as_ptr(),
                Some(smart_cb::<Self>),
                raw_ptr as *mut c_void,
            )
        })
    }
    fn close(&self) {
        unsafe { elm_menu_close(self.as_raw()) }
    }
    fn open(&self, x: i32, y: i32) {
        unsafe {
            elm_menu_open(self.as_raw());
            elm_menu_move(self.as_raw(), x, y);
        }
    }
}

pub trait ComboboxExt: ElmObject {
    #[deprecated = "use efltk::HoverSel::new(&parent) instead"]
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_combobox_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
    /// Append a text item to the combobox.
    /// EFL's combobox wraps a genlist internally; there is no
    /// `elm_combobox_item_append` — items are added via
    /// `elm_genlist_item_append` with an `Elm_Genlist_Item_Class`.
    fn append(&self, label: &str) {
        // Build a genlist item class whose text_get callback returns the label.
        // SAFETY: the item class is intentionally leaked so it lives for the
        // duration of the program (standard EFL C pattern).
        let itc: *mut Elm_Genlist_Item_Class = unsafe {
            let itc = elm_genlist_item_class_new();
            (*itc).item_style = CString::new("default").unwrap().into_raw();
            (*itc).func.text_get = Some(combobox_text_get);
            (*itc).func.content_get = None;
            (*itc).func.state_get = None;
            (*itc).func.del = Some(combobox_item_del);
            itc
        };
        // Heap-allocate the label so it outlives this call; freed in del cb.
        let data: *mut std::ffi::c_char = CString::new(label).unwrap().into_raw();
        unsafe {
            elm_genlist_item_append(
                self.as_raw(),
                itc,
                data as *mut c_void,
                std::ptr::null_mut(),
                Elm_Genlist_Item_Type_ELM_GENLIST_ITEM_NONE,
                None,
                std::ptr::null_mut(),
            );
        }
    }
    fn with_item(self, label: &str) -> Self {
        self.append(label);
        self
    }
    fn with_items(self, items: &[&str]) -> Self {
        for item in items {
            self.append(item);
        }
        self
    }
    fn value(&self) -> u32 {
        self.index()
    }
    fn set_value(&self, value: u32) {
        self.set_index(value);
    }
    fn index(&self) -> u32 {
        let mut count = 0u32;
        let mut temp = self.first().as_raw();
        while temp != self.selected().as_raw() {
            temp = unsafe { elm_genlist_item_next_get(temp) };
            count += 1;
        }
        count
    }
    fn set_index(&self, value: u32) {
        let mut temp = self.first().as_raw();
        for _ in 0..value {
            temp = unsafe { elm_genlist_item_next_get(temp) };
        }
        if !temp.is_null() {
            unsafe { elm_genlist_item_selected_set(temp, true as Eina_Bool) };
        }
    }
    fn selected(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_genlist_selected_item_get(self.as_raw()) })
    }
    fn first(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_genlist_first_item_get(self.as_raw()) })
    }
    fn last(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_genlist_last_item_get(self.as_raw()) })
    }
    fn clear(&self) {
        unsafe {
            elm_combobox_hover_end(self.as_raw());
            elm_genlist_clear(self.as_raw());
        };
    }
}

/// Genlist `text_get` callback: returns a strdup of the label stored as item data.
unsafe extern "C" fn combobox_text_get(
    data: *mut c_void,
    _obj: *mut Evas_Object,
    _part: *const std::ffi::c_char,
) -> *mut std::ffi::c_char {
    if data.is_null() {
        return std::ptr::null_mut();
    }
    // EFL frees the returned pointer with free(), so we must heap-duplicate it.
    unsafe {
        let src = data as *const std::ffi::c_char;
        let bytes = std::ffi::CStr::from_ptr(src).to_bytes_with_nul();
        let layout = std::alloc::Layout::array::<u8>(bytes.len()).unwrap();
        let buf = std::alloc::alloc(layout);
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, bytes.len());
        buf as *mut std::ffi::c_char
    }
}

/// Genlist `del` callback: frees the heap-allocated label.
unsafe extern "C" fn combobox_item_del(data: *mut c_void, _obj: *mut Evas_Object) {
    if !data.is_null() {
        unsafe { drop(CString::from_raw(data as *mut std::ffi::c_char)) };
    }
}

pub trait FileSelExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_fileselector_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
    /// Set the starting directory path shown in the selector.
    fn set_path(&self, path: &str) {
        let c = CString::new(path).unwrap();
        unsafe { elm_fileselector_path_set(self.as_raw(), c.as_ptr()) };
    }
    fn with_path(self, path: &str) -> Self {
        self.set_path(path);
        self
    }
    /// Get the currently displayed directory path.
    fn path(&self) -> String {
        unsafe {
            let ptr = elm_fileselector_path_get(self.as_raw());
            if ptr.is_null() {
                String::new()
            } else {
                CStr::from_ptr(ptr).to_string_lossy().into_owned()
            }
        }
    }
    /// Get the currently selected file/directory path.
    fn selected(&self) -> String {
        unsafe {
            let ptr = elm_fileselector_selected_get(self.as_raw());
            if ptr.is_null() {
                String::new()
            } else {
                CStr::from_ptr(ptr).to_string_lossy().into_owned()
            }
        }
    }
    /// Set the selected path programmatically.
    fn set_selected(&self, path: &str) -> bool {
        let c = CString::new(path).unwrap();
        unsafe { elm_fileselector_selected_set(self.as_raw(), c.as_ptr()) != 0 }
    }
    /// Show only directories (no files).
    fn set_folder_only(&self, value: bool) {
        unsafe { elm_fileselector_folder_only_set(self.as_raw(), value as Eina_Bool) };
    }
    fn with_folder_only(self, value: bool) -> Self {
        self.set_folder_only(value);
        self
    }
    fn folder_only(&self) -> bool {
        unsafe { elm_fileselector_folder_only_get(self.as_raw()) != 0 }
    }
    /// Enable save-dialog mode (shows a filename entry field).
    fn set_is_save(&self, value: bool) {
        unsafe { elm_fileselector_is_save_set(self.as_raw(), value as Eina_Bool) };
    }
    fn with_is_save(self, value: bool) -> Self {
        self.set_is_save(value);
        self
    }
    fn is_save(&self) -> bool {
        unsafe { elm_fileselector_is_save_get(self.as_raw()) != 0 }
    }
    /// Allow multiple file selection.
    fn set_multi_select(&self, value: bool) {
        unsafe { elm_fileselector_multi_select_set(self.as_raw(), value as Eina_Bool) };
    }
    fn with_multi_select(self, value: bool) -> Self {
        self.set_multi_select(value);
        self
    }
    fn multi_select(&self) -> bool {
        unsafe { elm_fileselector_multi_select_get(self.as_raw()) != 0 }
    }
    /// Show hidden files and directories.
    fn set_hidden_visible(&self, value: bool) {
        unsafe { elm_fileselector_hidden_visible_set(self.as_raw(), value as Eina_Bool) };
    }
    fn with_hidden_visible(self, value: bool) -> Self {
        self.set_hidden_visible(value);
        self
    }
    fn hidden_visible(&self) -> bool {
        unsafe { elm_fileselector_hidden_visible_get(self.as_raw()) != 0 }
    }
    /// Use tree-expand mode instead of flat list.
    fn set_expandable(&self, value: bool) {
        unsafe { elm_fileselector_expandable_set(self.as_raw(), value as Eina_Bool) };
    }
    fn with_expandable(self, value: bool) -> Self {
        self.set_expandable(value);
        self
    }
    fn expandable(&self) -> bool {
        unsafe { elm_fileselector_expandable_get(self.as_raw()) != 0 }
    }
    /// Set the display mode (list or grid).
    fn set_mode(&self, value: super::FileSelectorMode) {
        unsafe { elm_fileselector_mode_set(self.as_raw(), value as Elm_Fileselector_Mode) };
    }
    fn with_mode(self, value: super::FileSelectorMode) -> Self {
        self.set_mode(value);
        self
    }
    fn mode(&self) -> super::FileSelectorMode {
        match unsafe { elm_fileselector_mode_get(self.as_raw()) } {
            1 => super::FileSelectorMode::Grid,
            _ => super::FileSelectorMode::List,
        }
    }
    /// Show the built-in OK/Cancel buttons.
    fn set_buttons_ok_cancel(&self, value: bool) {
        unsafe { elm_fileselector_buttons_ok_cancel_set(self.as_raw(), value as Eina_Bool) };
    }
    fn with_buttons_ok_cancel(self, value: bool) -> Self {
        self.set_buttons_ok_cancel(value);
        self
    }
    fn buttons_ok_cancel(&self) -> bool {
        unsafe { elm_fileselector_buttons_ok_cancel_get(self.as_raw()) != 0 }
    }
}

/// Gengrid `text_get` callback: returns a heap-duplicate of the label stored
/// as item data. EFL calls `free()` on the returned pointer.
unsafe extern "C" fn gengrid_text_get(
    data: *mut c_void,
    _obj: *mut Evas_Object,
    _part: *const std::ffi::c_char,
) -> *mut std::ffi::c_char {
    if data.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let src = data as *const std::ffi::c_char;
        let bytes = std::ffi::CStr::from_ptr(src).to_bytes_with_nul();
        let layout = std::alloc::Layout::array::<u8>(bytes.len()).unwrap();
        let buf = std::alloc::alloc(layout);
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, bytes.len());
        buf as *mut std::ffi::c_char
    }
}

/// Gengrid `del` callback: frees the heap-allocated label.
unsafe extern "C" fn gengrid_item_del(data: *mut c_void, _obj: *mut Evas_Object) {
    if !data.is_null() {
        unsafe { drop(CString::from_raw(data as *mut std::ffi::c_char)) };
    }
}

pub trait GengridExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_gengrid_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
    /// Append a labelled item to the grid.
    /// Items use `Elm_Gengrid_Item_Class` with a `text_get` callback;
    /// the label is heap-allocated and freed in the `del` callback.
    fn append(&self, label: &str) -> super::WidgetItem {
        let itc: *mut Elm_Gengrid_Item_Class = unsafe {
            let itc = elm_gengrid_item_class_new();
            (*itc).item_style = CString::new("default").unwrap().into_raw();
            (*itc).func.text_get = Some(gengrid_text_get);
            (*itc).func.content_get = None;
            (*itc).func.state_get = None;
            (*itc).func.del = Some(gengrid_item_del);
            itc
        };
        let data: *mut std::ffi::c_char = CString::new(label).unwrap().into_raw();
        super::WidgetItem::from_raw(unsafe {
            elm_gengrid_item_append(
                self.as_raw(),
                itc,
                data as *mut c_void,
                None,
                std::ptr::null_mut(),
            )
        })
    }
    fn with_item(self, label: &str) -> Self {
        self.append(label);
        self
    }
    fn with_items(self, items: &[&str]) -> Self {
        for item in items {
            self.append(item);
        }
        self
    }
    /// Set the size of each grid cell in pixels.
    fn set_item_size(&self, w: i32, h: i32) {
        unsafe { elm_gengrid_item_size_set(self.as_raw(), w, h) };
    }
    fn with_item_size(self, w: i32, h: i32) -> Self {
        self.set_item_size(w, h);
        self
    }
    fn item_size(&self) -> (i32, i32) {
        let (mut w, mut h) = (0i32, 0i32);
        unsafe { elm_gengrid_item_size_get(self.as_raw(), &mut w, &mut h) };
        (w, h)
    }
    /// Scroll horizontally instead of vertically.
    fn set_horizontal(&self, value: bool) {
        unsafe { elm_gengrid_horizontal_set(self.as_raw(), value as Eina_Bool) };
    }
    fn with_horizontal(self, value: bool) -> Self {
        self.set_horizontal(value);
        self
    }
    fn horizontal(&self) -> bool {
        unsafe { elm_gengrid_horizontal_get(self.as_raw()) != 0 }
    }
    /// Allow selecting multiple items at once.
    fn set_multi_select(&self, value: bool) {
        unsafe { elm_gengrid_multi_select_set(self.as_raw(), value as Eina_Bool) };
    }
    fn with_multi_select(self, value: bool) -> Self {
        self.set_multi_select(value);
        self
    }
    fn multi_select(&self) -> bool {
        unsafe { elm_gengrid_multi_select_get(self.as_raw()) != 0 }
    }
    fn selected(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_gengrid_selected_item_get(self.as_raw()) })
    }
    fn first(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_gengrid_first_item_get(self.as_raw()) })
    }
    fn last(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_gengrid_last_item_get(self.as_raw()) })
    }
    fn find(&self, item: super::WidgetItem) -> u32 {
        let mut count = 0u32;
        let mut temp = self.first().as_raw();
        while temp != item.as_raw() {
            temp = unsafe { elm_gengrid_item_next_get(self.as_raw()) };
            count += 1;
        }
        count
    }
    fn index(&self) -> u32 {
        self.find(self.selected())
    }
    fn value(&self) -> u32 {
        self.index()
    }
    fn set_index(&self, value: u32) {
        let mut temp = self.first().as_raw();
        for _ in 0..value {
            if temp.is_null() {
                return;
            }
            temp = unsafe { elm_gengrid_item_next_get(temp) };
        }
        if !temp.is_null() {
            unsafe { elm_gengrid_item_selected_set(temp, true as Eina_Bool) };
        }
    }
    fn set_value(&self, value: u32) {
        self.set_index(value);
    }
    fn clear(&self) {
        unsafe { elm_gengrid_clear(self.as_raw()) };
    }
}

pub trait GenlistExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_genlist_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
    /// Append a labelled item to the list.
    /// Items use `Elm_Genlist_Item_Class` with a `text_get` callback;
    /// the label is heap-allocated and freed in the `del` callback.
    fn append(&self, label: &str) -> super::WidgetItem {
        let itc: *mut Elm_Genlist_Item_Class = unsafe {
            let itc = elm_genlist_item_class_new();
            (*itc).item_style = CString::new("default").unwrap().into_raw();
            (*itc).func.text_get = Some(genlist_text_get);
            (*itc).func.content_get = None;
            (*itc).func.state_get = None;
            (*itc).func.del = Some(genlist_item_del);
            itc
        };
        let data: *mut std::ffi::c_char = CString::new(label).unwrap().into_raw();
        super::WidgetItem::from_raw(unsafe {
            elm_genlist_item_append(
                self.as_raw(),
                itc,
                data as *mut c_void,
                std::ptr::null_mut(),
                Elm_Genlist_Item_Type_ELM_GENLIST_ITEM_NONE,
                None,
                std::ptr::null_mut(),
            )
        })
    }
    fn with_item(self, label: &str) -> Self {
        self.append(label);
        self
    }
    fn with_items(self, items: &[&str]) -> Self {
        for item in items {
            self.append(item);
        }
        self
    }
    fn selected(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_genlist_selected_item_get(self.as_raw()) })
    }
    fn first(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_genlist_first_item_get(self.as_raw()) })
    }
    fn last(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_genlist_last_item_get(self.as_raw()) })
    }
    fn find(&self, item: super::WidgetItem) -> u32 {
        let mut count = 0u32;
        let mut temp = self.first().as_raw();
        while !temp.is_null() && temp != item.as_raw() {
            temp = unsafe { elm_genlist_item_next_get(temp) };
            count += 1;
        }
        count
    }
    fn index(&self) -> u32 {
        self.find(self.selected())
    }
    fn value(&self) -> u32 {
        self.index()
    }
    fn set_index(&self, value: u32) {
        let mut temp = self.first().as_raw();
        for _ in 0..value {
            if temp.is_null() {
                return;
            }
            temp = unsafe { elm_genlist_item_next_get(temp) };
        }
        if !temp.is_null() {
            unsafe { elm_genlist_item_selected_set(temp, true as Eina_Bool) };
        }
    }
    fn set_value(&self, value: u32) {
        self.set_index(value);
    }
    /// Allow selecting multiple items at once.
    fn set_multi_select(&self, value: bool) {
        unsafe { elm_genlist_multi_select_set(self.as_raw(), value as Eina_Bool) };
    }
    fn with_multi_select(self, value: bool) -> Self {
        self.set_multi_select(value);
        self
    }
    fn multi_select(&self) -> bool {
        unsafe { elm_genlist_multi_select_get(self.as_raw()) != 0 }
    }
    fn clear(&self) {
        unsafe { elm_genlist_clear(self.as_raw()) };
    }
}

/// Genlist `text_get` callback: returns a heap-duplicate of the label stored
/// as item data.
unsafe extern "C" fn genlist_text_get(
    data: *mut c_void,
    _obj: *mut Evas_Object,
    _part: *const std::ffi::c_char,
) -> *mut std::ffi::c_char {
    if data.is_null() {
        return std::ptr::null_mut();
    }
    // EFL frees the returned pointer with free(), so we must heap-duplicate it.
    unsafe {
        let src = data as *const std::ffi::c_char;
        let bytes = std::ffi::CStr::from_ptr(src).to_bytes_with_nul();
        let layout = std::alloc::Layout::array::<u8>(bytes.len()).unwrap();
        let buf = std::alloc::alloc(layout);
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, bytes.len());
        buf as *mut std::ffi::c_char
    }
}

/// Genlist `del` callback: frees the heap-allocated label.
unsafe extern "C" fn genlist_item_del(data: *mut c_void, _obj: *mut Evas_Object) {
    if !data.is_null() {
        unsafe { drop(CString::from_raw(data as *mut std::ffi::c_char)) };
    }
}

pub trait CheckExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_check_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
    fn set_value(&self, value: bool) {
        unsafe { elm_check_state_set(self.as_raw(), value as Eina_Bool) };
    }
    fn value(&self) -> bool {
        unsafe { elm_check_state_get(self.as_raw()) != 0 }
    }
    fn with_callback<F: FnMut(Self) + 'static>(self, sign: CheckSignal, func: F) -> Self {
        self.set_callback(sign, func);
        self
    }
}

pub trait RangerExt: ElmObject {
    fn value(&self) -> f64;
    fn set_format(&self, value: &str);
    fn set_step(&self, step: f64);
    fn set_range(&self, min: f64, max: f64);
    fn set_value(&self, value: f64);
    fn with_callback<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.set_callback(RangerSignal::Changed, func);
        self
    }
    fn with_signal<F: FnMut(Self) + 'static>(self, sign: RangerSignal, func: F) -> Self {
        self.set_callback(sign, func);
        self
    }
    fn with_range(self, min: f64, max: f64) -> Self {
        self.set_range(min, max);
        self
    }
    fn with_step(self, value: f64) -> Self {
        self.set_step(value);
        self
    }
    fn with_format(self, value: &str) -> Self {
        self.set_format(value);
        self
    }
}

pub trait SelectorExt: ElmObject {
    fn find(&self, item: super::WidgetItem) -> u32;
    fn add(&self, label: &str) -> super::WidgetItem;
    fn lenght(&self) -> u32;
    fn value(&self) -> u32;
    fn clear(&self);
    fn set_value(&self, value: u32);
    fn with_callback<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.set_callback(SelectorSignal::Selected, func);
        self
    }
    fn with_signal<F: FnMut(Self) + 'static>(self, sign: SelectorSignal, func: F) -> Self {
        self.set_callback(sign, func);
        self
    }
    fn add_items(&self, items: &[&str]) {
        for item in items {
            self.add(item);
        }
        self.set_value(0);
    }
    fn with_value(self, value: u32) -> Self {
        self.set_value(value);
        self
    }
    fn with_item(self, label: &str) -> Self {
        self.add(label);
        self
    }
    fn with_items(self, items: &[&str]) -> Self {
        self.add_items(items);
        self
    }
}
pub trait SliderExt: RangerExt {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_slider_add(prt.as_raw()) })
            .with_conf()
            .with_horizontal(true);
        prt.add(&elm);
        elm
    }
    fn set_horizontal(&self, value: bool) {
        unsafe { elm_slider_horizontal_set(self.as_raw(), value as Eina_Bool) };
        self.set_weight(value, !value);
    }
    fn with_horizontal(self, value: bool) -> Self {
        self.set_horizontal(value);
        self
    }
}
pub trait ActionSliderExt: ElmObject {
    #[deprecated = "use efltk::FlipSelector::new(&parent) instead"]
    fn new(prt: &impl ContainerExt, left: &str, center: &str, right: &str) -> Self {
        let elm = Self::from_raw(unsafe { elm_actionslider_add(prt.as_raw()) }).with_conf();
        elm.set_left(left);
        elm.set_center(center);
        elm.set_right(right);
        elm.set_indicator(&elm.label());
        elm.set_position(super::ActionSliderPos::Right);
        prt.add(&elm);
        elm
    }
    fn set_right(&self, value: &str) {
        self.set_part("right", value);
    }
    fn set_center(&self, value: &str) {
        self.set_part("center", value);
    }
    fn set_left(&self, value: &str) {
        self.set_part("left", value);
    }
    fn set_indicator(&self, value: &str) {
        self.set_part("indicator", value);
    }
    fn set_position(&self, value: super::ActionSliderPos) {
        unsafe { elm_actionslider_indicator_pos_set(self.as_raw(), value as Elm_Actionslider_Pos) };
    }
    fn position(&self) -> super::ActionSliderPos {
        unsafe {
            match elm_actionslider_indicator_pos_get(self.as_raw()) {
                2 => super::ActionSliderPos::Center,
                3 => super::ActionSliderPos::Right,
                _ => super::ActionSliderPos::Left,
            }
        }
    }
    fn label(&self) -> String {
        unsafe {
            let ptr = elm_actionslider_selected_label_get(self.as_raw());
            CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
    fn with_position(self, value: super::ActionSliderPos) -> Self {
        self.set_position(value);
        self
    }
}

pub trait CalendarExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_calendar_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
    fn set_selected(&self, value: super::Tm) {
        let mut tm_ = value.to_tm();
        unsafe { elm_calendar_selected_time_set(self.as_raw(), &mut tm_) };
    }
    fn selected(&self) -> super::Tm {
        let mut tm_ = super::Tm::default().to_tm();
        unsafe { elm_calendar_selected_time_get(self.as_raw(), &mut tm_) };
        super::Tm::from_tm(tm_)
    }
}

pub trait CtxpopupExt: SelectorExt {
    #[deprecated = "rse refl::Popup::new(&parent) instead"]
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_ctxpopup_add(prt.as_raw()) }).with_conf();
        elm.set_parent(&prt.window());
        elm.set_auto_hide(true);
        prt.add(&elm);
        elm
    }
    fn with_appends<F: FnMut(Self) + 'static + Clone>(self, items: &[&str], func: F) -> Self {
        for item in items {
            self.append(item, item, func.clone());
        }
        self
    }
    fn append<F: FnMut(Self) + 'static>(
        &self,
        icon_: &str,
        label_: &str,
        func: F,
    ) -> super::WidgetItem {
        let raw_ptr: *mut Box<dyn FnMut(Self)> = Box::into_raw(Box::new(Box::new(func)));
        super::WidgetItem::from_raw(unsafe {
            elm_ctxpopup_item_append(
                self.as_raw(),
                CString::new(icon_).unwrap().as_ptr(),
                super::Icon::new(self).with_standard(label_).as_raw(),
                Some(smart_cb::<Self>),
                raw_ptr as *mut c_void,
            )
        })
    }
    fn set_parent(&self, prt: &impl ContainerExt) {
        unsafe { elm_ctxpopup_hover_parent_set(self.as_raw(), prt.as_raw()) };
    }
    fn set_auto_hide(&self, value: bool) {
        unsafe { elm_ctxpopup_auto_hide_disabled_set(self.as_raw(), value as Eina_Bool) };
    }
}

pub trait NotifyExt: ElmObject {
    #[deprecated = "rse refl::Popup::new(&parent) instead"]
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_notify_add(prt.as_raw()) }).with_conf();
        elm.set_parent(&prt.window());
        prt.add(&elm);
        elm
    }
    fn dismiss(&self) {
        unsafe { elm_notify_dismiss(self.as_raw()) };
    }
    fn set_timeout(&self, value: f64) {
        unsafe { elm_notify_timeout_set(self.as_raw(), value) };
    }
    fn set_parent(&self, prt: &impl ContainerExt) {
        unsafe { elm_notify_parent_set(self.as_raw(), prt.as_raw()) };
    }
}
pub trait HoverSelExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_hoversel_add(prt.as_raw()) }).with_conf();
        elm.set_parent(&prt.window());
        prt.add(&elm);
        elm
    }
    fn add_item<F: FnMut(Self) + 'static>(
        &self,
        icon: &str,
        label: &str,
        func: F,
    ) -> super::WidgetItem {
        let raw_ptr: *mut Box<dyn FnMut(Self)> = Box::into_raw(Box::new(Box::new(func)));
        super::WidgetItem::from_raw(unsafe {
            elm_hoversel_item_add(
                self.as_raw(),
                CString::new(icon).unwrap().as_ptr(),
                CString::new(label).unwrap().as_ptr(),
                2,
                Some(smart_cb::<Self>),
                raw_ptr as *mut c_void,
            )
        })
    }
    fn with_item<F: FnMut(Self) + 'static + Clone>(self, icon: &str, label: &str, func: F) -> Self {
        self.add_item(icon, label, func.clone());
        self
    }
    fn set_parent(&self, prt: &impl ContainerExt) {
        unsafe { elm_hoversel_hover_parent_set(self.as_raw(), prt.as_raw()) };
    }
    fn clear(&self) {
        unsafe {
            elm_hoversel_hover_end(self.as_raw());
            elm_hoversel_clear(self.as_raw());
        };
    }
}

pub trait FlipSelExt: SelectorExt {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_flipselector_add(prt.as_raw()) })
            .with_conf()
            .with_interval(3.0);
        prt.add(&elm);
        elm
    }
    fn append<F: FnMut(Self) + 'static>(&self, label: &str, func: F) -> super::WidgetItem {
        let raw_ptr: *mut Box<dyn FnMut(Self)> = Box::into_raw(Box::new(Box::new(func)));
        super::WidgetItem::from_raw(unsafe {
            elm_flipselector_item_append(
                self.as_raw(),
                CString::new(label).unwrap().as_ptr(),
                Some(smart_cb::<Self>),
                raw_ptr as *mut c_void,
            )
        })
    }
    fn set_interval(&self, value: f64) {
        unsafe { elm_flipselector_first_interval_set(self.as_raw(), value) }
    }
    fn with_interval(self, value: f64) -> Self {
        self.set_interval(value);
        self
    }
}

pub trait EntryExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_entry_add(prt.as_raw()) })
            .with_conf()
            .with_single_line(true)
            .with_scrollable(true);
        prt.add(&elm);
        elm
    }
    fn with_callback<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.set_callback(EntrySignal::Changed, func);
        self
    }
    fn with_signal<F: FnMut(Self) + 'static>(self, sign: EntrySignal, func: F) -> Self {
        self.set_callback(sign, func);
        self
    }
    fn with_editable(self, value: bool) -> Self {
        self.set_editable(value);
        self
    }
    fn with_scrollable(self, value: bool) -> Self {
        self.set_scrollable(value);
        self
    }
    fn with_single_line(self, value: bool) -> Self {
        self.set_single_line(value);
        self
    }
    fn with_value(self, value: &str) -> Self {
        self.set_value(value);
        self
    }
    fn with_guide(self, guide: &str) -> Self {
        self.set_part("guide", guide);
        self
    }
    fn set_value(&self, value: &str) {
        let ctext = CString::new(value).unwrap();
        unsafe { elm_entry_entry_set(self.as_raw(), ctext.as_ptr()) };
    }
    fn set_scrollable(&self, value: bool) {
        unsafe { elm_entry_scrollable_set(self.as_raw(), value as Eina_Bool) };
    }
    fn set_menu(&self, value: bool) {
        unsafe { elm_entry_context_menu_disabled_set(self.as_raw(), value as Eina_Bool) };
    }
    fn set_password(&self, value: bool) {
        unsafe { elm_entry_password_set(self.as_raw(), value as Eina_Bool) };
    }
    fn set_editable(&self, value: bool) {
        unsafe { elm_entry_editable_set(self.as_raw(), value as Eina_Bool) };
    }
    fn set_single_line(&self, value: bool) {
        unsafe { elm_entry_single_line_set(self.as_raw(), value as Eina_Bool) };
    }
    fn set_context_menu_disabled(&self, value: bool) {
        unsafe { elm_entry_context_menu_disabled_set(self.as_raw(), value as Eina_Bool) };
    }
    fn set_file(&self, file_: &str) {
        let file = CString::new(file_).unwrap();
        unsafe {
            elm_entry_file_set(
                self.as_raw(),
                file.as_ptr(),
                Elm_Text_Format_ELM_TEXT_FORMAT_MARKUP_UTF8,
            )
        };
    }
    fn context_menu_clear(&self) {
        unsafe { elm_entry_context_menu_clear(self.as_raw()) };
    }
    fn value(&self) -> String {
        unsafe {
            let ptr = elm_entry_entry_get(self.as_raw());
            CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
    fn editable(&self) -> bool {
        unsafe { elm_entry_editable_get(self.as_raw()) != 0 }
    }
}

pub trait IconExt: ElmObject {
    fn new(parent: &impl super::ElmObject) -> Self {
        let elm = Self::from_raw(unsafe { elm_icon_add(parent.as_raw()) });
        parent.set_content(&elm, "icon");
        elm
    }
    fn with_standard(self, value: &str) -> Self {
        self.set_standard(value);
        self
    }
    fn set_standard(&self, value: &str) {
        unsafe { elm_icon_standard_set(self.as_raw(), CString::new(value).unwrap().as_ptr()) };
    }
}

pub trait ImageExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_image_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
    fn with_file(self, file: &str) -> Self {
        self.set_file(file);
        self
    }
    fn with_file_group(self, file: &str, group: &str) -> Self {
        self.set_file_group(file, group);
        self
    }
    fn set_file(&self, file: &str) -> bool {
        let cfile = CString::new(file).unwrap();
        unsafe { elm_image_file_set(self.as_raw(), cfile.as_ptr(), std::ptr::null()) != 0 }
    }
    fn set_file_group(&self, file: &str, group: &str) -> bool {
        let cfile = CString::new(file).unwrap();
        let cgroup = CString::new(group).unwrap();
        unsafe { elm_image_file_set(self.as_raw(), cfile.as_ptr(), cgroup.as_ptr()) != 0 }
    }
    fn set_prescale(&self, size: i32) {
        unsafe { elm_image_prescale_set(self.as_raw(), size) };
    }
    fn prescale(&self) -> i32 {
        unsafe { elm_image_prescale_get(self.as_raw()) }
    }
    fn set_smooth(&self, value: bool) {
        unsafe { elm_image_smooth_set(self.as_raw(), value as Eina_Bool) };
    }
    fn smooth(&self) -> bool {
        unsafe { elm_image_smooth_get(self.as_raw()) != 0 }
    }
    fn set_animated(&self, value: bool) {
        unsafe { elm_image_animated_set(self.as_raw(), value as Eina_Bool) };
    }
    fn animated(&self) -> bool {
        unsafe { elm_image_animated_get(self.as_raw()) != 0 }
    }
    fn set_animated_play(&self, value: bool) {
        unsafe { elm_image_animated_play_set(self.as_raw(), value as Eina_Bool) };
    }
    fn animated_play(&self) -> bool {
        unsafe { elm_image_animated_play_get(self.as_raw()) != 0 }
    }
    fn animated_available(&self) -> bool {
        unsafe { elm_image_animated_available_get(self.as_raw()) != 0 }
    }
    fn set_editable(&self, value: bool) {
        unsafe { elm_image_editable_set(self.as_raw(), value as Eina_Bool) };
    }
    fn editable(&self) -> bool {
        unsafe { elm_image_editable_get(self.as_raw()) != 0 }
    }
    fn set_fill_outside(&self, value: bool) {
        unsafe { elm_image_fill_outside_set(self.as_raw(), value as Eina_Bool) };
    }
    fn fill_outside(&self) -> bool {
        unsafe { elm_image_fill_outside_get(self.as_raw()) != 0 }
    }
    fn set_orient(&self, orient: super::ImageOrient) {
        unsafe { elm_image_orient_set(self.as_raw(), orient as u32) };
    }
    fn set_resizable(&self, up: bool, down: bool) {
        unsafe { elm_image_resizable_set(self.as_raw(), up as Eina_Bool, down as Eina_Bool) };
    }
    fn image_size(&self) -> (i32, i32) {
        let (mut w, mut h) = (0i32, 0i32);
        unsafe { elm_image_object_size_get(self.as_raw(), &mut w, &mut h) };
        (w, h)
    }
}

pub trait IndexExt: LayoutExt {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_index_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
    fn append<F: FnMut(Self) + 'static>(&self, letter: &str, func: F) -> super::WidgetItem {
        let raw_ptr: *mut Box<dyn FnMut(Self)> = Box::into_raw(Box::new(Box::new(func)));
        super::WidgetItem::from_raw(unsafe {
            elm_index_item_append(
                self.as_raw(),
                CString::new(letter).unwrap().as_ptr(),
                Some(smart_cb::<Self>),
                raw_ptr as *mut c_void,
            )
        })
    }
    fn with_item<F: FnMut(Self) + 'static>(self, letter: &str, func: F) -> Self {
        self.append(letter, func);
        self
    }
    fn add(&self, letter: &str) -> super::WidgetItem {
        self.append(letter, |_| {})
    }
    fn add_items(&self, items: &[&str]) {
        for item in items {
            self.add(item);
        }
    }
    fn with_items(self, items: &[&str]) -> Self {
        self.add_items(items);
        self
    }
    fn selected(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_index_selected_item_get(self.as_raw(), 0) })
    }
    fn selected_at_level(&self, level: i32) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_index_selected_item_get(self.as_raw(), level) })
    }
    fn clear(&self) {
        unsafe { elm_index_item_clear(self.as_raw()) };
    }
    fn level_go(&self, level: i32) {
        unsafe { elm_index_level_go(self.as_raw(), level) };
    }
    fn set_item_level(&self, level: i32) {
        unsafe { elm_index_item_level_set(self.as_raw(), level) };
    }
    fn item_level(&self) -> i32 {
        unsafe { elm_index_item_level_get(self.as_raw()) }
    }
    fn horizontal(&self) -> bool {
        unsafe { elm_index_horizontal_get(self.as_raw()) != 0 }
    }
    fn set_autohide_disabled(&self, value: bool) {
        unsafe { elm_index_autohide_disabled_set(self.as_raw(), value as Eina_Bool) };
    }
    fn autohide_disabled(&self) -> bool {
        unsafe { elm_index_autohide_disabled_get(self.as_raw()) != 0 }
    }
    fn set_indicator_disabled(&self, value: bool) {
        unsafe { elm_index_indicator_disabled_set(self.as_raw(), value as Eina_Bool) };
    }
    fn indicator_disabled(&self) -> bool {
        unsafe { elm_index_indicator_disabled_get(self.as_raw()) != 0 }
    }
    fn set_delay_change_time(&self, value: f64) {
        unsafe { elm_index_delay_change_time_set(self.as_raw(), value) };
    }
    fn delay_change_time(&self) -> f64 {
        unsafe { elm_index_delay_change_time_get(self.as_raw()) }
    }
}

pub trait SeparatorExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_separator_add(prt.as_raw()) })
            .with_conf()
            .with_horizontal(true);
        prt.add(&elm);
        elm
    }
    fn with_horizontal(self, value: bool) -> Self {
        self.set_horizontal(value);
        self
    }
    fn set_horizontal(&self, value: bool) {
        unsafe { elm_separator_horizontal_set(self.as_raw(), value as Eina_Bool) };
        self.set_weight(value, !value);
    }
}

pub trait LayoutExt: ElmObject {
    fn with_file(self, file: &str, group: &str) -> Self {
        self.set_file(file, group);
        self
    }
    fn with_theme(self, klass: &str, group: &str, style: &str) -> Self {
        self.set_theme(klass, group, style);
        self
    }
    fn set_file(&self, file: &str, group: &str) {
        let cfile = CString::new(file).unwrap();
        let cgroup = CString::new(group).unwrap();
        unsafe { elm_layout_file_set(self.as_raw(), cfile.as_ptr(), cgroup.as_ptr()) };
    }
    fn set_theme(&self, klass: &str, group: &str, style: &str) {
        let cklass = CString::new(klass).unwrap();
        let cgroup = CString::new(group).unwrap();
        let cstyle = CString::new(style).unwrap();
        unsafe {
            elm_layout_theme_set(
                self.as_raw(),
                cklass.as_ptr(),
                cgroup.as_ptr(),
                cstyle.as_ptr(),
            )
        };
    }
    fn sizing_eval(&self) {
        unsafe { elm_layout_sizing_eval(self.as_raw()) };
    }
    fn freeze(&self) -> i32 {
        unsafe { elm_layout_freeze(self.as_raw()) }
    }
    fn thaw(&self) -> i32 {
        unsafe { elm_layout_thaw(self.as_raw()) }
    }
}

pub trait GridExt: ContainerExt {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_grid_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
    fn set_virtual_size(&self, w: i32, h: i32) {
        unsafe { elm_grid_size_set(self.as_raw(), w, h) };
    }
    fn pack(&self, subobj: &impl ElmObject, x: i32, y: i32, w: i32, h: i32) {
        unsafe { elm_grid_pack(self.as_raw(), subobj.as_raw(), x, y, w, h) };
    }
    fn clear(&self, clear_items: bool) {
        unsafe { elm_grid_clear(self.as_raw(), clear_items as Eina_Bool) };
    }
    fn set_pack(&self, subobj: &impl ElmObject, x: i32, y: i32, w: i32, h: i32) {
        unsafe { elm_grid_pack_set(subobj.as_raw(), x, y, w, h) };
    }
}

pub trait TableExt: ContainerExt {
    #[deprecated = "use efltk::Box::new(&parent) instead"]
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_table_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
    fn pack(&self, subobj: &impl ElmObject, col: i32, row: i32, colspan: i32, rowspan: i32) {
        unsafe { elm_table_pack(self.as_raw(), subobj.as_raw(), col, row, colspan, rowspan) };
    }
    fn with_pack(
        self,
        subobj: &impl ElmObject,
        col: i32,
        row: i32,
        colspan: i32,
        rowspan: i32,
    ) -> Self {
        self.pack(subobj, col, row, colspan, rowspan);
        self
    }
    fn set_homogeneous(&self, value: bool) {
        unsafe { elm_table_homogeneous_set(self.as_raw(), value as Eina_Bool) };
    }
    fn with_homogeneous(self, value: bool) -> Self {
        self.set_homogeneous(value);
        self
    }
    fn set_padding(&self, horizontal: i32, vertical: i32) {
        unsafe { elm_table_padding_set(self.as_raw(), horizontal, vertical) };
    }
    fn with_padding(self, horizontal: i32, vertical: i32) -> Self {
        self.set_padding(horizontal, vertical);
        self
    }
    fn set_cell_align(&self, horizontal: f64, vertical: f64) {
        unsafe { elm_table_align_set(self.as_raw(), horizontal, vertical) };
    }
    fn with_cell_align(self, horizontal: f64, vertical: f64) -> Self {
        self.set_cell_align(horizontal, vertical);
        self
    }
    fn clear(&self, clear_items: bool) {
        unsafe { elm_table_clear(self.as_raw(), clear_items as Eina_Bool) };
    }
}

pub trait ListExt: SelectorExt {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe {
            let ptr = elm_list_add(prt.as_raw());
            elm_list_scroller_policy_set(
                ptr,
                Elm_Scroller_Policy_ELM_SCROLLER_POLICY_AUTO,
                Elm_Scroller_Policy_ELM_SCROLLER_POLICY_AUTO,
            );
            ptr
        })
        .with_mode(super::ListMode::Expand)
        .with_conf();
        prt.add(&elm);
        elm
    }
    fn add_item<F: FnMut(Self) + 'static>(
        &self,
        icon_: &str,
        label_: &str,
        func: F,
    ) -> super::WidgetItem {
        let raw_ptr: *mut Box<dyn FnMut(Self)> = Box::into_raw(Box::new(Box::new(func)));
        super::WidgetItem::from_raw(unsafe {
            elm_list_item_append(
                self.as_raw(),
                CString::new(icon_).unwrap().as_ptr(),
                super::Icon::new(self).with_standard(label_).as_raw(),
                std::ptr::null_mut(),
                Some(smart_cb::<Self>),
                raw_ptr as *mut c_void,
            )
        })
    }
    fn set_mode(&self, mode: super::ListMode) {
        unsafe { elm_list_mode_set(self.as_raw(), mode as Elm_List_Mode) };
    }
    fn with_mode(self, mode: super::ListMode) -> Self {
        self.set_mode(mode);
        self
    }
}

pub trait FrameExt: ElmObject {
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_frame_add(parent.as_raw()) })
            .with_autocollapse(true)
            .with_conf();
        parent.add(&elm);
        elm
    }
    fn with_callback<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.set_callback(TriggerSignal::Clicked, func);
        self
    }
    fn with_signal<F: FnMut(Self) + 'static>(self, sign: TriggerSignal, func: F) -> Self {
        self.set_callback(sign, func);
        self
    }
    fn with_autocollapse(self, value: bool) -> Self {
        self.set_autocollapse(value);
        self
    }
    fn set_autocollapse(&self, value: bool) {
        unsafe { elm_frame_autocollapse_set(self.as_raw(), value as Eina_Bool) };
    }
    fn set_collapse(&self, value: bool) {
        unsafe { elm_frame_collapse_set(self.as_raw(), value as Eina_Bool) };
    }
}

pub trait NaviframeExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_naviframe_add(prt.as_raw()) })
            .with_prev(false)
            .with_conf();
        prt.add(&elm);
        elm
    }
    fn set_prev(&self, value: bool) {
        unsafe { elm_naviframe_prev_btn_auto_pushed_set(self.as_raw(), value as Eina_Bool) };
    }
    fn with_prev(self, value: bool) -> Self {
        self.set_prev(value);
        self
    }
    fn set_content_preserve_on_pop(&self, value: bool) {
        unsafe { elm_naviframe_content_preserve_on_pop_set(self.as_raw(), value as Eina_Bool) };
    }
    fn push(&self, child: &impl super::ElmObject) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe {
            let item = elm_naviframe_item_push(
                self.as_raw(),
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                child.as_raw(),
                std::ptr::null(),
            );
            elm_naviframe_item_title_visible_set(item, 0);
            item
        })
    }
}

pub trait PanelExt: ElmObject {
    #[deprecated = "rse refl::Popup::new(&parent) instead"]
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_panel_add(prt.as_raw()) })
            .with_orient(PanelOrient::Bottom)
            .with_scrollable(true)
            .with_hidden(true)
            .with_conf();
        prt.add(&elm);
        elm
    }
    fn with_orient(self, orient: PanelOrient) -> Self {
        self.set_orient(orient);
        self
    }
    fn with_hidden(self, hidden: bool) -> Self {
        self.set_hidden(hidden);
        self
    }
    fn with_scrollable(self, scrollable: bool) -> Self {
        self.set_scrollable(scrollable);
        self
    }
    fn set_hidden(&self, hidden: bool) {
        unsafe { elm_panel_hidden_set(self.as_raw(), hidden as Eina_Bool) };
    }
    fn set_scrollable(&self, scrollable: bool) {
        unsafe { elm_panel_scrollable_set(self.as_raw(), scrollable as Eina_Bool) };
    }
    fn set_orient(&self, orient: PanelOrient) {
        unsafe { elm_panel_orient_set(self.as_raw(), orient as u32) };
    }
}

pub trait PanesExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_panes_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
    fn set_horizontal(&self, value: bool) {
        unsafe { elm_panes_horizontal_set(self.as_raw(), value as Eina_Bool) };
        self.set_weight(value, !value);
    }
    fn set_fixed_size(&self, left: f64) {
        if (0.0..1.0).contains(&left) {
            unsafe {
                elm_panes_content_left_size_set(self.as_raw(), left);
                elm_panes_content_right_size_set(self.as_raw(), 1.0 - left);
                elm_panes_fixed_set(self.as_raw(), true as Eina_Bool);
            };
        }
    }
    fn with_horizontal(self, value: bool) -> Self {
        self.set_horizontal(value);
        self
    }
    fn with_fixed_size(self, left: f64) -> Self {
        self.set_fixed_size(left);
        self
    }
}

pub trait PopupExt: ContainerExt + Clone
where
    Self: 'static,
{
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_popup_add(prt.window().as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
    fn set_message(&self, icon: &str, title: &str, text: &str) {
        self.set_content(&super::Icon::new(self).with_standard(icon), "title,icon");
        self.set_part("title,text", title);
        self.set_part("default", text);
    }
    fn with_message(self, icon: &str, title: &str, text: &str) -> Self {
        self.set_message(icon, title, text);
        self
    }
    fn set_timeout(&self, timeout: f64) {
        unsafe { elm_popup_timeout_set(self.as_raw(), timeout) };
    }
    fn set_close(&self) {
        let but = super::Button::new(self)
            .with_text("Close")
            .with_icon("close")
            .with_callback({
                let elm = self.clone();
                move |_| elm.dismiss()
            });
        self.set_content(&but, "button3");
    }
    fn set_ok<F: FnMut(super::Button) + 'static>(&self, mut func: F) {
        let but = super::Button::new(self).with_text("Ok").with_callback({
            let elm = self.clone();
            move |wgt| {
                func(wgt);
                elm.dismiss();
            }
        });
        self.set_content(&but, "button1");
    }
    fn set_cancel<F: FnMut(super::Button) + 'static>(&self, mut func: F) {
        let but = super::Button::new(self).with_text("Cancel").with_callback({
            let elm = self.clone();
            move |wgt| {
                func(wgt);
                elm.dismiss();
            }
        });
        self.set_content(&but, "button2");
    }
    fn with_timeout(self, timeout: f64) -> Self {
        match timeout > 0.0 {
            true => self.set_timeout(timeout),
            false => self.set_close(),
        }
        self
    }
    fn with_close(self) -> Self {
        self.set_close();
        self
    }
    fn with_ok<F: FnMut(super::Button) + 'static>(self, func: F) -> Self {
        self.set_ok(func);
        self
    }
    fn with_cancel<F: FnMut(super::Button) + 'static>(self, func: F) -> Self {
        self.set_cancel(func);
        self
    }
    fn dismiss(&self) {
        unsafe { elm_popup_dismiss(self.as_raw()) };
    }
}

pub trait ProgressBarExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_progressbar_add(prt.as_raw()) })
            .with_conf()
            .with_horizontal(true);
        prt.add(&elm);
        elm
    }
    fn value(&self) -> f64 {
        unsafe { elm_progressbar_value_get(self.as_raw()) }
    }
    fn set_horizontal(&self, value: bool) {
        unsafe { elm_progressbar_horizontal_set(self.as_raw(), value as Eina_Bool) };
        self.set_weight(value, !value);
    }
    fn set_value(&self, value: f64) {
        unsafe { elm_progressbar_value_set(self.as_raw(), value) };
    }
    fn set_unit_format(&self, value: &str) {
        let ctext = CString::new(value).unwrap();
        unsafe { elm_progressbar_unit_format_set(self.as_raw(), ctext.as_ptr()) };
    }
    fn with_horizontal(self, value: bool) -> Self {
        self.set_horizontal(value);
        self
    }
    fn with_format(self, value: &str) -> Self {
        self.set_unit_format(value);
        self
    }
    fn with_callback<F: FnMut(Self) + 'static>(self, sign: ProgressBarSignal, func: F) -> Self {
        self.set_callback(sign, func);
        self
    }
}

pub trait RadioExt: ElmObject {
    fn new<F: FnMut(Self) + 'static + Clone>(
        parent: &impl ContainerExt,
        items: &[&str],
        func: F,
    ) -> Self {
        let elm = Self::item(parent);
        for (idx, item) in items.iter().enumerate() {
            if idx == 0 {
                elm.set_text(item);
                elm.set_icon(item);
                elm.set_callback(RadioSignal::Changed, func.clone());
            } else {
                let child = Self::item(parent)
                    .with_text(item)
                    .with_icon(item)
                    .with_callback(func.clone());
                elm.add_group(&child);
            }
        }
        elm
    }
    fn item(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_radio_add(prt.as_raw()) }).with_conf();
        elm.set_state_value(0);
        prt.add(&elm);
        elm
    }
    fn with_callback<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.set_callback(RadioSignal::Changed, func);
        self
    }
    fn with_signal<F: FnMut(Self) + 'static>(self, sign: RadioSignal, func: F) -> Self {
        self.set_callback(sign, func);
        self
    }
    fn with_add(self, parent: &impl ContainerExt, child: &str) -> Self {
        self.add(parent, child);
        self
    }
    fn add(&self, prt: &impl ContainerExt, child: &str) {
        self.add_group(&Self::item(prt).with_text(child));
    }
    fn add_group(&self, child: &Self) {
        child.set_state_value(self.value() + 1);
        unsafe { elm_radio_group_add(child.as_raw(), self.as_raw()) };
        self.set_value(child.state_value());
    }
    fn set_state_value(&self, value: i32) {
        unsafe { elm_radio_state_value_set(self.as_raw(), value) };
    }
    fn value(&self) -> i32 {
        unsafe { elm_radio_value_get(self.as_raw()) as i32 }
    }
    fn state_value(&self) -> i32 {
        unsafe { elm_radio_state_value_get(self.as_raw()) as i32 }
    }
    fn set_value(&self, value: i32) {
        unsafe { elm_radio_value_set(self.as_raw(), value) };
    }
}

pub trait PrefsExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_prefs_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
    fn with_file(self, file: &str) -> Self {
        self.set_file(file);
        self
    }
    fn with_file_group(self, file: &str, group: &str) -> Self {
        self.set_file_group(file, group);
        self
    }
    fn set_file(&self, file: &str) -> bool {
        let cfile = CString::new(file).unwrap();
        unsafe { elm_prefs_file_set(self.as_raw(), cfile.as_ptr(), std::ptr::null()) != 0 }
    }
    fn set_file_group(&self, file: &str, group: &str) -> bool {
        let cfile = CString::new(file).unwrap();
        let cgroup = CString::new(group).unwrap();
        unsafe { elm_prefs_file_set(self.as_raw(), cfile.as_ptr(), cgroup.as_ptr()) != 0 }
    }
    fn set_autosave(&self, value: bool) {
        unsafe { elm_prefs_autosave_set(self.as_raw(), value as Eina_Bool) };
    }
    fn autosave(&self) -> bool {
        unsafe { elm_prefs_autosave_get(self.as_raw()) != 0 }
    }
    fn reset(&self, mode: super::PrefsResetMode) {
        unsafe { elm_prefs_reset(self.as_raw(), mode as u32) };
    }
    fn set_item_disabled(&self, name: &str, value: bool) {
        let cname = CString::new(name).unwrap();
        unsafe { elm_prefs_item_disabled_set(self.as_raw(), cname.as_ptr(), value as Eina_Bool) };
    }
    fn item_disabled(&self, name: &str) -> bool {
        let cname = CString::new(name).unwrap();
        unsafe { elm_prefs_item_disabled_get(self.as_raw(), cname.as_ptr()) != 0 }
    }
    fn item_swallow(&self, name: &str, child: &impl ElmObject) -> bool {
        let cname = CString::new(name).unwrap();
        unsafe { elm_prefs_item_swallow(self.as_raw(), cname.as_ptr(), child.as_raw()) != 0 }
    }
    fn set_item_editable(&self, name: &str, value: bool) {
        let cname = CString::new(name).unwrap();
        unsafe { elm_prefs_item_editable_set(self.as_raw(), cname.as_ptr(), value as Eina_Bool) };
    }
    fn item_editable(&self, name: &str) -> bool {
        let cname = CString::new(name).unwrap();
        unsafe { elm_prefs_item_editable_get(self.as_raw(), cname.as_ptr()) != 0 }
    }
    fn set_item_visible(&self, name: &str, value: bool) {
        let cname = CString::new(name).unwrap();
        unsafe { elm_prefs_item_visible_set(self.as_raw(), cname.as_ptr(), value as Eina_Bool) };
    }
    fn item_visible(&self, name: &str) -> bool {
        let cname = CString::new(name).unwrap();
        unsafe { elm_prefs_item_visible_get(self.as_raw(), cname.as_ptr()) != 0 }
    }
}

pub trait VideoExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_video_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
    fn with_file(self, filename: &str) -> Self {
        self.set_file(filename);
        self
    }
    fn set_file(&self, filename: &str) -> bool {
        let cfilename = CString::new(filename).unwrap();
        unsafe { elm_video_file_set(self.as_raw(), cfilename.as_ptr()) != 0 }
    }
    fn set_audio_level(&self, value: f64) {
        unsafe { elm_video_audio_level_set(self.as_raw(), value) };
    }
    fn audio_level(&self) -> f64 {
        unsafe { elm_video_audio_level_get(self.as_raw()) }
    }
    fn set_audio_mute(&self, value: bool) {
        unsafe { elm_video_audio_mute_set(self.as_raw(), value as Eina_Bool) };
    }
    fn audio_mute(&self) -> bool {
        unsafe { elm_video_audio_mute_get(self.as_raw()) != 0 }
    }
    fn play_length(&self) -> f64 {
        unsafe { elm_video_play_length_get(self.as_raw()) }
    }
    fn seekable(&self) -> bool {
        unsafe { elm_video_is_seekable_get(self.as_raw()) != 0 }
    }
    fn set_play_position(&self, value: f64) {
        unsafe { elm_video_play_position_set(self.as_raw(), value) };
    }
    fn play_position(&self) -> f64 {
        unsafe { elm_video_play_position_get(self.as_raw()) }
    }
    fn playing(&self) -> bool {
        unsafe { elm_video_is_playing_get(self.as_raw()) != 0 }
    }
    fn play(&self) {
        unsafe { elm_video_play(self.as_raw()) };
    }
    fn pause(&self) {
        unsafe { elm_video_pause(self.as_raw()) };
    }
    fn stop(&self) {
        unsafe { elm_video_stop(self.as_raw()) };
    }
}

pub trait PlayerExt: LayoutExt + ContainerExt {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_player_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
    fn with_video(self, video: &impl VideoExt) -> Self {
        self.set_video(video);
        self
    }
    fn set_video(&self, video: &impl VideoExt) {
        self.set_content(video, "video");
        video.show();
    }
}

pub trait ScrollerExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_scroller_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
    fn with_policy(self, policy_h: super::ScrollerPolicy, policy_v: super::ScrollerPolicy) -> Self {
        self.set_policy(policy_h, policy_v);
        self
    }
    fn with_bounce(self, horizontal: bool, vertical: bool) -> Self {
        self.set_bounce(horizontal, vertical);
        self
    }
    fn with_page_snapping(self, horizontal: bool, vertical: bool) -> Self {
        self.set_page_snapping(horizontal, vertical);
        self
    }
    fn set_policy(&self, policy_h: super::ScrollerPolicy, policy_v: super::ScrollerPolicy) {
        unsafe { elm_scroller_policy_set(self.as_raw(), policy_h as u32, policy_v as u32) };
    }
    fn policy(&self) -> (super::ScrollerPolicy, super::ScrollerPolicy) {
        let (mut policy_h, mut policy_v) = (0u32, 0u32);
        unsafe { elm_scroller_policy_get(self.as_raw(), &mut policy_h, &mut policy_v) };
        (
            match policy_h {
                1 => super::ScrollerPolicy::On,
                2 => super::ScrollerPolicy::Off,
                3 => super::ScrollerPolicy::Last,
                _ => super::ScrollerPolicy::Auto,
            },
            match policy_v {
                1 => super::ScrollerPolicy::On,
                2 => super::ScrollerPolicy::Off,
                3 => super::ScrollerPolicy::Last,
                _ => super::ScrollerPolicy::Auto,
            },
        )
    }
    fn set_bounce(&self, horizontal: bool, vertical: bool) {
        unsafe {
            elm_scroller_bounce_set(
                self.as_raw(),
                horizontal as Eina_Bool,
                vertical as Eina_Bool,
            )
        };
    }
    fn bounce(&self) -> (bool, bool) {
        let (mut h, mut v) = (0 as Eina_Bool, 0 as Eina_Bool);
        unsafe { elm_scroller_bounce_get(self.as_raw(), &mut h, &mut v) };
        (h != 0, v != 0)
    }
    fn set_page_snapping(&self, horizontal: bool, vertical: bool) {
        unsafe {
            elm_scroller_page_snap_set(
                self.as_raw(),
                horizontal as Eina_Bool,
                vertical as Eina_Bool,
            )
        };
    }
    fn page_snapping(&self) -> (bool, bool) {
        let (mut h, mut v) = (0 as Eina_Bool, 0 as Eina_Bool);
        unsafe { elm_scroller_page_snap_get(self.as_raw(), &mut h, &mut v) };
        (h != 0, v != 0)
    }
    fn scroll_region(&self) -> (i32, i32, i32, i32) {
        let (mut x, mut y, mut w, mut h) = (0i32, 0i32, 0i32, 0i32);
        unsafe { elm_scroller_region_get(self.as_raw(), &mut x, &mut y, &mut w, &mut h) };
        (x, y, w, h)
    }
    fn last_page(&self) -> (i32, i32) {
        let (mut h, mut v) = (0i32, 0i32);
        unsafe { elm_scroller_last_page_get(self.as_raw(), &mut h, &mut v) };
        (h, v)
    }
}

pub trait SegmentControlExt: SelectorExt {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_segment_control_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
}

pub trait ToolBarExt: SelectorExt {
    #[deprecated = "use efltk::SegmentControl::new(&parent) instead"]
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_toolbar_add(prt.as_raw()) }).with_conf();
        elm.set_shrink_mode(super::Shrink::Menu);
        prt.add(&elm);
        elm
    }
    fn add_item<F: FnMut(Self) + 'static>(
        &self,
        icon: &str,
        label: &str,
        func: F,
    ) -> super::WidgetItem {
        let raw_ptr: *mut Box<dyn FnMut(Self)> = Box::into_raw(Box::new(Box::new(func)));
        super::WidgetItem::from_raw(unsafe {
            elm_toolbar_item_append(
                self.as_raw(),
                CString::new(icon).unwrap().as_ptr(),
                CString::new(label).unwrap().as_ptr(),
                Some(smart_cb::<Self>),
                raw_ptr as *mut c_void,
            )
        })
    }
    fn with_homogeneous(self, value: bool) -> Self {
        unsafe { elm_toolbar_homogeneous_set(self.as_raw(), value as Eina_Bool) };
        self
    }
    fn set_shrink_mode(&self, value: super::Shrink) {
        unsafe { elm_toolbar_shrink_mode_set(self.as_raw(), value as u32) };
    }
}

pub trait WindowExt: ElmObject {
    fn new(id: &str, title: &str) -> Self {
        Self::from_raw(unsafe {
            elm_win_util_standard_add(
                CString::new(id).unwrap().as_ptr(),
                CString::new(title).unwrap().as_ptr(),
            )
        })
        .with_min_size(400, 640)
        .with_center(true)
        .with_autodel(true)
    }
    fn with_center(self, value: bool) -> Self {
        self.set_center(value, value);
        self
    }
    fn with_autodel(self, value: bool) -> Self {
        self.set_autodel(value);
        self
    }
    fn set_center(&self, h: bool, v: bool) {
        unsafe { elm_win_center(self.as_raw(), h as Eina_Bool, v as Eina_Bool) }
    }
    fn set_autodel(&self, value: bool) {
        unsafe {
            elm_win_autodel_set(self.as_raw(), value as Eina_Bool);
        }
    }
    fn set_type(&self, value: super::WinType) {
        unsafe {
            elm_win_type_set(self.as_raw(), value as Elm_Win_Type);
        }
    }
}

pub trait Update<T>
where
    Self: 'static,
{
    fn update(&self, value: T);
}

impl Update<i32> for super::Radio {
    fn update(&self, value: i32) {
        if self.value() != value {
            self.set_value(value);
        };
    }
}

impl Update<&String> for super::Entry {
    fn update(&self, value: &String) {
        if !self.focus() && self.text() != *value {
            self.set_text(value);
        };
    }
}

impl<T: RangerExt + 'static> Update<f64> for T {
    fn update(&self, value: f64) {
        if self.value() != value {
            self.set_value(value);
        };
    }
}

impl<T: SelectorExt + 'static> Update<u32> for T {
    fn update(&self, value: u32) {
        if !self.focus() && (0..self.lenght()).contains(&value) {
            self.set_value(value);
        };
    }
}

impl<T: SelectorExt + 'static> Update<(Vec<String>, u32)> for T {
    fn update(&self, value: (Vec<String>, u32)) {
        if !self.focus() && self.lenght() != (value.0.len() as u32) {
            self.clear();
            for item in &value.0 {
                self.add(item);
            }
            self.update(value.1);
        }
    }
}

pub trait Component: Default + 'static {
    type Event: 'static;
    type State: Default + 'static;
    fn handle(msg: Self::Event, model: &mut Self::State, sender: Sender<Self::Event>) -> bool;
    fn update(&self, model: &Self::State);
    fn view(&mut self, parent: &impl ContainerExt, sender: Sender<Self::Event>);
    fn mount(prt: &impl ContainerExt) {
        let (sender, receiver) = channel::<Self::Event>();
        let mut page = Self::default();
        let mut model = Self::State::default();
        page.view(prt, sender.clone());
        page.update(&model);
        super::Timer::new(0.02, move || {
            if let Ok(msg) = receiver.try_recv()
                && Self::handle(msg, &mut model, sender.clone())
            {
                page.update(&model);
            };
            true
        });
    }
    fn run(title: &str) {
        run(move || {
            let win = super::Window::new("Main", title);
            Self::mount(&win);
            win
        });
    }
}
