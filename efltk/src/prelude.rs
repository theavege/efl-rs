pub use std::sync::mpsc::Sender;
use {
    efltk_sys::*,
    std::{
        ffi::{CStr, CString, c_void},
        ptr::NonNull,
        sync::mpsc::channel,
    },
};

#[derive(Default)]
pub enum Signal {
    #[default]
    Changed,
    Clicked,
    Selected,
    Unfocused,
}

impl Signal {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Changed => "changed",
            Self::Clicked => "clicked",
            Self::Selected => "selected",
            Self::Unfocused => "unfocused",
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

impl super::Timer {
    pub fn new<F: FnMut() -> bool + 'static>(timeout: f64, func: F) -> Self {
        let raw_ptr: *mut Box<EcoreCb> = Box::into_raw(Box::new(Box::new(func)));
        Self::from(unsafe { ecore_timer_add(timeout, Some(ecore_task_cb), raw_ptr as *mut c_void) })
    }
    pub fn as_ptr(&self) -> *mut Ecore_Timer {
        self.0.expect("Empty Ecore_Timer!").as_ptr()
    }
}

impl From<*mut Ecore_Timer> for super::Timer {
    fn from(obj: *mut Ecore_Timer) -> Self {
        Self(NonNull::new(obj))
    }
}

impl super::EventHandler {
    pub fn as_ptr(&self) -> *mut Ecore_Event_Handler {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    pub fn new<F: FnMut() -> bool + 'static>(type_: i32, func: F) -> Self {
        let raw_ptr: *mut Box<EcoreCb> = Box::into_raw(Box::new(Box::new(func)));
        Self::from(unsafe {
            ecore_event_handler_add(type_, Some(ecore_event_handler_cb), raw_ptr as *mut c_void)
        })
    }
    pub fn del(&self) {
        unsafe { ecore_event_handler_del(self.as_ptr()) };
    }
}

impl From<*mut Ecore_Event_Handler> for super::EventHandler {
    fn from(obj: *mut Ecore_Event_Handler) -> Self {
        Self(NonNull::new(obj))
    }
}

type EcoreCb = dyn FnMut() -> bool;

pub(crate) unsafe extern "C" fn smart_cb<T: WidgetExt>(
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

impl super::WidgetItem {
    pub fn as_raw(&self) -> *mut Evas_Object {
        self.0.expect("Empty Evas_Object!").as_ptr()
    }
    pub fn from_raw(obj: *mut Evas_Object) -> Self {
        Self(NonNull::new(obj))
    }
    pub fn text(&self) -> String {
        unsafe {
            let ptr = elm_object_item_part_text_get(self.as_raw(), std::ptr::null());
            if !ptr.is_null() {
                CStr::from_ptr(ptr).to_string_lossy().into_owned()
            } else {
                String::new()
            }
        }
    }
    pub fn del(&self) {
        unsafe { elm_object_item_del(self.as_raw()) };
    }
}
pub trait WidgetExt: Sized {
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
    fn parent<T: WidgetExt>(&self) -> T {
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
    fn with_text(self, text: &str) -> Self {
        self.set_text(text);
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
    fn with_content(self, obj: &impl WidgetExt, value: &str) -> Self {
        self.set_content(obj, value);
        self
    }
    fn set_content(&self, obj: &impl WidgetExt, value: &str) {
        let ctext = CString::new(value).unwrap();
        unsafe { elm_object_part_content_set(self.as_raw(), ctext.as_ptr(), obj.as_raw()) }
        obj.show();
    }
    fn content(&self, value: &str) -> Option<super::WidgetItem> {
        let ctext = CString::new(value).unwrap();
        let ptr = unsafe { elm_object_part_content_get(self.as_raw(), ctext.as_ptr()) };
        match ptr.is_null() {
            true => None,
            false => Some(super::WidgetItem::from_raw(ptr)),
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
    fn with_signal<F: FnMut(Self) + 'static>(self, sign: Signal, func: F) -> Self {
        self.set_callback(sign, func);
        self
    }
    fn window(&self) -> super::Window {
        super::Window::from_raw(unsafe { elm_win_get(self.as_raw()) })
    }
    fn with_callback<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.set_callback(Signal::Changed, func);
        self
    }
    fn set_callback<F: FnMut(Self) + 'static>(&self, sign: Signal, func: F) {
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
    fn call_signal(&self, sign: Signal) {
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

pub trait LabelExt: WidgetExt {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe {
            let ptr = elm_label_add(prt.as_raw());
            elm_object_style_set(ptr, CString::new("marker").unwrap().as_ptr());
            elm_label_line_wrap_set(ptr, Elm_Wrap_Type_ELM_WRAP_WORD);
            ptr
        })
        .with_conf();
        prt.add(&elm);
        elm
    }
}
pub trait SpinnerExt: RangerExt {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_spinner_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
}

pub trait ButtonExt: WidgetExt {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_button_add(prt.as_raw()) })
            .with_conf()
            .with_weight(true, false)
            .with_signal(Signal::Clicked, |wgt| wgt.call_signal(Signal::Changed));
        prt.add(&elm);
        elm
    }
    fn with_menu(prt: &impl ContainerExt, menu: super::Menu) -> Self {
        Self::new(prt).with_cursor(Cursor::Hand1).with_callback({
            move |wgt| {
                let pos = wgt.geometry();
                menu.open(pos.0, pos.1 + pos.3);
            }
        })
    }
    fn do_callback(&self) {
        self.call_signal(Signal::Clicked);
    }
}

