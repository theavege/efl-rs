pub use std::sync::mpsc::Sender;
use {
    refl_sys::*,
    std::{
        ffi::{CStr, CString, c_void},
        sync::mpsc::channel,
    },
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

pub trait EcoreEventExt: Sized {
    fn as_raw(&self) -> *mut Ecore_Event_Handler;
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

pub trait EventHandlerExt: EcoreEventExt + From<*mut Ecore_Event_Handler> {
    fn new<F: FnMut() -> bool + 'static>(type_: i32, func: F) -> Self {
        let raw_ptr: *mut Box<EcoreCb> = Box::into_raw(Box::new(Box::new(func)));
        Self::from(unsafe {
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
    fn conf(&self) {
        self.set_align(Align::Fill, Align::Fill);
        self.set_weight(true, true);
    }
    fn with_conf(self) -> Self {
        self.conf();
        self
    }
    fn show(&self) {
        unsafe {
            evas_object_show(self.as_raw());
        };
    }
    fn set_color(&self, r: i32, g: i32, b: i32, a: i32) {
        unsafe {
            evas_object_color_set(self.as_raw(), r, g, b, a);
        };
    }
    fn set_weight(&self, x: bool, y: bool) {
        unsafe {
            evas_object_size_hint_weight_set(self.as_raw(), x as u8 as f64, y as u8 as f64);
        };
    }
    fn with_color(self, r: i32, g: i32, b: i32, a: i32) -> Self {
        self.set_color(r, g, b, a);
        self
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
    fn set_align(&self, x: Align, y: Align) {
        unsafe {
            evas_object_size_hint_align_set(self.as_raw(), x.to_f64(), y.to_f64());
        };
    }
    fn with_align(self, x: Align, y: Align) -> Self {
        self.set_align(x, y);
        self
    }
    fn smart_callback_add<T: ElmObject, F: FnMut(T) + 'static>(&self, name: &str, func: F) {
        let raw_ptr: *mut Box<dyn FnMut(T)> = Box::into_raw(Box::new(Box::new(func)));
        unsafe {
            evas_object_smart_callback_add(
                self.as_raw(),
                CString::new(name).unwrap().as_ptr(),
                Some(smart_cb::<T>),
                raw_ptr as *mut c_void,
            );
        }
    }
    fn with_size(self, w: i32, h: i32) -> Self {
        self.set_min_size(w, h);
        self.set_weight(w == 0, h == 0);
        self.resize(w, h);
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
    fn parent_raw(&self) -> *mut Evas_Object {
        unsafe { efl_parent_get(self.as_raw()) }
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
    fn with_cursor(self, value: &str) -> Self {
        self.set_cursor(value);
        self
    }
    fn set_tooltip(&self, value: &str) {
        let ctext = CString::new(value).unwrap();
        unsafe { elm_object_tooltip_text_set(self.as_raw(), ctext.as_ptr()) }
    }
    fn set_cursor(&self, value: &str) -> bool {
        let ctext = CString::new(value).unwrap();
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
            if !ptr.is_null() {
                CStr::from_ptr(ptr).to_string_lossy().into_owned()
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
            CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
}

pub trait ContainerExt: ElmObject {
    fn add(&self, child: &impl ElmObject) {
        child.show();
    }
    fn inside(self, mut func: impl FnMut(&Self)) -> Self {
        func(&self);
        self
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
        let mut r: i32 = 0;
        let mut g: i32 = 0;
        let mut b: i32 = 0;
        let mut a: i32 = 0;
        unsafe { elm_colorselector_color_get(self.as_raw(), &mut r, &mut g, &mut b, &mut a) };
        (r, g, b, a)
    }
    fn set_color(&self, r: i32, g: i32, b: i32, a: i32) {
        unsafe { elm_colorselector_color_set(self.as_raw(), r, g, b, a) };
    }
}

pub trait DiskselectorExt: Sized + ElmObject {
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

pub trait MenuExt: SelectorExt {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_menu_add(prt.as_raw()) })
            //~ .with_dismissed(|wgt| wgt.del())
            .with_conf();
        prt.add(&elm);
        elm
    }
    fn main_menu(win: &impl ContainerExt) -> Self {
        Self::from_raw(unsafe { elm_win_main_menu_get(win.as_raw()) })
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
    fn open(&self) {
        unsafe { elm_menu_open(self.as_raw()) }
    }
    fn index(&self) -> u32 {
        unsafe { elm_menu_item_index_get(self.selected().as_raw()) as u32 }
    }
    fn icon(&self) -> String {
        unsafe {
            let ptr = elm_menu_item_icon_name_get(self.selected().as_raw());
            CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
    fn last_item(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_menu_last_item_get(self.as_raw()) })
    }
    fn first_item(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { refl_sys::elm_menu_first_item_get(self.as_raw()) })
    }
    fn with_index(self, value: u32) -> Self {
        self.set_index(value);
        self
    }
    fn set_index(&self, value: u32) {
        let mut temp = self.first().as_raw();
        for _idx in 0..value {
            temp = unsafe { elm_menu_item_next_get(temp) }
        }
        unsafe { elm_menu_item_selected_set(temp, true as Eina_Bool) };
    }
    fn clear_items(&self) {
        let mut temp = self.first().as_raw();
        while temp != self.last().as_raw() {
            super::WidgetItem::from_raw(temp).del();
            temp = unsafe { refl_sys::elm_menu_item_next_get(temp) };
        }
        self.last().del();
    }
}

pub trait ComboboxExt: ElmObject {
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
    fn hover_begin(&self) {
        unsafe { elm_combobox_hover_begin(self.as_raw()) };
    }
    fn hover_end(&self) {
        unsafe { elm_combobox_hover_end(self.as_raw()) };
    }
    fn expanded(&self) -> bool {
        unsafe { elm_combobox_expanded_get(self.as_raw()) != 0 }
    }
    fn value(&self) -> u32 {
        self.index()
    }
    fn set_value(&self, value: u32) {
        self.set_index(value);
    }
    fn index(&self) -> u32 {
        let selected = self.selected_raw();
        let mut temp = self.first_raw();
        let mut count = 0u32;
        while !temp.is_null() && temp != selected {
            temp = unsafe { elm_genlist_item_next_get(temp) };
            count += 1;
        }
        count
    }
    fn set_index(&self, value: u32) {
        let mut temp = self.first_raw();
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
    fn selected_raw(&self) -> *mut Elm_Object_Item {
        unsafe { elm_genlist_selected_item_get(self.as_raw()) }
    }
    fn first_raw(&self) -> *mut Elm_Object_Item {
        unsafe { elm_genlist_first_item_get(self.as_raw()) }
    }
    fn last_raw(&self) -> *mut Elm_Object_Item {
        unsafe { elm_genlist_last_item_get(self.as_raw()) }
    }
    fn clear(&self) {
        unsafe { elm_combobox_hover_end(self.as_raw()) };
        unsafe { elm_genlist_clear(self.as_raw()) };
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
    #[deprecated = "use refl::SegmentControl::new(&parent) instead"]
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_check_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
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

pub trait RangerExt: ElmObject {
    fn value(&self) -> f64;
    fn set_format(&self, value: &str);
    fn set_step(&self, step: f64);
    fn set_range(&self, min: f64, max: f64);
    fn set_value(&self, value: f64);
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
    fn add<F: FnMut(Self) + 'static>(&self, label: &str, func: F) -> super::WidgetItem;
    fn first(&self) -> super::WidgetItem;
    fn last(&self) -> super::WidgetItem;
    fn selected(&self) -> super::WidgetItem;
    fn value(&self) -> u32;
    fn clear(&self);
    fn set_value(&self, value: u32);
    fn add_items<F: FnMut(Self) + 'static + Clone>(&self, items: &[&str], func: F) {
        for item in items {
            self.add(item, func.clone());
        }
    }
    fn with_item<F: FnMut(Self) + 'static>(self, label: &str, func: F) -> Self {
        self.add(label, func);
        self
    }
    fn with_items<F: FnMut(Self) + 'static + Clone>(self, items: &[&str], func: F) -> Self {
        self.add_items(items, func);
        self
    }
}

pub trait ActionSliderExt: ElmObject {
    #[deprecated = "use refl::SegmentControl::new(&parent) instead"]
    fn new(prt: &impl ContainerExt, left: &str, center: &str, right: &str) -> Self {
        let elm = Self::from_raw(unsafe { elm_actionslider_add(prt.as_raw()) });
        elm.conf();
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

pub trait CtxpopupExt: ElmObject {
    #[deprecated = "rse refl::Notify::new(&parent) instead"]
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_ctxpopup_add(prt.as_raw()) }).with_conf();
        prt.add(&elm);
        elm
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

pub trait HoverSelExt: ElmObject {
    #[deprecated = "use refl::FlipSelector::new(&parent) instead"]
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_hoversel_add(prt.as_raw()) }).with_conf();
        elm.set_auto_update(true);
        prt.add(&elm);
        elm
    }
    fn append<F: FnMut(Self) + 'static>(
        &self,
        icon_: &str,
        label_: &str,
        func: F,
    ) -> super::WidgetItem {
        let raw_ptr: *mut Box<dyn FnMut(Self)> = Box::into_raw(Box::new(Box::new(func)));
        super::WidgetItem::from_raw(unsafe {
            elm_hoversel_item_add(
                self.as_raw(),
                CString::new(label_).unwrap().as_ptr(),
                CString::new(icon_).unwrap().as_ptr(),
                2,
                Some(smart_cb::<Self>),
                raw_ptr as *mut c_void,
            )
        })
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

pub trait FlipSelExt: SelectorExt {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_flipselector_add(prt.as_raw()) })
            .with_conf()
            .with_interval(3.0);
        prt.add(&elm);
        elm
    }
    fn find(&self, item: super::WidgetItem) -> u32 {
        let mut count = 0;
        let mut temp = self.first().as_raw();
        while temp != item.as_raw() {
            temp = unsafe { elm_flipselector_item_next_get(temp) };
            count += 1;
        }
        count
    }
    fn first_item(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe {
            refl_sys::elm_flipselector_first_item_get(self.as_raw())
        })
    }
    fn clear_items(&self) {
        let mut temp = self.first().as_raw();
        while temp != self.last().as_raw() {
            super::WidgetItem::from_raw(temp).del();
            temp = unsafe { refl_sys::elm_flipselector_item_next_get(temp) };
        }
        self.last().del();
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
    fn set_index(&self, value: u32) {
        let mut temp = self.first().as_raw();
        for _ in 0..value {
            temp = unsafe { elm_flipselector_item_next_get(temp) };
        }
        unsafe { elm_flipselector_item_selected_set(temp, true as Eina_Bool) }
    }
    fn index(&self) -> u32 {
        self.find(self.selected())
    }
    fn last_item(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_flipselector_last_item_get(self.as_raw()) })
    }
    fn set_interval(&self, value: f64) {
        unsafe { elm_flipselector_first_interval_set(self.as_raw(), value) }
    }
    fn with_interval(self, value: f64) -> Self {
        self.set_interval(value);
        self
    }
    fn lenght(&self) -> u32 {
        self.find(self.last())
    }
}

