pub use std::sync::mpsc::Sender;
use {
    refl_sys::*,
    std::{
        ffi::{CString, c_char, c_int, c_void},
        sync::mpsc::channel,
    },
};

pub fn run(func: impl Fn()) {
    let c_args = std::env::args()
        .map(|arg| CString::new(arg).unwrap())
        .map(|arg| arg.as_ptr())
        .collect::<Vec<*const c_char>>();

    unsafe {
        elm_init(c_args.len() as c_int, c_args.as_ptr() as *mut *mut c_char);
        elm_policy_set(
            Elm_Policy_ELM_POLICY_QUIT,
            Elm_Policy_Quit_ELM_POLICY_QUIT_LAST_WINDOW_CLOSED as i32,
        );
    };
    func();

    unsafe {
        elm_run();
        elm_shutdown();
    }
}

pub fn exit() {
    unsafe {
        efl_exit(0);
    };
}

pub trait EcoreEventExt: Sized {
    fn as_raw(&self) -> *mut Ecore_Event_Handler;
    fn from_raw(obj: *mut Ecore_Event_Handler) -> Self;
}

pub trait EcoreTimerExt: Sized {
    fn as_raw(&self) -> *mut Ecore_Timer;
    fn from_raw(obj: *mut Ecore_Timer) -> Self;
}

pub trait TimerExt: EcoreTimerExt {
    fn new<F: FnMut() -> bool + 'static>(timeout: f64, func: F) -> Self {
        let raw_ptr: *mut Box<EcoreCb> = Box::into_raw(Box::new(Box::new(func)));
        Self::from_raw(unsafe {
            ecore_timer_add(timeout, Some(ecore_task_cb), raw_ptr as *mut c_void)
        })
    }
    fn del(&self) {
        unsafe { ecore_timer_del(self.as_raw()) };
    }
    fn set_freeze(&self) {
        unsafe { ecore_timer_freeze(self.as_raw()) };
    }
    fn freeze(&self) -> bool {
        unsafe { ecore_timer_freeze_get(self.as_raw()) != 0 }
    }
    fn set_delay(&self, value: f64) {
        unsafe { ecore_timer_delay(self.as_raw(), value) };
    }
}

pub trait EventHandlerExt: EcoreEventExt {
    fn new<F: FnMut() -> bool + 'static>(type_: i32, func: F) -> Self {
        let raw_ptr: *mut Box<EcoreCb> = Box::into_raw(Box::new(Box::new(func)));
        Self::from_raw(unsafe {
            ecore_event_handler_add(type_, Some(ecore_event_handler_cb), raw_ptr as *mut c_void)
        })
    }
    fn del(&self) {
        unsafe { ecore_event_handler_del(self.as_raw()) };
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
    _type_: c_int,
    _event: *mut c_void,
) -> Eina_Bool {
    unsafe {
        let func: &mut Box<EcoreCb> = &mut *(data as *mut Box<EcoreCb>);
        func() as Eina_Bool
    }
}