pub trait ContainerExt: WidgetExt {
    fn add(&self, child: &impl WidgetExt) {
        self.set_content(child, "default");
        child.show();
    }
    fn inside(&self, mut func: impl FnMut(&Self)) {
        func(self);
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
    fn add_item(&self, item: &impl WidgetExt) {
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
    fn popup(prt: &impl WidgetExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_menu_add(prt.window().as_raw()) })
            .with_conf()
            .with_signal(Signal::Selected, |wgt| wgt.call_signal(Signal::Changed));
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

pub trait FileEntryExt: WidgetExt {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_fileselector_entry_add(prt.as_raw()) });
        elm.set_inwin(true);
        elm.set_expandable(false);
        elm.set_folder_only(false);
        elm.set_path(
            &std::env::var(match cfg!(target_os = "windows") {
                true => "%HOMEPATH%",
                false => "HOME",
            })
            .unwrap(),
        );
        prt.add(&elm);
        elm
    }
    fn set_inwin(&self, mode: bool) {
        unsafe { elm_fileselector_entry_inwin_mode_set(self.as_raw(), mode as Eina_Bool) };
    }
    fn set_expandable(&self, mode: bool) {
        unsafe { elm_fileselector_entry_expandable_set(self.as_raw(), mode as Eina_Bool) };
    }
    fn set_folder_only(&self, mode: bool) {
        unsafe { elm_fileselector_entry_folder_only_set(self.as_raw(), mode as Eina_Bool) };
    }
    fn set_path(&self, path: &str) {
        let c = CString::new(path).unwrap();
        unsafe { elm_fileselector_current_name_set(self.as_raw(), c.as_ptr()) };
    }
}

pub trait CheckExt: WidgetExt {
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
}

pub trait RangerExt: WidgetExt {
    fn value(&self) -> f64;
    fn set_format(&self, format: &str);
    fn set_step(&self, step: f64);
    fn set_range(&self, min: f64, max: f64);
    fn set_value(&self, value: f64);
    fn do_callback(self) {
        self.call_signal(Signal::Changed);
    }
    fn with_range(self, min: f64, max: f64) -> Self {
        self.set_range(min, max);
        self
    }
    fn with_step(self, step: f64) -> Self {
        self.set_step(step);
        self
    }
    fn with_value(self, value: f64) -> Self {
        self.set_value(value);
        self
    }
    fn with_format(self, format: &str) -> Self {
        self.set_format(format);
        self
    }
}