pub trait EntryExt: ElmObject {
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
        self.entry()
    }
    fn editable(&self) -> bool {
        unsafe { elm_entry_editable_get(self.as_raw()) != 0 }
    }
    fn entry(&self) -> String {
        unsafe {
            let ptr = elm_entry_entry_get(self.as_raw());
            CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
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
        let ctext = CString::new(value).unwrap();
        unsafe { elm_icon_standard_set(self.as_raw(), ctext.as_ptr()) };
    }
}

pub trait SeparatorExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_separator_add(prt.as_raw()) });
        elm.conf();
        prt.add(&elm);
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

pub trait LayoutExt: ContainerExt {
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
    fn set_virtual_size(&self, w: i32, h: i32) {
        unsafe { elm_grid_size_set(self.as_raw(), w, h) };
    }
    fn with_virtual_size(self, w: i32, h: i32) -> Self {
        self.set_virtual_size(w, h);
        self
    }
    fn virtual_size(&self) -> (i32, i32) {
        let (mut w, mut h) = (0i32, 0i32);
        unsafe { elm_grid_size_get(self.as_raw(), &mut w, &mut h) };
        (w, h)
    }
    fn pack(&self, subobj: &impl ElmObject, x: i32, y: i32, w: i32, h: i32) {
        unsafe { elm_grid_pack(self.as_raw(), subobj.as_raw(), x, y, w, h) };
    }
    fn with_pack(self, subobj: &impl ElmObject, x: i32, y: i32, w: i32, h: i32) -> Self {
        self.pack(subobj, x, y, w, h);
        self
    }
    fn unpack(&self, subobj: &impl ElmObject) {
        unsafe { elm_grid_unpack(self.as_raw(), subobj.as_raw()) };
    }
    fn clear(&self, clear_items: bool) {
        unsafe { elm_grid_clear(self.as_raw(), clear_items as Eina_Bool) };
    }
    fn set_pack(&self, subobj: &impl ElmObject, x: i32, y: i32, w: i32, h: i32) {
        unsafe { elm_grid_pack_set(subobj.as_raw(), x, y, w, h) };
    }
    fn pack_get(&self, subobj: &impl ElmObject) -> (i32, i32, i32, i32) {
        let (mut x, mut y, mut w, mut h) = (0i32, 0i32, 0i32, 0i32);
        unsafe { elm_grid_pack_get(subobj.as_raw(), &mut x, &mut y, &mut w, &mut h) };
        (x, y, w, h)
    }
}