pub trait EvasObject: Default {
    fn as_raw(&self) -> *mut Evas_Object;
    fn from_raw(obj: *mut Evas_Object) -> Self;
    fn conf(&self) {
        self.set_align(super::Align::Fill, super::Align::Fill);
        self.set_weight(true, true);
    }
    fn show(&self) {
        unsafe {
            evas_object_show(self.as_raw());
        };
    }
    fn with_color(self, r: i32, g: i32, b: i32, a: i32) -> Self {
        self.set_color(r, g, b, a);
        self
    }
    fn set_color(&self, r: i32, g: i32, b: i32, a: i32) {
        unsafe {
            evas_object_color_set(
                self.as_raw(),
                r as c_int,
                g as c_int,
                b as c_int,
                a as c_int,
            );
        };
    }
    fn set_weight(&self, x: bool, y: bool) {
        unsafe {
            evas_object_size_hint_weight_set(self.as_raw(), x as u8 as f64, y as u8 as f64);
        };
    }
    fn with_weight(self, x: bool, y: bool) -> Self {
        self.set_weight(x, y);
        self
    }
    fn del(&self) {
        unsafe {
            evas_object_del(self.as_raw());
        };
    }
    fn parent(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { efl_parent_get(self.as_raw()) })
    }
    fn set_align(&self, x: super::Align, y: super::Align) {
        unsafe {
            evas_object_size_hint_align_set(self.as_raw(), x.to_f64(), y.to_f64());
        };
    }
    fn with_align(self, x: super::Align, y: super::Align) -> Self {
        self.set_align(x, y);
        self
    }
    fn smart_callback_add<T: ElmObject, F: FnMut(T) + 'static>(&self, name: &str, func: F) {
        let name = std::ffi::CString::new(name).unwrap();
        let raw_ptr: *mut Box<dyn FnMut(T)> = Box::into_raw(Box::new(Box::new(func)));
        unsafe {
            evas_object_smart_callback_add(
                self.as_raw(),
                name.as_ptr(),
                Some(smart_cb::<T>),
                raw_ptr as *mut c_void,
            );
        }
    }
    fn with_size(self, w: i32, h: i32) -> Self {
        self.resize(w, h);
        self.set_min_size(w, h);
        self.set_weight(w == 0, h == 0);
        self
    }
    fn resize(&self, w: i32, h: i32) {
        unsafe {
            evas_object_resize(self.as_raw(), w, h);
        };
    }
    fn set_min_size(&self, w: i32, h: i32) {
        unsafe {
            evas_object_size_hint_min_set(self.as_raw(), w, h);
        };
    }
    fn hide(&self) {
        unsafe {
            evas_object_hide(self.as_raw());
        };
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
    fn set_icon(&self, value: &str) {
        super::Icon::new(self).with_standard(value);
    }
    fn set_text(&self, text: &str) {
        self.set_part("default", text);
    }
    fn set_part(&self, part: &str, text: &str) {
        let c_part = std::ffi::CString::new(part).unwrap();
        let c_text = std::ffi::CString::new(text).unwrap();
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
    fn parent_raw(&self) -> *mut Evas_Object {
        unsafe { efl_parent_get(self.as_raw()) }
    }
    fn set_style(&self, value: &str) -> bool {
        let ctext = std::ffi::CString::new(value).unwrap();
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
    fn with_cursor(self, value: &str) -> Self {
        self.set_cursor(value);
        self
    }
    fn set_tooltip(&self, value: &str) {
        let ctext = std::ffi::CString::new(value).unwrap();
        unsafe { elm_object_tooltip_text_set(self.as_raw(), ctext.as_ptr()) }
    }
    fn set_cursor(&self, value: &str) -> bool {
        let ctext = std::ffi::CString::new(value).unwrap();
        unsafe { elm_object_cursor_set(self.as_raw(), ctext.as_ptr()) != 0 }
    }
    fn disabled(&self) -> bool {
        unsafe { elm_object_disabled_get(self.as_raw()) != 0 }
    }
    fn focus(&self) -> bool {
        unsafe { elm_object_focus_get(self.as_raw()) != 0 }
    }
    fn set_disabled(&self, value: bool) {
        unsafe { elm_object_disabled_set(self.as_raw(), value as Eina_Bool) }
    }
    fn set_focus(&self, value: bool) {
        unsafe { elm_object_focus_set(self.as_raw(), value as Eina_Bool) }
    }
    fn with_content(self, obj: &impl ElmObject, value: &str) -> Self {
        self.set_content(obj, value);
        self
    }
    fn set_content(&self, obj: &impl ElmObject, value: &str) {
        let ctext = std::ffi::CString::new(value).unwrap();
        unsafe { elm_object_part_content_set(self.as_raw(), ctext.as_ptr(), obj.as_raw()) }
        obj.show();
    }
    fn content(&self, value: &str) -> Option<super::WidgetItem> {
        let ctext = std::ffi::CString::new(value).unwrap();
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
            if !ptr.is_null() {
                std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
            } else {
                String::new()
            }
        }
    }
    fn window(&self) -> super::Window {
        super::Window::from_raw(unsafe { elm_win_get(self.as_raw()) })
    }
    fn style(&self) -> String {
        unsafe {
            let ptr = elm_object_style_get(self.as_raw());
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
}

pub trait ContainerExt: ElmObject {
    fn add(&self, child: &impl ElmObject) {
        child.show();
    }
    fn insert(self, mut func: impl FnMut(&Self)) -> Self {
        func(&self);
        self
    }
}

pub trait BoxExt: ContainerExt {
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_box_add(parent.as_raw()) });
        elm.set_homogeneous(true);
        elm.set_horizontal(false);
        elm.conf();
        parent.add(&elm);
        elm
    }
    fn with_horizontal(self, value: bool) -> Self {
        self.set_horizontal(value);
        self
    }
    fn with_homogeneous(self, value: bool) -> Self {
        self.set_homogeneous(value);
        self
    }
    fn with_padding(self, horizontal: i32, vertical: i32) -> Self {
        self.set_padding(horizontal, vertical);
        self
    }
    fn horizontal(&self) -> bool {
        unsafe { elm_box_horizontal_get(self.as_raw()) != 0 }
    }
    fn clear(&self) {
        unsafe { elm_box_clear(self.as_raw()) };
    }
    fn recalculate(&self) {
        unsafe { elm_box_recalculate(self.as_raw()) };
    }
    fn unpack_all(&self) {
        unsafe { elm_box_unpack_all(self.as_raw()) };
    }
    fn pack_end(&self, child: &impl super::ElmObject) {
        unsafe { elm_box_pack_end(self.as_raw(), child.as_raw()) };
    }
    fn set_horizontal(&self, value: bool) {
        unsafe { elm_box_horizontal_set(self.as_raw(), value as Eina_Bool) };
        self.set_weight(value, !value);
    }
    fn set_homogeneous(&self, value: bool) {
        unsafe { elm_box_homogeneous_set(self.as_raw(), value as Eina_Bool) };
    }
    fn set_padding(&self, horizontal: i32, vertical: i32) {
        unsafe { elm_box_padding_set(self.as_raw(), horizontal, vertical) };
    }
}

pub trait BubbleExt: ElmObject {
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_bubble_add(parent.as_raw()) });
        elm.conf();
        elm.set_pos(1);
        parent.add(&elm);
        elm
    }
    fn set_pos(&self, value: i32) {
        unsafe { elm_bubble_pos_set(self.as_raw(), value) };
    }
    fn pos(&self) -> i32 {
        unsafe { elm_bubble_pos_get(self.as_raw()) }
    }
}