pub trait SelectorExt: WidgetExt {
    fn find(&self, item: super::WidgetItem) -> u32;
    fn add(&self, label: &str) -> super::WidgetItem;
    fn lenght(&self) -> u32;
    fn value(&self) -> u32;
    fn clear(&self);
    fn set_value(&self, value: u32);
    fn add_items(&self, items: &[&str]) {
        for item in items {
            self.add(item);
        }
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

pub trait CalendarExt: WidgetExt {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe {
            let prt = elm_calendar_add(prt.as_raw());
            elm_calendar_selectable_set(prt, Elm_Calendar_Selectable_ELM_CALENDAR_SELECTABLE_DAY);
            prt
        })
        .with_conf();
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

pub trait EntryExt: WidgetExt {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_entry_add(prt.as_raw()) })
            .with_conf()
            .with_single_line(true)
            .with_scrollable(true);
        prt.add(&elm);
        elm
    }
    fn with_menu(prt: &impl ContainerExt, menu: super::Menu) -> Self {
        Self::new(prt)
            .with_editable(false)
            .with_cursor(Cursor::Hand1)
            .with_signal(Signal::Clicked, {
                move |wgt| {
                    let pos = wgt.geometry();
                    menu.open(pos.0, pos.1 + pos.3);
                }
            })
    }
    fn do_callback(self) {
        self.call_signal(Signal::Changed);
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
        unsafe { elm_entry_single_line_set(self.as_raw(), value as Eina_Bool) };
        self.set_weight(true, !value);
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

pub trait IconExt: WidgetExt {
    fn new(parent: &impl WidgetExt) -> Self {
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

pub trait SeparatorExt: WidgetExt {
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

pub trait ListExt: SelectorExt {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe {
            let ptr = elm_list_add(prt.as_raw());
            elm_list_mode_set(ptr, Elm_List_Mode_ELM_LIST_SCROLL);
            elm_list_scroller_policy_set(
                ptr,
                Elm_Scroller_Policy_ELM_SCROLLER_POLICY_AUTO,
                Elm_Scroller_Policy_ELM_SCROLLER_POLICY_AUTO,
            );
            elm_list_go(ptr);
            ptr
        })
        .with_conf()
        .with_signal(Signal::Selected, |wgt| wgt.call_signal(Signal::Changed));
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
}

pub trait FrameExt: ContainerExt {
    fn new(parent: &impl ContainerExt) -> Self {
        let prt = super::Box::new(parent).with_horizontal(true);
        let elm = Self::from_raw(unsafe { elm_frame_add(prt.as_raw()) })
            .with_autocollapse(true)
            .with_conf()
            .with_signal(Signal::Clicked, |wgt| wgt.call_signal(Signal::Changed));
        prt.add(&elm);
        elm
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

pub trait NaviframeExt: WidgetExt {
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
    fn push(&self, child: &impl WidgetExt) -> super::WidgetItem {
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

pub trait PanesExt: WidgetExt {
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
        let elm = Self::from_raw(unsafe { elm_popup_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
    fn with_cancel<F: FnMut(super::Button) + 'static>(self, mut func: F) -> Self {
        let but = super::Button::new(&self)
            .with_text("Cancel")
            .with_callback({
                let elm = self.clone();
                move |wgt| {
                    func(wgt);
                    elm.dismiss();
                }
            });
        self.set_content(&but, "button2");
        self
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
    fn with_ok<F: FnMut(super::Button) + 'static>(self, mut func: F) -> Self {
        let but = super::Button::new(&self).with_text("Ok").with_callback({
            let elm = self.clone();
            move |wgt| {
                func(wgt);
                elm.dismiss();
            }
        });
        self.set_content(&but, "button1");
        self
    }
    fn warning(prt: &impl ContainerExt, message: &str) {
        Self::new(prt)
            .with_timeout(0.0)
            .set_message("dialog-info", "WARNING", message)
    }
    fn set_message(&self, icon: &str, title: &str, text: &str) {
        self.set_content(&super::Icon::new(self).with_standard(icon), "title,icon");
        self.set_part("title,text", title);
        self.set_part("default", text);
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
    fn dismiss(&self) {
        unsafe { elm_popup_dismiss(self.as_raw()) };
    }
}

pub trait ProgressBarExt: WidgetExt {
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
}

pub trait RadioExt: WidgetExt {
    fn from_items<F: FnMut(Self) + 'static + Clone>(
        prt: &impl ContainerExt,
        items: &[&str],
        func: F,
    ) -> Self {
        let elm = Self::new(prt);
        for (idx, item) in items.iter().enumerate() {
            if idx == 0 {
                elm.set_callback(Signal::Changed, func.clone());
                elm.set_state(idx as i32);
                elm.set_text(item);
                elm.set_icon(item);
            } else {
                elm.add_group(
                    &Self::new(prt)
                        .with_callback(func.clone())
                        .with_state(idx as i32)
                        .with_text(item)
                        .with_icon(item),
                );
            }
        }
        elm.set_value(0);
        elm
    }
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_radio_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
    }
    fn add_group(&self, child: &Self) {
        unsafe { elm_radio_group_add(child.as_raw(), self.as_raw()) };
    }
    fn set_state(&self, state: i32) {
        unsafe { elm_radio_state_value_set(self.as_raw(), state) };
    }
    fn with_state(self, state: i32) -> Self {
        self.set_state(state);
        self
    }
    fn value(&self) -> i32 {
        unsafe { elm_radio_value_get(self.as_raw()) as i32 }
    }
    fn set_value(&self, value: i32) {
        unsafe { elm_radio_value_set(self.as_raw(), value) };
    }
}

pub trait SegmentControlExt: SelectorExt {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_segment_control_add(prt.as_raw()) })
            .with_conf()
            .with_weight(true, false);
        prt.add(&elm);
        elm
    }
}

pub trait WindowExt: WidgetExt {
    fn new(id: &str, title: &str) -> Self {
        Self::from_raw(unsafe {
            elm_win_util_standard_add(
                CString::new(id).unwrap().as_ptr(),
                CString::new(title).unwrap().as_ptr(),
            )
        })
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
        if (0..self.lenght()).contains(&value) {
            self.set_value(value);
        };
    }
}

impl<T: SelectorExt + 'static> Update<(Vec<String>, u32)> for T {
    fn update(&self, value: (Vec<String>, u32)) {
        if self.lenght() != (value.0.len() as u32) {
            self.clear();
            for item in &value.0 {
                self.add(item);
            }
        }
        self.update(value.1);
    }
}

pub trait FileSelExt: WidgetExt {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_fileselector_add(prt.as_raw()) })
            .with_folder_only(true)
            .with_expandable(false)
            .with_buttons_ok_cancel(false);
        prt.add(&elm);
        elm
    }
    fn set_path(&self, path: &str) {
        let c = CString::new(path).unwrap();
        unsafe { elm_fileselector_path_set(self.as_raw(), c.as_ptr()) };
    }
    fn with_path(self, path: &str) -> Self {
        self.set_path(path);
        self
    }
    fn path(&self) -> String {
        unsafe {
            let ptr = elm_fileselector_path_get(self.as_raw());
            match ptr.is_null() {
                true => String::new(),
                false => CStr::from_ptr(ptr).to_string_lossy().into_owned(),
            }
        }
    }
    fn selected(&self) -> String {
        unsafe {
            let ptr = elm_fileselector_selected_get(self.as_raw());
            match ptr.is_null() {
                true => String::new(),
                false => CStr::from_ptr(ptr).to_string_lossy().into_owned(),
            }
        }
    }
    fn set_selected(&self, path: &str) -> bool {
        let c = CString::new(path).unwrap();
        unsafe { elm_fileselector_selected_set(self.as_raw(), c.as_ptr()) != 0 }
    }
    fn with_folder_only(self, value: bool) -> Self {
        unsafe { elm_fileselector_folder_only_set(self.as_raw(), value as Eina_Bool) };
        self
    }
    fn with_is_save(self, value: bool) -> Self {
        unsafe { elm_fileselector_is_save_set(self.as_raw(), value as Eina_Bool) };
        self
    }
    fn with_multi_select(self, value: bool) -> Self {
        unsafe { elm_fileselector_multi_select_set(self.as_raw(), value as Eina_Bool) };
        self
    }
    fn with_hidden_visible(self, value: bool) -> Self {
        unsafe { elm_fileselector_hidden_visible_set(self.as_raw(), value as Eina_Bool) };
        self
    }
    fn with_expandable(self, value: bool) -> Self {
        unsafe { elm_fileselector_expandable_set(self.as_raw(), value as Eina_Bool) };
        self
    }
    fn with_buttons_ok_cancel(self, value: bool) -> Self {
        unsafe { elm_fileselector_buttons_ok_cancel_set(self.as_raw(), value as Eina_Bool) };
        self
    }
}

pub trait ClockExt: WidgetExt {
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

pub trait ColorSelExt: WidgetExt {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_colorselector_add(prt.as_raw()) })
            .with_conf()
            .with_signal(Signal::Changed, |wgt| wgt.call_signal(Signal::Selected));
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
    fn run(title: &str, width: i32, height: i32) {
        run(move || {
            let win = super::Window::new("Main", title)
                .with_min_size(width, height)
                .with_center(true);
            Self::mount(&win);
            win
        });
    }
}