pub trait TableExt: ContainerExt {
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
    fn unpack(&self, subobj: &impl ElmObject) {
        unsafe { elm_table_unpack(self.as_raw(), subobj.as_raw()) };
    }
    fn set_homogeneous(&self, value: bool) {
        unsafe { elm_table_homogeneous_set(self.as_raw(), value as Eina_Bool) };
    }
    fn with_homogeneous(self, value: bool) -> Self {
        self.set_homogeneous(value);
        self
    }
    fn homogeneous(&self) -> bool {
        unsafe { elm_table_homogeneous_get(self.as_raw()) != 0 }
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
    fn append<F: FnMut(Self) + 'static>(
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
    fn find(&self, item: super::WidgetItem) -> u32 {
        let mut count = 0;
        let mut temp = self.first().as_raw();
        while temp != item.as_raw() {
            temp = unsafe { elm_list_item_next(temp) };
            count += 1;
        }
        count
    }
    fn set_index(&self, value: u32) {
        let mut temp = self.first().as_raw();
        for _ in 0..value {
            temp = unsafe { elm_list_item_next(temp) };
        }
        unsafe { elm_list_item_selected_set(temp, true as Eina_Bool) }
    }
    fn lenght(&self) -> u32 {
        self.find(self.last())
    }
    fn index(&self) -> u32 {
        self.find(self.selected())
    }
    fn value(&self) -> u32 {
        self.index()
    }
}

pub trait NaviframeExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_naviframe_add(prt.as_raw()) });
        elm.set_prev(false);
        elm.conf();
        prt.add(&elm);
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