pub trait MenuExt: OnDismissed {
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_menu_add(parent.as_raw()) });
        elm.conf();
        parent.add(&elm);
        elm.close();
        elm.on_dismissed(|wgt| wgt.del());
        elm
    }
    fn close(&self) {
        unsafe { elm_menu_close(self.as_raw()) }
    }
    fn open(&self) {
        unsafe { elm_menu_open(self.as_raw()) }
    }
    fn index(&self) -> u32 {
        unsafe { elm_menu_item_index_get(self.selected_raw()) as u32 }
    }
    fn icon(&self) -> String {
        unsafe {
            let ptr = elm_menu_item_icon_name_get(self.selected_raw());
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
    fn selected(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_menu_item_object_get(self.selected_raw()) })
    }
    fn selected_raw(&self) -> *mut Evas_Object {
        unsafe { elm_menu_selected_item_get(self.as_raw()) }
    }
    fn with_index(self, value: u32) -> Self {
        self.set_index(value);
        self
    }
    fn set_index(&self, value: u32) {
        let mut raw = unsafe { elm_menu_first_item_get(self.as_raw()) };
        for _idx in 0..value {
            raw = unsafe { elm_menu_item_next_get(raw) }
        }
        unsafe { elm_menu_item_selected_set(raw, true as Eina_Bool) };
    }
}

pub trait ButtonExt: ElmObject {
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_button_add(parent.as_raw()) });
        elm.conf();
        parent.add(&elm);
        elm
    }
}

pub trait CheckExt: ElmObject {
    #[deprecated = "use efl::SegmentControl::new(&parent) instead"]
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_check_add(parent.as_raw()) });
        elm.conf();
        parent.add(&elm);
        elm
    }
    fn set_state(&self, value: bool) {
        unsafe { elm_check_state_set(self.as_raw(), value as Eina_Bool) };
    }
    fn set_value(&self, value: bool) {
        self.set_state(value);
    }
    fn state(&self) -> bool {
        unsafe { elm_check_state_get(self.as_raw()) != 0 }
    }
    fn value(&self) -> bool {
        self.state()
    }
}

pub trait SelectorExt: ElmObject {
    fn add<F: FnMut(Self) + 'static>(&self, icon: &str, label: &str, func: F) -> super::WidgetItem;
    fn add_items<F: FnMut(Self) + 'static + Clone>(&self, items: &[&str], func: F) {
        for item in items {
            self.add(item, item, func.clone());
        }
    }
    fn with_item<F: FnMut(Self) + 'static>(self, label: &str, func: F) -> Self {
        self.add(label, label, func);
        self
    }
    fn with_items<F: FnMut(Self) + 'static + Clone>(self, items: &[&str], func: F) -> Self {
        self.add_items(items, func);
        self
    }
}

pub trait ActionSliderExt: ElmObject {
    #[deprecated = "use efl::SegmentControl::new(&parent) instead"]
    fn new(parent: &impl ContainerExt, left: &str, center: &str, right: &str) -> Self {
        let elm = Self::from_raw(unsafe { elm_actionslider_add(parent.as_raw()) });
        elm.conf();
        elm.set_left(left);
        elm.set_center(center);
        elm.set_right(right);
        elm.set_indicator(&elm.label());
        elm.set_position(super::ActionSliderPos::Right);
        parent.add(&elm);
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
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
    fn with_position(self, value: super::ActionSliderPos) -> Self {
        self.set_position(value);
        self
    }
}

pub trait CalendarExt: ElmObject {
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_calendar_add(parent.as_raw()) });
        elm.conf();
        parent.add(&elm);
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

pub trait ClockExt: ElmObject {
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_clock_add(parent.as_raw()) });
        elm.conf();
        parent.add(&elm);
        elm
    }
    fn time(&self) -> (i32, i32, i32) {
        let hrs: *mut ::std::os::raw::c_int = std::ptr::null_mut();
        let min: *mut ::std::os::raw::c_int = std::ptr::null_mut();
        let sec: *mut ::std::os::raw::c_int = std::ptr::null_mut();
        unsafe { elm_clock_time_get(self.as_raw(), hrs, min, sec) };
        (hrs as i32, min as i32, sec as i32)
    }
    fn set_time(&self, hrs: i32, min: i32, sec: i32) {
        unsafe { elm_clock_time_set(self.as_raw(), hrs, min, sec) };
    }
}

pub trait CtxpopupExt: ElmObject {
    #[deprecated = "use efl::Notify::new(&parent) instead"]
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_ctxpopup_add(parent.as_raw()) });
        elm.conf();
        parent.add(&elm);
        elm
    }
    fn set_horizontal(&self, value: bool) {
        unsafe { elm_ctxpopup_horizontal_set(self.as_raw(), value as Eina_Bool) };
    }
    fn set_auto_hide(&self, value: bool) {
        unsafe { elm_ctxpopup_auto_hide_disabled_set(self.as_raw(), value as Eina_Bool) };
    }
    fn first(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_ctxpopup_first_item_get(self.as_raw()) })
    }
    fn prev(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_ctxpopup_item_prev_get(self.as_raw()) })
    }
    fn last(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_ctxpopup_last_item_get(self.as_raw()) })
    }
    fn selected(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_ctxpopup_selected_item_get(self.as_raw()) })
    }
}

pub trait LabelExt: ElmObject {
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_label_add(parent.as_raw()) });
        elm.conf();
        parent.add(&elm);
        elm
    }
}

pub trait HoverSelExt: ElmObject {
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_hoversel_add(parent.as_raw()) });
        elm.conf();
        elm.set_auto_update(true);
        parent.add(&elm);
        elm
    }
    fn set_auto_update(&self, value: bool) {
        unsafe { elm_hoversel_auto_update_set(self.as_raw(), value as Eina_Bool) };
    }
    fn set_horizontal(&self, value: bool) {
        unsafe { elm_hoversel_horizontal_set(self.as_raw(), value as Eina_Bool) };
    }
    fn clear(&self) {
        unsafe { elm_hoversel_clear(self.as_raw()) };
    }
    fn end(&self) {
        unsafe { elm_hoversel_hover_end(self.as_raw()) };
    }
    fn begin(&self) {
        unsafe { elm_hoversel_hover_begin(self.as_raw()) };
    }
}

pub trait FlipSelectorExt: ElmObject {
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_flipselector_add(parent.as_raw()) });
        elm.conf();
        elm.set_interval(3.0);
        parent.add(&elm);
        elm
    }
    fn selected(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_flipselector_selected_item_get(self.as_raw()) })
    }
    fn with_items<F: FnMut(Self) + 'static + Clone>(self, items: &[&str], func: F) -> Self {
        self.add_items(items, func);
        self
    }
    fn with_item<F: FnMut(Self) + 'static>(self, item: &str, func: F) -> Self {
        self.add(item, func);
        self
    }
    fn add_items<F: FnMut(Self) + 'static + Clone>(&self, items: &[&str], func: F) {
        for item in items {
            self.add(item, func.clone());
        }
    }
    fn add<F: FnMut(Self) + 'static>(&self, label: &str, func: F) {
        let clabel = std::ffi::CString::new(label).unwrap();
        let raw_ptr: *mut Box<dyn FnMut(Self)> = Box::into_raw(Box::new(Box::new(func)));
        unsafe {
            elm_flipselector_item_append(
                self.as_raw(),
                clabel.as_ptr(),
                Some(smart_cb::<Self>),
                raw_ptr as *mut std::os::raw::c_void,
            )
        };
    }
    fn value(&self) -> u32 {
        self.index()
    }
    fn set_value(&self, value: u32) {
        self.set_index(value)
    }
    fn set_index(&self, value: u32) {
        let mut temp = self.first_raw();
        for _ in 0..value {
            temp = unsafe { elm_flipselector_item_next_get(temp) };
        }
        unsafe { elm_flipselector_item_selected_set(temp, true as Eina_Bool) }
    }
    fn index(&self) -> u32 {
        let selected = self.selected_raw();
        let mut temp = self.first_raw();
        let mut count = 0;
        while temp != selected {
            temp = unsafe { elm_flipselector_item_next_get(temp) };
            count += 1;
        }
        count
    }
    fn selected_raw(&self) -> *mut Evas_Object {
        unsafe { elm_flipselector_selected_item_get(self.as_raw()) }
    }
    fn first_raw(&self) -> *mut Evas_Object {
        unsafe { elm_flipselector_first_item_get(self.as_raw()) }
    }
    fn last_raw(&self) -> *mut Evas_Object {
        unsafe { elm_flipselector_last_item_get(self.as_raw()) }
    }
    fn set_interval(&self, value: f64) {
        unsafe { elm_flipselector_first_interval_set(self.as_raw(), value) }
    }
    fn lenght(&self) -> u32 {
        let last = self.last_raw();
        let mut temp = self.first_raw();
        let mut count = 0;
        while temp != last {
            temp = unsafe { elm_flipselector_item_next_get(temp) };
            count += 1;
        }
        count
    }
}

pub trait EntryExt: ElmObject {
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_entry_add(parent.as_raw()) });
        elm.conf();
        elm.set_single_line(true);
        elm.set_scrollable(true);
        elm.set_menu(true);
        parent.add(&elm);
        elm
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
    fn set_value(&self, value: &str) {
        self.set_entry(value);
    }
    fn set_entry(&self, value: &str) {
        let ctext = std::ffi::CString::new(value).unwrap();
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
        let file = std::ffi::CString::new(file_).unwrap();
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
        self.entry()
    }
    fn editable(&self) -> bool {
        unsafe { elm_entry_editable_get(self.as_raw()) != 0 }
    }
    fn entry(&self) -> String {
        unsafe {
            let ptr = elm_entry_entry_get(self.as_raw());
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
}

pub trait FrameExt: ElmObject {
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_frame_add(parent.as_raw()) });
        elm.conf();
        elm.set_autocollapse(true);
        parent.add(&elm);
        elm
    }
    fn with_collapse(self, value: bool) -> Self {
        self.set_collapse(value);
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
    fn collapse(&self) -> bool {
        unsafe { elm_frame_collapse_get(self.as_raw()) != 0 }
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
        let ctext = std::ffi::CString::new(value).unwrap();
        unsafe { elm_icon_standard_set(self.as_raw(), ctext.as_ptr()) };
    }
}

pub trait SeparatorExt: ElmObject {
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_separator_add(parent.as_raw()) });
        elm.conf();
        parent.add(&elm);
        elm
    }
    fn with_horizontal(self, value: bool) -> Self {
        self.set_horizontal(value);
        self
    }
    fn set_horizontal(&self, value: bool) {
        unsafe { elm_separator_horizontal_set(self.as_raw(), value as Eina_Bool) };
    }
}