pub trait PanelExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_panel_add(prt.as_raw()) });
        elm.conf();
        elm.set_orient(super::PanelOrient::Bottom);
        elm.set_hidden(true);
        prt.add(&elm);
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
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_panes_add(prt.as_raw()) });
        elm.conf();
        prt.add(&elm);
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

pub trait PopupExt: ContainerExt {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_popup_add(prt.window().as_raw()) });
        elm.conf();
        prt.add(&elm);
        elm
    }
    fn set_title_text(&self, value: &str) {
        self.set_part("title,text", value)
    }
    fn set_title_icon(&self, value: &super::Icon) {
        self.set_content(value, "title,icon")
    }
}

pub trait ProgressBarExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_progressbar_add(prt.as_raw()) });
        elm.conf();
        prt.add(&elm);
        elm
    }
    fn value(&self) -> f64 {
        unsafe { elm_progressbar_value_get(self.as_raw()) }
    }
    fn set_value(&self, value: f64) {
        unsafe { elm_progressbar_value_set(self.as_raw(), value) };
    }
    fn set_unit_format(&self, value: &str) {
        let ctext = CString::new(value).unwrap();
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
    fn item(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_radio_add(prt.as_raw()) });
        elm.conf();
        elm.set_state_value(0);
        prt.add(&elm);
        elm
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

pub trait ScrollerExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_scroller_add(prt.as_raw()) });
        elm.conf();
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