pub trait ListExt: ElmObject {
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_list_add(parent.as_raw()) });
        elm.conf();
        elm.set_mode(super::ListMode::Expand);
        elm.go();
        parent.add(&elm);
        elm
    }
    fn clear(&self) {
        unsafe { elm_list_clear(self.as_raw()) };
    }
    fn go(&self) {
        unsafe { elm_list_go(self.as_raw()) };
    }
    fn set_bounce(&self, h_bounce: bool, v_bounce: bool) {
        unsafe { elm_list_bounce_set(self.as_raw(), h_bounce as Eina_Bool, v_bounce as Eina_Bool) };
    }
    fn with_horizontal(self, value: bool) -> Self {
        self.set_horizontal(value);
        self
    }
    fn set_horizontal(&self, value: bool) {
        unsafe { elm_list_horizontal_set(self.as_raw(), value as Eina_Bool) };
    }
    fn set_multi(&self, value: bool) {
        unsafe { elm_list_multi_select_set(self.as_raw(), value as Eina_Bool) };
    }
    fn set_mode(&self, value: super::ListMode) {
        unsafe { elm_list_mode_set(self.as_raw(), value as Elm_List_Mode) };
    }
    fn value(&self) -> u32 {
        self.index()
    }
    fn set_value(&self, value: u32) {
        self.set_index(value)
    }
    fn selected(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_list_item_object_get(self.selected_raw()) })
    }
    fn set_index(&self, value: u32) {
        let mut temp = self.first_raw();
        for _ in 0..value {
            temp = unsafe { elm_list_item_next(temp) };
        }
        unsafe { elm_list_item_selected_set(temp, true as Eina_Bool) }
    }
    fn index(&self) -> u32 {
        let selected = self.selected_raw();
        let mut temp = self.first_raw();
        let mut count = 0;
        while temp != selected {
            temp = unsafe { elm_list_item_next(temp) };
            count += 1;
        }
        count
    }
    fn selected_raw(&self) -> *mut Evas_Object {
        unsafe { elm_list_selected_item_get(self.as_raw()) }
    }
    fn first_raw(&self) -> *mut Evas_Object {
        unsafe { elm_list_first_item_get(self.as_raw()) }
    }
    fn last_raw(&self) -> *mut Evas_Object {
        unsafe { elm_list_last_item_get(self.as_raw()) }
    }
    fn lenght(&self) -> u32 {
        let last = self.last_raw();
        let mut temp = self.first_raw();
        let mut count = 0;
        while temp != last {
            temp = unsafe { elm_toolbar_item_next_get(temp) };
            count += 1;
        }
        count
    }
    fn on_activated<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("activated", func);
    }
    fn on_selected<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("selected", func);
    }
    fn on_unselected<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("unselected", func);
    }
    fn on_longpressed<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("longpressed", func);
    }
    fn on_clicked_double<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("clicked,double", func);
    }
    fn on_clicked_right<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("clicked,right", func);
    }
}

pub trait NaviframeExt: ElmObject {
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_naviframe_add(parent.as_raw()) });
        elm.set_prev(false);
        elm.conf();
        parent.add(&elm);
        elm
    }
    fn set_prev(&self, value: bool) {
        unsafe { elm_naviframe_prev_btn_auto_pushed_set(self.as_raw(), value as Eina_Bool) };
    }
    fn set_event_enabled(&self, value: bool) {
        unsafe { elm_naviframe_event_enabled_set(self.as_raw(), value as Eina_Bool) };
    }
    fn set_content_preserve_on_pop(&self, value: bool) {
        unsafe { elm_naviframe_content_preserve_on_pop_set(self.as_raw(), value as Eina_Bool) };
    }
    fn push(&self, child: &impl super::ElmObject) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe {
            elm_naviframe_item_push(
                self.as_raw(),
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                child.as_raw(),
                std::ptr::null(),
            )
        })
    }
    fn promote(&self) {
        self.to_top(&self.bottom())
    }
    fn to_top(&self, item: &super::WidgetItem) {
        unsafe { elm_naviframe_item_promote(item.as_raw()) };
    }
    fn bottom(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_naviframe_bottom_item_get(self.as_raw()) })
    }
    fn top(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_naviframe_top_item_get(self.as_raw()) })
    }
}

pub trait NotifyExt: ElmObject {
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_notify_add(parent.window().as_raw()) });
        elm.conf();
        parent.add(&elm);
        elm
    }
    fn set_timeout(&self, value: f64) {
        unsafe { elm_notify_timeout_set(self.as_raw(), value) };
    }
}

pub trait PanelExt: ElmObject {
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_panel_add(parent.as_raw()) });
        elm.conf();
        elm.set_orient(super::PanelOrient::Bottom);
        elm.set_hidden(true);
        parent.add(&elm);
        elm
    }
    fn set_hidden(&self, value: bool) {
        unsafe { elm_panel_hidden_set(self.as_raw(), value as Eina_Bool) };
    }
    fn set_scrollable(&self, value: bool) {
        unsafe { elm_panel_scrollable_set(self.as_raw(), value as Eina_Bool) };
    }
    fn set_orient(&self, value: super::PanelOrient) {
        unsafe { elm_panel_orient_set(self.as_raw(), value as u32) };
    }
    fn set_scrollable_content(&self, value: f64) {
        unsafe { elm_panel_scrollable_content_size_set(self.as_raw(), value) };
    }
    fn hidden(&self) -> bool {
        unsafe { elm_panel_hidden_get(self.as_raw()) != 0 }
    }
}

pub trait PanesExt: ElmObject {
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_panes_add(parent.as_raw()) });
        elm.conf();
        parent.add(&elm);
        elm
    }
    fn with_horizontal(self, value: bool) -> Self {
        self.set_horizontal(value);
        self
    }
    fn with_fixed(self, value: bool) -> Self {
        self.set_fixed(value);
        self
    }
    fn set_horizontal(&self, value: bool) {
        unsafe { elm_panes_horizontal_set(self.as_raw(), value as Eina_Bool) };
    }
    fn set_fixed(&self, value: bool) {
        unsafe { elm_panes_fixed_set(self.as_raw(), value as Eina_Bool) };
    }
    fn set_left_size(&self, value: f64) {
        if (0.0..1.0).contains(&value) {
            unsafe { elm_panes_content_left_size_set(self.as_raw(), value) };
        }
    }
    fn set_left_min_size(&self, value: i32) {
        unsafe { elm_panes_content_left_min_size_set(self.as_raw(), value) };
    }
    fn set_left_min_relative_size(&self, value: f64) {
        if (0.0..1.0).contains(&value) {
            unsafe { elm_panes_content_left_min_relative_size_set(self.as_raw(), value) };
        }
    }
    fn set_right_size(&self, value: f64) {
        if (0.0..1.0).contains(&value) {
            unsafe { elm_panes_content_right_size_set(self.as_raw(), value) };
        }
    }
    fn set_right_min_size(&self, value: i32) {
        unsafe { elm_panes_content_right_min_size_set(self.as_raw(), value) };
    }
    fn set_right_min_relative_size(&self, value: f64) {
        if (0.0..1.0).contains(&value) {
            unsafe { elm_panes_content_right_min_relative_size_set(self.as_raw(), value) };
        }
    }
}

pub trait PopupExt: ElmObject {
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_popup_add(parent.window().as_raw()) });
        elm.conf();
        parent.add(&elm);
        elm
    }
    fn set_title_text(&self, value: &str) {
        self.set_part("title,text", value)
    }
    fn set_title_icon(&self, value: &super::Icon) {
        self.set_content(value, "title,icon")
    }
    //~ fn set_button1<F: FnMut(super::Button) + 'static>(&self, label: &str, mut func: F) {
    //~ let pop = self.clone();
    //~ self.set_content(
    //~ &super::Button::new(self)
    //~ .with_text(label)
    //~ .with_icon(label)
    //~ .with_clicked(move |wgt| {
    //~ func(wgt);
    //~ pop.del();
    //~ }),
    //~ "button1",
    //~ );
    //~ }
    //~ fn set_button2<F: FnMut(super::Button) + 'static>(&self, label: &str, mut func: F) {
    //~ let pop = self.clone();
    //~ self.set_content(
    //~ &super::Button::new(self)
    //~ .with_text(label)
    //~ .with_icon(label)
    //~ .with_clicked(move |wgt| {
    //~ func(wgt);
    //~ pop.del();
    //~ }),
    //~ "button2",
    //~ );
    //~ }
    //~ fn set_button3<F: FnMut(super::Button) + 'static>(&self, label: &str, mut func: F) {
    //~ let pop = self.clone();
    //~ self.set_content(
    //~ &super::Button::new(self)
    //~ .with_text(label)
    //~ .with_icon(label)
    //~ .with_clicked(move |wgt| {
    //~ func(wgt);
    //~ pop.del();
    //~ }),
    //~ "button3",
    //~ );
    //~ }
}

pub trait ProgressBarExt: ElmObject {
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_progressbar_add(parent.as_raw()) });
        elm.conf();
        parent.add(&elm);
        elm
    }
    fn value(&self) -> f64 {
        unsafe { elm_progressbar_value_get(self.as_raw()) }
    }
    fn set_value(&self, value: f64) {
        unsafe { elm_progressbar_value_set(self.as_raw(), value) };
    }
    fn set_unit_format(&self, value: &str) {
        let ctext = std::ffi::CString::new(value).unwrap();
        unsafe { elm_progressbar_unit_format_set(self.as_raw(), ctext.as_ptr()) };
    }
}

pub trait RadioExt: OnChanged {
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
                elm.on_changed(func.clone());
            } else {
                let child = Self::item(parent).with_text(item).with_icon(item);
                child.on_changed(func.clone());
                elm.add_group(&child);
            }
        }
        elm
    }
    fn item(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_radio_add(parent.as_raw()) });
        elm.conf();
        elm.set_state_value(0);
        parent.add(&elm);
        elm
    }
    fn with_add(self, parent: &impl ContainerExt, child: &str) -> Self {
        self.add(parent, child);
        self
    }
    fn add(&self, parent: &impl ContainerExt, child: &str) {
        self.add_group(&Self::item(parent).with_text(child));
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

pub trait ScrollerExt: ElmObject {
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_scroller_add(parent.as_raw()) });
        elm.conf();
        parent.add(&elm);
        elm
    }
    fn set_state(&self, policy_h: super::ScrollerPolicy, policy_v: super::ScrollerPolicy) {
        unsafe { elm_scroller_policy_set(self.as_raw(), policy_h as u32, policy_v as u32) };
    }
    fn last_page(&self) -> (i32, i32) {
        let (mut h, mut v) = (0i32, 0i32);
        unsafe { elm_scroller_last_page_get(self.as_raw(), &mut h, &mut v) };
        (h, v)
    }
}