pub trait SegmentControlExt: ElmObject {
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_segment_control_add(prt.as_raw()) });
        elm.conf();
        prt.add(&elm);
        elm
    }
    fn selected(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_segment_control_item_selected_get(self.as_raw()) })
    }
    fn value(&self) -> u32 {
        self.index()
    }
    fn index(&self) -> u32 {
        unsafe { elm_segment_control_item_index_get(self.selected().as_raw()) as u32 }
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
            CStr::from_ptr(ptr).to_string_lossy().into_owned()
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
    fn add(&self, icon: &str, label: &str) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe {
            elm_segment_control_item_add(
                self.as_raw(),
                super::Icon::new(self).with_standard(icon).as_raw(),
                CString::new(label).unwrap().as_ptr(),
            )
        })
    }
}

pub trait ToolBarExt: ElmObject {
    #[deprecated = "use refl::SegmentControl::new(&parent) instead"]
    fn new(prt: &impl ContainerExt) -> Self {
        let elm = Self::from_raw(unsafe { elm_toolbar_add(prt.as_raw()) });
        elm.conf();
        elm.set_shrink_mode(super::Shrink::Menu);
        prt.add(&elm);
        elm
    }
    fn append<F: FnMut(Self) + 'static>(
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
        super::WidgetItem::from_raw(unsafe { elm_toolbar_selected_item_get(self.as_raw()) })
    }
    fn value(&self) -> u32 {
        self.index()
    }
    fn set_value(&self, value: u32) {
        self.set_index(value)
    }
    fn set_index(&self, value: u32) {
        let mut temp = self.first().as_raw();
        for _ in 0..value {
            temp = unsafe { elm_toolbar_item_next_get(temp) };
        }
        unsafe { elm_toolbar_item_selected_set(temp, true as Eina_Bool) }
    }
    fn index(&self) -> u32 {
        self.find(self.selected())
    }
    fn lenght(&self) -> u32 {
        self.find(self.last())
    }
    fn first(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_toolbar_first_item_get(self.as_raw()) })
    }
    fn last(&self) -> super::WidgetItem {
        super::WidgetItem::from_raw(unsafe { elm_toolbar_last_item_get(self.as_raw()) })
    }
    fn find(&self, item: super::WidgetItem) -> u32 {
        let mut count = 0;
        let mut temp = self.first().as_raw();
        while temp != item.as_raw() {
            temp = unsafe { elm_toolbar_item_next_get(temp) };
            count += 1;
        }
        count
    }
}

pub trait WindowExt: OnDeleteRequest {
    fn new(id_: &str, title_: &str) -> Self {
        let elm = Self::from_raw(unsafe {
            elm_win_util_standard_add(
                CString::new(id_).unwrap().as_ptr(),
                CString::new(title_).unwrap().as_ptr(),
            )
        });
        elm.resize(360, 640);
        elm.set_autodel(true);
        elm.set_center(true, true);
        elm.on_delete_request(|_| exit());
        elm
    }

    fn add_object(&self, child: &impl ElmObject) {
        unsafe { elm_win_resize_object_add(self.as_raw(), child.as_raw()) };
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

pub trait OnActivated: ElmObject {
    fn on_activated<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("activated", func);
    }
    fn with_activated<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.on_activated(func);
        self
    }
}

pub trait OnDone: ElmObject {
    fn on_done<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("done", func);
    }
    fn with_done<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.on_done(func);
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

pub trait OnClickedDouble: ElmObject {
    fn on_clicked_double<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("clicked,double", func);
    }
    fn with_clicked_double<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.on_clicked_double(func);
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

pub trait OnUnselected: ElmObject {
    fn on_unselected<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("unselected", func);
    }
    fn with_toggled<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.on_unselected(func);
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

pub trait OnClickedBlock: ElmObject {
    fn on_clicked_block<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("block,clicked", func);
    }
    fn with_clicked_block<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.on_clicked_block(func);
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

pub trait OnChangedDelay: ElmObject {
    fn on_changed_delay<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("delay,changed", func);
    }
    fn with_changed_delay<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.on_changed_delay(func);
        self
    }
}

pub trait OnChangedRight: ElmObject {
    fn on_clicked_right<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("clicked,right", func);
    }
    fn with_clicked_right<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.on_clicked_right(func);
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

pub trait OnPressedLong: ElmObject {
    fn on_pressed_long<F: FnMut(Self) + 'static>(&self, func: F) {
        self.smart_callback_add("longpressed", func);
    }
    fn with_pressed_long<F: FnMut(Self) + 'static>(self, func: F) -> Self {
        self.on_pressed_long(func);
        self
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