pub trait SegmentControlExt: ElmObject {
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_segment_control_add(parent.as_raw()) });
        elm.conf();
        parent.add(&elm);
        elm
    }
    fn selected(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe {
            elm_segment_control_item_object_get(self.selected_raw())
        })
    }
    fn selected_raw(&self) -> *mut Evas_Object {
        unsafe { elm_segment_control_item_selected_get(self.as_raw()) }
    }
    fn value(&self) -> u32 {
        self.index()
    }
    fn index(&self) -> u32 {
        unsafe { elm_segment_control_item_index_get(self.selected_raw()) as u32 }
    }
    fn set_value(&self, value: u32) {
        self.set_index(value);
    }
    fn set_index(&self, value: u32) {
        unsafe {
            elm_segment_control_item_selected_set(
                elm_segment_control_item_get(self.as_raw(), value as i32),
                true as Eina_Bool,
            )
        };
    }
    fn label(&self) -> String {
        unsafe {
            let ptr = elm_segment_control_item_label_get(self.as_raw(), self.index() as i32);
            std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
    fn with_item(self, label: &str) -> Self {
        self.add(label, label);
        self
    }
    fn with_items(self, items: &[&str]) -> Self {
        self.add_items(items);
        self
    }
    fn add_items(&self, items: &[&str]) {
        for item in items {
            self.add(item, item);
        }
    }
    fn add(&self, icon: &str, label: &str) -> *mut Evas_Object {
        let clabel = std::ffi::CString::new(label).unwrap();
        let icon = super::Icon::new(self).with_standard(icon);
        unsafe { elm_segment_control_item_add(self.as_raw(), icon.as_raw(), clabel.as_ptr()) }
    }
}

pub trait SliderExt: ElmObject {
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_slider_add(parent.as_raw()) });
        elm.conf();
        parent.add(&elm);
        elm
    }
    fn set_horizontal(&self, value: bool) {
        unsafe { elm_slider_horizontal_set(self.as_raw(), value as Eina_Bool) };
    }
    fn set_inverted(&self, value: bool) {
        unsafe { elm_slider_inverted_set(self.as_raw(), value as Eina_Bool) };
    }
    fn set_range_enabled(&self, value: bool) {
        unsafe { elm_slider_range_enabled_set(self.as_raw(), value as Eina_Bool) };
    }
    fn with_range(self, min: f64, max: f64) -> Self {
        self.set_range(min, max);
        self
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
    fn with_step(self, value: f64) -> Self {
        self.set_step(value);
        self
    }
    fn value(&self) -> f64 {
        unsafe { elm_slider_value_get(self.as_raw()) }
    }
    fn with_format(self, value: &str) -> Self {
        self.set_format(value);
        self
    }
    fn set_format(&self, value: &str) {
        let ctext = std::ffi::CString::new(value).unwrap();
        unsafe { elm_slider_unit_format_set(self.as_raw(), ctext.as_ptr()) };
    }
}

pub trait SpinnerExt: ElmObject {
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_spinner_add(parent.as_raw()) });
        elm.conf();
        parent.add(&elm);
        elm
    }
    fn with_step(self, value: f64) -> Self {
        self.set_step(value);
        self
    }
    fn with_format(self, value: &str) -> Self {
        self.set_format(value);
        self
    }
    fn with_range(self, min: f64, max: f64) -> Self {
        self.set_range(min, max);
        self
    }
    fn set_range(&self, min: f64, max: f64) {
        unsafe { elm_spinner_min_max_set(self.as_raw(), min, max) };
    }
    fn set_step(&self, value: f64) {
        unsafe { elm_spinner_step_set(self.as_raw(), value) };
    }
    fn set_value(&self, value: f64) {
        unsafe { elm_spinner_value_set(self.as_raw(), value) };
    }
    fn value(&self) -> f64 {
        unsafe { elm_spinner_value_get(self.as_raw()) }
    }
    fn set_format(&self, value: &str) {
        let ctext = std::ffi::CString::new(value).unwrap();
        unsafe { elm_spinner_label_format_set(self.as_raw(), ctext.as_ptr()) };
    }
}

pub trait ToolBarExt: ElmObject {
    fn new(parent: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_toolbar_add(parent.as_raw()) });
        elm.conf();
        elm.set_shrink_mode(super::Shrink::Menu);
        parent.add(&elm);
        elm
    }
    fn with_homogeneous(self, value: bool) -> Self {
        self.set_homogeneous(value);
        self
    }
    fn with_horizontal(self, value: bool) -> Self {
        self.set_horizontal(value);
        self
    }
    fn set_horizontal(&self, value: bool) {
        unsafe { elm_toolbar_horizontal_set(self.as_raw(), value as Eina_Bool) };
    }
    fn set_homogeneous(&self, value: bool) {
        unsafe { elm_toolbar_homogeneous_set(self.as_raw(), value as Eina_Bool) };
    }
    fn set_shrink_mode(&self, value: super::Shrink) {
        unsafe { elm_toolbar_shrink_mode_set(self.as_raw(), value as u32) };
    }
    fn selected(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_toolbar_item_object_get(self.selected_raw()) })
    }
    fn value(&self) -> u32 {
        self.index()
    }
    fn set_value(&self, value: u32) {
        self.set_index(value)
    }
    fn set_index(&self, value: u32) {
        let mut temp = self.first_raw();
        for _ in 0..value {
            temp = unsafe { elm_toolbar_item_next_get(temp) };
        }
        unsafe { elm_toolbar_item_selected_set(temp, true as Eina_Bool) }
    }
    fn index(&self) -> u32 {
        let selected = self.selected_raw();
        let mut temp = self.first_raw();
        let mut count = 0;
        while temp != selected {
            temp = unsafe { elm_toolbar_item_next_get(temp) };
            count += 1;
        }
        count
    }
    fn lenght(&self) -> u32 {
        let last = self.last_raw();
        let mut temp = self.first_raw();
        let mut count = 0;
        while temp != last {
            temp = unsafe { elm_toolbar_item_next_get(temp) };
            count += 1;
        }
        count
    }
    fn selected_raw(&self) -> *mut Evas_Object {
        unsafe { elm_toolbar_selected_item_get(self.as_raw()) }
    }
    fn first_raw(&self) -> *mut Evas_Object {
        unsafe { elm_toolbar_first_item_get(self.as_raw()) }
    }
    fn last_raw(&self) -> *mut Evas_Object {
        unsafe { elm_toolbar_last_item_get(self.as_raw()) }
    }
}

pub trait WindowExt: OnDeleteRequest {
    fn new(id_: &str, title_: &str) -> Self {
        let id = std::ffi::CString::new(id_).unwrap();
        let title = std::ffi::CString::new(title_).unwrap();
        let elm = Self::from_raw(unsafe { elm_win_util_standard_add(id.as_ptr(), title.as_ptr()) });
        elm.resize(360, 640);
        elm.set_autodel(true);
        elm.set_center(true, true);
        elm.on_delete_request(|_| exit());
        elm.show();
        elm
    }
    fn with_center(self, value: bool) -> Self {
        self.set_center(value, value);
        self
    }
    fn with_autodel(self, value: bool) -> Self {
        self.set_autodel(value);
        self
    }
    fn with_borderless(self, value: bool) -> Self {
        self.set_borderless(value);
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
    fn set_borderless(&self, value: bool) {
        unsafe {
            elm_win_borderless_set(self.as_raw(), value as Eina_Bool);
        }
    }
    fn set_fullscreen(&self, value: bool) {
        unsafe {
            elm_win_fullscreen_set(self.as_raw(), value as Eina_Bool);
        }
    }
    fn set_type(&self, value: super::WinType) {
        unsafe {
            elm_win_type_set(self.as_raw(), value as Elm_Win_Type);
        }
    }
    fn set_floating_mode(&self, value: bool) {
        unsafe {
            elm_win_floating_mode_set(self.as_raw(), value as Eina_Bool);
        }
    }
    fn autodel(&self) -> bool {
        unsafe { elm_win_autodel_get(self.as_raw()) != 0 }
    }
}

pub trait OnChanged: ElmObject {
    fn on_changed<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("changed", func);
    }
    fn with_changed<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.on_changed(func);
        self
    }
}

pub trait OnClicked: ElmObject {
    fn on_clicked<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("clicked", func);
    }
    fn with_clicked<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.on_clicked(func);
        self
    }
}

pub trait OnDoubleClicked: ElmObject {
    fn on_double_clicked<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("clicked,double", func);
    }
    fn with_double_clicked<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.on_double_clicked(func);
        self
    }
}

pub trait OnSelected: ElmObject {
    fn on_selected<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("selected", func);
    }
    fn with_selected<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.on_selected(func);
        self
    }
}

pub trait OnToggled: ElmObject {
    fn on_toggled<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("toggled", func);
    }
    fn with_toggled<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.on_toggled(func);
        self
    }
}

pub trait OnPosChanged: ElmObject {
    fn on_pos_changed<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("pos_changed", func);
    }
    fn with_pos_changed<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.on_pos_changed(func);
        self
    }
}

pub trait OnTimeout: ElmObject {
    fn on_timeout<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("timeout", func);
    }
    fn with_timeout<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.on_timeout(func);
        self
    }
}

pub trait OnBlockClicked: ElmObject {
    fn on_block_clicked<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("block,clicked", func);
    }
    fn with_block_clicked<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.on_block_clicked(func);
        self
    }
}

pub trait OnDismissed: ElmObject {
    fn on_dismissed<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("dismissed", func);
    }
    fn with_dismissed<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.on_dismissed(func);
        self
    }
}

pub trait OnExpanded: ElmObject {
    fn on_expanded<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("expanded", func);
    }
    fn with_expanded<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.on_expanded(func);
        self
    }
}

pub trait OnItemSelected: ElmObject {
    fn on_item_selected<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("item,selected", func);
    }
    fn with_item_selected<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.on_item_selected(func);
        self
    }
}

pub trait OnItemPressed: ElmObject {
    fn on_item_pressed<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("item,pressed", func);
    }
    fn with_item_pressed<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.on_item_pressed(func);
        self
    }
}

pub trait OnFilterDone: ElmObject {
    fn on_filter_done<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("filter,done", func);
    }
    fn with_filter_done<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.on_filter_done(func);
        self
    }
}

pub trait OnDeleteRequest: ElmObject {
    fn on_delete_request<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("delete,request", func);
    }
    fn with_delete_request<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.on_delete_request(func);
        self
    }
}

pub trait OnDelayChanged: ElmObject {
    fn on_delay_changed<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("delay,changed", func);
    }
    fn with_delay_changed<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.on_delay_changed(func);
        self
    }
}

pub trait OnPressed: ElmObject {
    fn on_pressed<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("press", func);
    }
    fn with_pressed<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.on_pressed(func);
        self
    }
}

pub trait Component: Default + 'static {
    type Event: 'static;
    type State: Default + 'static;
    fn handle(msg: Self::Event, model: &mut Self::State, sender: Sender<Self::Event>) -> bool;
    fn update(&self, model: &Self::State);
    fn view(&mut self, parent: &impl ContainerExt, sender: Sender<Self::Event>);
    fn mount(parent: &impl ContainerExt) {
        let (sender, receiver) = channel::<Self::Event>();
        let mut page = Self::default();
        let mut model = Self::State::default();
        page.view(parent, sender.clone());
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
        run(move || Self::mount(&super::Window::new("Main", title)));
    }
}
