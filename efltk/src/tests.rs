//! Comprehensive test suite for efl-rs widgets and traits.
//!
//! This module provides unit tests, integration tests, and documentation tests
//! for all widget types and traits in the EFL bindings.

#[cfg(test)]
mod widget_tests {
    use crate::prelude::*;
    use crate::{
        Box, Button, Check, Entry, Frame, Icon, Label, List, Menu, Naviframe, Panes, Popup,
        ProgressBar, Radio, SegmentControl, Slider, Spinner, Window,
    };

    // Note: These tests verify type safety and trait implementations.
    // Full functional tests require an EFL display environment.

    #[test]
    fn test_widget_types_exist() {
        // Verify all widget types are defined
        let _box = Box::default();
        let _button = Button::default();
        let _check = Check::default();
        let _entry = Entry::default();
        let _frame = Frame::default();
        let _icon = Icon::default();
        let _label = Label::default();
        let _list = List::default();
        let _menu = Menu::default();
        let _naviframe = Naviframe::default();
        let _panes = Panes::default();
        let _popup = Popup::default();
        let _progressbar = ProgressBar::default();
        let _radio = Radio::default();
        let _segment_control = SegmentControl::default();
        let _slider = Slider::default();
        let _spinner = Spinner::default();
        let _window = Window::default();
    }

    #[test]
    fn test_widget_ext_trait_bounds() {
        // Verify all widgets implement WidgetExt
        fn requires_widget_ext<T: WidgetExt>(_: &T) {}

        let box_widget = Box::default();
        let button = Button::default();
        let check = Check::default();
        let entry = Entry::default();
        let frame = Frame::default();
        let icon = Icon::default();
        let label = Label::default();
        let list = List::default();
        let menu = Menu::default();
        let naviframe = Naviframe::default();
        let panes = Panes::default();
        let popup = Popup::default();
        let progressbar = ProgressBar::default();
        let radio = Radio::default();
        let segment_control = SegmentControl::default();
        let slider = Slider::default();
        let spinner = Spinner::default();
        let window = Window::default();

        requires_widget_ext(&box_widget);
        requires_widget_ext(&button);
        requires_widget_ext(&check);
        requires_widget_ext(&entry);
        requires_widget_ext(&frame);
        requires_widget_ext(&icon);
        requires_widget_ext(&label);
        requires_widget_ext(&list);
        requires_widget_ext(&menu);
        requires_widget_ext(&naviframe);
        requires_widget_ext(&panes);
        requires_widget_ext(&popup);
        requires_widget_ext(&progressbar);
        requires_widget_ext(&radio);
        requires_widget_ext(&segment_control);
        requires_widget_ext(&slider);
        requires_widget_ext(&spinner);
        requires_widget_ext(&window);
    }

    #[test]
    fn test_container_ext_implementations() {
        // Verify container widgets implement ContainerExt
        fn requires_container_ext<T: ContainerExt>(_: &T) {}

        let box_widget = Box::default();
        let frame = Frame::default();
        let naviframe = Naviframe::default();
        let panes = Panes::default();
        let popup = Popup::default();
        let window = Window::default();

        requires_container_ext(&box_widget);
        requires_container_ext(&frame);
        requires_container_ext(&naviframe);
        requires_container_ext(&panes);
        requires_container_ext(&popup);
        requires_container_ext(&window);
    }

    #[test]
    fn test_orient_ext_implementations() {
        // Verify orientable widgets implement OrientExt
        fn requires_orient_ext<T: OrientExt>(_: &T) {}

        let box_widget = Box::default();
        let panes = Panes::default();
        let separator = Separator::default();
        let slider = Slider::default();

        requires_orient_ext(&box_widget);
        requires_orient_ext(&panes);
        requires_orient_ext(&separator);
        requires_orient_ext(&slider);
    }

    #[test]
    fn test_text_ext_implementations() {
        // Verify text widgets implement TextExt
        fn requires_text_ext<T: TextExt>(_: &T) {}

        let button = Button::default();
        let check = Check::default();
        let entry = Entry::default();
        let frame = Frame::default();
        let label = Label::default();
        let radio = Radio::default();

        requires_text_ext(&button);
        requires_text_ext(&check);
        requires_text_ext(&entry);
        requires_text_ext(&frame);
        requires_text_ext(&label);
        requires_text_ext(&radio);
    }

    #[test]
    fn test_input_ext_implementations() {
        // Verify input widgets implement InputExt with correct types
        fn requires_input_ext_bool<T: InputExt<bool>>(_: &T) {}
        fn requires_input_ext_i32<T: InputExt<i32>>(_: &T) {}
        fn requires_input_ext_f64<T: InputExt<f64>>(_: &T) {}
        fn requires_input_ext_string<T: InputExt<String>>(_: &T) {}

        let button = Button::default();
        let check = Check::default();
        let frame = Frame::default();

        let menu = Menu::default();
        let radio = Radio::default();
        let segment_control = SegmentControl::default();

        let entry = Entry::default();

        let slider = Slider::default();
        let spinner = Spinner::default();

        requires_input_ext_bool(&button);
        requires_input_ext_bool(&check);
        requires_input_ext_bool(&frame);

        requires_input_ext_i32(&menu);
        requires_input_ext_i32(&radio);
        requires_input_ext_i32(&segment_control);

        requires_input_ext_string(&entry);

        requires_input_ext_f64(&slider);
        requires_input_ext_f64(&spinner);
    }

    #[test]
    fn test_selector_ext_implementations() {
        // Verify selector widgets implement SelectorExt
        fn requires_selector_ext<T: SelectorExt>(_: &T) {}

        let list = List::default();
        let menu = Menu::default();
        let segment_control = SegmentControl::default();

        requires_selector_ext(&list);
        requires_selector_ext(&menu);
        requires_selector_ext(&segment_control);
    }

    #[test]
    fn test_ranger_ext_implementations() {
        // Verify range widgets implement RangerExt
        fn requires_ranger_ext<T: RangerExt>(_: &T) {}

        let slider = Slider::default();
        let spinner = Spinner::default();

        requires_ranger_ext(&slider);
        requires_ranger_ext(&spinner);
    }

    #[test]
    fn test_specialized_trait_implementations() {
        // Verify specialized trait implementations
        fn requires_box_ext<T: BoxExt>(_: &T) {}
        fn requires_button_ext<T: ButtonExt>(_: &T) {}
        fn requires_check_ext<T: CheckExt>(_: &T) {}
        fn requires_entry_ext<T: EntryExt>(_: &T) {}
        fn requires_frame_ext<T: FrameExt>(_: &T) {}
        fn requires_icon_ext<T: IconExt>(_: &T) {}
        fn requires_label_ext<T: LabelExt>(_: &T) {}
        fn requires_list_ext<T: ListExt>(_: &T) {}
        fn requires_menu_ext<T: MenuExt>(_: &T) {}
        fn requires_naviframe_ext<T: NaviframeExt>(_: &T) {}
        fn requires_panes_ext<T: PanesExt>(_: &T) {}
        fn requires_popup_ext<T: PopupExt>(_: &T) {}
        fn requires_progressbar_ext<T: ProgressbarExt>(_: &T) {}
        fn requires_radio_ext<T: RadioExt>(_: &T) {}
        fn requires_segment_control_ext<T: SegmentControlExt>(_: &T) {}
        fn requires_separator_ext<T: SeparatorExt>(_: &T) {}
        fn requires_slider_ext<T: SliderExt>(_: &T) {}
        fn requires_spinner_ext<T: SpinnerExt>(_: &T) {}
        fn requires_window_ext<T: WindowExt>(_: &T) {}

        let box_widget = Box::default();
        let button = Button::default();
        let check = Check::default();
        let entry = Entry::default();
        let frame = Frame::default();
        let icon = Icon::default();
        let label = Label::default();
        let list = List::default();
        let menu = Menu::default();
        let naviframe = Naviframe::default();
        let panes = Panes::default();
        let popup = Popup::default();
        let progressbar = ProgressBar::default();
        let radio = Radio::default();
        let segment_control = SegmentControl::default();
        let separator = Separator::default();
        let slider = Slider::default();
        let spinner = Spinner::default();
        let window = Window::default();

        requires_box_ext(&box_widget);
        requires_button_ext(&button);
        requires_check_ext(&check);
        requires_entry_ext(&entry);
        requires_frame_ext(&frame);
        requires_icon_ext(&icon);
        requires_label_ext(&label);
        requires_list_ext(&list);
        requires_menu_ext(&menu);
        requires_naviframe_ext(&naviframe);
        requires_panes_ext(&panes);
        requires_popup_ext(&popup);
        requires_progressbar_ext(&progressbar);
        requires_radio_ext(&radio);
        requires_segment_control_ext(&segment_control);
        requires_separator_ext(&separator);
        requires_slider_ext(&slider);
        requires_spinner_ext(&spinner);
        requires_window_ext(&window);
    }
}

#[cfg(test)]
mod trait_method_tests {
    use crate::error::CStringExt;
    use crate::prelude::*;

    #[test]
    fn test_signal_enum() {
        // Test Signal enum variants
        assert_eq!(Signal::Changed.as_ref(), "changed");
        assert_eq!(Signal::Clicked.as_ref(), "clicked");
        assert_eq!(Signal::Selected.as_ref(), "selected");
        assert_eq!(Signal::Unfocused.as_ref(), "unfocused");
    }

    #[test]
    fn test_align_enum_conversions() {
        // Test Align enum to f64 conversions
        assert_eq!(f64::from(Align::Fill), -1.0);
        assert_eq!(f64::from(Align::Left), 0.0);
        assert_eq!(f64::from(Align::Center), 0.5);
        assert_eq!(f64::from(Align::Right), 1.0);
    }

    #[test]
    fn test_cursor_enum() {
        // Test Cursor enum variants
        assert_eq!(Cursor::Hand1.as_ref(), "hand1");
        assert_eq!(Cursor::Hand2.as_ref(), "hand2");
        assert_eq!(Cursor::Hand3.as_ref(), "hand3");
        assert_eq!(Cursor::Bogocity.as_ref(), "bogocity");
        assert_eq!(Cursor::Xterm.as_ref(), "xterm");
    }

    #[test]
    fn test_cstring_extension_valid_strings() {
        // Test CStringExt with valid strings
        let result = "hello".to_cstring();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string_lossy(), "hello");

        let result = String::from("world").to_cstring();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string_lossy(), "world");
    }

    #[test]
    fn test_cstring_extension_invalid_string() {
        // Test CStringExt with invalid string (contains null byte)
        let result = "hello\0world".to_cstring();
        assert!(result.is_err());
    }

    #[test]
    fn test_cstring_expect_valid() {
        // Test expect_cstring with valid string
        let cstring = "valid_string".expect_cstring("test");
        assert_eq!(cstring.to_string_lossy(), "valid_string");
    }

    #[test]
    #[should_panic(expected = "Failed to create CString")]
    fn test_cstring_expect_invalid() {
        // Test expect_cstring with invalid string should panic
        "invalid\0string".expect_cstring("test context");
    }

    #[test]
    fn test_panel_orient_enum() {
        // Test PanelOrient enum variants
        let top = PanelOrient::Top as i32;
        let bottom = PanelOrient::Bottom as i32;
        let left = PanelOrient::Left as i32;
        let right = PanelOrient::Right as i32;

        assert_eq!(top, 0);
        assert!(bottom > top);
        assert!(left > bottom);
        assert!(right > left);
    }
}

#[cfg(test)]
mod timer_tests {
    use crate::Timer;
    use crate::prelude::*;

    #[test]
    fn test_timer_creation() {
        // Test that Timer can be created (requires EFL initialization for full test)
        // This test verifies the type exists and has the expected interface
        let timer = Timer::default();
        assert!(timer.0.is_none());
    }

    #[test]
    fn test_timer_from_raw() {
        // Test Timer::from_raw with null pointer
        let timer = Timer::from(std::ptr::null_mut());
        assert!(timer.0.is_none());
    }
}

#[cfg(test)]
mod widget_item_tests {
    use crate::WidgetItem;
    use crate::prelude::*;

    #[test]
    fn test_widget_item_default() {
        // Test WidgetItem default construction
        let item = WidgetItem::default();
        assert!(item.0.is_none());
    }

    #[test]
    fn test_widget_item_from_raw_null() {
        // Test WidgetItem::from_raw with null pointer
        let item = WidgetItem::from_raw(std::ptr::null_mut());
        assert!(item.0.is_none());
    }

    #[test]
    fn test_widget_item_as_raw_panics_on_empty() {
        // Test that as_raw panics on empty WidgetItem
        let item = WidgetItem::default();
        let result = std::panic::catch_unwind(|| {
            item.as_raw();
        });
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod tm_struct_tests {
    use crate::Tm;
    use efltk_sys::tm;

    #[test]
    fn test_tm_default() {
        // Test Tm default construction
        let tm = Tm::default();
        assert_eq!(tm.sec, 0);
        assert_eq!(tm.min, 0);
        assert_eq!(tm.hour, 0);
        assert_eq!(tm.mday, 0);
        assert_eq!(tm.mon, 0);
        assert_eq!(tm.year, 0);
        assert_eq!(tm.wday, 0);
        assert_eq!(tm.yday, 0);
        assert_eq!(tm.isdst, 0);
    }

    #[test]
    fn test_tm_to_tm_conversion() {
        // Test Tm to tm conversion
        let rust_tm = Tm {
            sec: 30,
            min: 15,
            hour: 10,
            mday: 5,
            mon: 6,
            year: 123,
            wday: 2,
            yday: 180,
            isdst: 1,
            #[cfg(target_os = "linux")]
            gmtoff: 3600,
            #[cfg(target_os = "linux")]
            zone: String::from("UTC"),
        };

        let c_tm = rust_tm.to_tm();
        assert_eq!(c_tm.tm_sec, 30);
        assert_eq!(c_tm.tm_min, 15);
        assert_eq!(c_tm.tm_hour, 10);
        assert_eq!(c_tm.tm_mday, 5);
        assert_eq!(c_tm.tm_mon, 6);
        assert_eq!(c_tm.tm_year, 123);
        assert_eq!(c_tm.tm_wday, 2);
        assert_eq!(c_tm.tm_yday, 180);
        assert_eq!(c_tm.tm_isdst, 1);
    }

    #[test]
    fn test_tm_from_tm_conversion() {
        // Test tm to Tm conversion
        let c_tm = tm {
            tm_sec: 45,
            tm_min: 30,
            tm_hour: 14,
            tm_mday: 15,
            tm_mon: 9,
            tm_year: 124,
            tm_wday: 5,
            tm_yday: 280,
            tm_isdst: 0,
            #[cfg(target_os = "linux")]
            tm_gmtoff: 7200,
            #[cfg(target_os = "linux")]
            tm_zone: std::ptr::null(),
        };

        let rust_tm = Tm::from_tm(c_tm);
        assert_eq!(rust_tm.sec, 45);
        assert_eq!(rust_tm.min, 30);
        assert_eq!(rust_tm.hour, 14);
        assert_eq!(rust_tm.mday, 15);
        assert_eq!(rust_tm.mon, 9);
        assert_eq!(rust_tm.year, 124);
        assert_eq!(rust_tm.wday, 5);
        assert_eq!(rust_tm.yday, 280);
        assert_eq!(rust_tm.isdst, 0);
    }

    #[test]
    fn test_tm_roundtrip() {
        // Test roundtrip conversion
        let original = Tm::default();
        let c_tm = original.to_tm();
        let converted = Tm::from_tm(c_tm);

        assert_eq!(original.sec, converted.sec);
        assert_eq!(original.min, converted.min);
        assert_eq!(original.hour, converted.hour);
    }
}

#[cfg(test)]
mod naviframe_tests {
    use crate::Naviframe;

    #[test]
    fn test_naviframe_default() {
        // Test Naviframe default construction
        let nav = Naviframe::default();
        assert!(nav.obj.is_none());
        assert_eq!(nav.lst.borrow().len(), 0);
    }

    #[test]
    fn test_naviframe_set_top_empty() {
        // Test set_top on empty Naviframe (should not panic)
        let nav = Naviframe::default();
        nav.set_top(0); // Should do nothing since list is empty
        assert_eq!(nav.lst.borrow().len(), 0);
    }

    #[test]
    fn test_naviframe_promote_empty() {
        // Test promote on empty Naviframe (should not panic)
        let nav = Naviframe::default();
        nav.promote(); // Should do nothing since list is empty
    }
}

#[cfg(test)]
mod integration_tests {
    use crate::prelude::*;
    use crate::{Box, Button, Entry, Label, Window};

    #[test]
    fn test_widget_chain_methods() {
        // Test method chaining on widgets
        let widget = Button::default();

        // Chain multiple builder methods
        let _chained = widget
            .with_defaults()
            .with_weight(true, true)
            .with_align(Align::Center, Align::Center);
    }

    #[test]
    fn test_container_add_pattern() {
        // Test container.add() pattern (without actual EFL initialization)
        let container = Box::default();
        let child = Button::default();

        // Verify the types are compatible (actual add requires EFL init)
        // This test ensures trait bounds are correct
        fn test_container_pattern<T: ContainerExt, U: WidgetExt>(container: &T, child: &U) {
            // Type checking only - actual add requires EFL
            let _ = (container, child);
        }

        test_container_pattern(&container, &child);
    }

    #[test]
    fn test_input_value_patterns() {
        // Test InputExt value patterns
        fn test_bool_input<T: InputExt<bool>>(widget: &T) {
            let _ = widget.value();
            widget.set_value(true);
        }

        fn test_i32_input<T: InputExt<i32>>(widget: &T) {
            let _ = widget.value();
            widget.set_value(42);
        }

        fn test_f64_input<T: InputExt<f64>>(widget: &T) {
            let _ = widget.value();
            widget.set_value(3.14);
        }

        fn test_string_input<T: InputExt<String>>(widget: &T) {
            let _ = widget.value();
            widget.set_value(String::from("test"));
        }

        test_bool_input(&Check::default());
        test_i32_input(&Radio::default());
        test_f64_input(&Slider::default());
        test_string_input(&Entry::default());
    }

    #[test]
    fn test_orient_builder_pattern() {
        // Test OrientExt builder pattern
        fn test_orient<T: OrientExt + Default>() {
            let widget = T::default();
            let _ = widget.with_horizontal(true);
        }

        test_orient::<Box>();
        test_orient::<Panes>();
        test_orient::<Separator>();
        test_orient::<Slider>();
    }

    #[test]
    fn test_text_builder_pattern() {
        // Test TextExt builder pattern
        fn test_text<T: TextExt + Default>() {
            let widget = T::default();
            let _ = widget.with_text("test label");
        }

        test_text::<Button>();
        test_text::<Label>();
        test_text::<Check>();
        test_text::<Radio>();
    }

    #[test]
    fn test_ranger_builder_pattern() {
        // Test RangerExt builder pattern
        fn test_ranger<T: RangerExt + Default>() {
            let widget = T::default();
            let _ = widget.with_format("%.2f");
            widget.set_range(0.0, 100.0);
            widget.set_step(1.0);
        }

        test_ranger::<Slider>();
        test_ranger::<Spinner>();
    }
}

#[cfg(test)]
mod callback_tests {
    use crate::Button;
    use crate::prelude::*;

    #[test]
    fn test_callback_type_safety() {
        // Test that callbacks can be attached (type checking only)
        let button = Button::default();

        // Verify callback signatures compile correctly
        let _with_callback = button.with_callback(|_w| {
            // Callback body
        });

        let _with_signal = button.with_signal(Signal::Clicked, |_w| {
            // Signal handler body
        });
    }

    #[test]
    fn test_signal_call_compiles() {
        // Test that signal calling compiles correctly
        let button = Button::default();

        // These should compile without errors
        button.call_signal(Signal::Changed);
        button.call_signal(Signal::Clicked);
        button.do_callback();
    }
}

#[cfg(test)]
mod geometry_tests {
    use crate::Button;
    use crate::prelude::*;

    #[test]
    fn test_geometry_method_exists() {
        // Test that geometry method exists and returns correct type
        let button = Button::default();

        // Without EFL init, geometry will be (0,0,0,0)
        let (x, y, w, h) = button.geometry();
        assert_eq!(x, 0);
        assert_eq!(y, 0);
        assert_eq!(w, 0);
        assert_eq!(h, 0);
    }

    #[test]
    fn test_size_hints() {
        // Test size hint methods
        let button = Button::default();

        let _chained = button
            .with_size(100, 50)
            .with_min_size(80, 40)
            .with_weight(true, false)
            .with_align(Align::Center, Align::Fill);
    }
}

#[cfg(test)]
mod content_tests {
    use crate::prelude::*;
    use crate::{Button, Icon};

    #[test]
    fn test_content_methods_compile() {
        // Test that content methods compile correctly
        let button = Button::default();
        let icon = Icon::default();

        // These should compile (actual setting requires EFL init)
        let _chained = button.with_content(&icon, "default");

        // Content retrieval should return Option
        let _content: Option<crate::WidgetItem> = button.content("default");
    }

    #[test]
    fn test_part_methods_compile() {
        // Test that part methods compile correctly
        let button = Button::default();

        let _chained = button.with_part("default", "button text");
    }
}

#[cfg(test)]
mod focus_tests {
    use crate::Button;
    use crate::prelude::*;

    #[test]
    fn test_focus_methods() {
        // Test focus methods
        let button = Button::default();

        // Without EFL init, focus should be false
        assert!(!button.focus());

        let _chained = button.with_focus(true);
    }
}

#[cfg(test)]
mod tooltip_tests {
    use crate::Button;
    use crate::prelude::*;

    #[test]
    fn test_tooltip_methods() {
        // Test tooltip methods
        let button = Button::default();

        let _chained = button.with_tooltip("This is a tooltip");
    }

    #[test]
    fn test_cursor_methods() {
        // Test cursor methods
        let button = Button::default();

        let _chained = button.with_cursor(Cursor::Hand1);
    }
}

#[cfg(test)]
mod disabled_tests {
    use crate::Button;
    use crate::prelude::*;

    #[test]
    fn test_disabled_methods() {
        // Test disabled methods
        let button = Button::default();

        // Without EFL init, should be false
        assert!(!button.disabled());

        let _chained = button.with_disabled(true);
        assert!(button.disabled());
    }
}

#[cfg(test)]
mod selector_tests {
    use crate::prelude::*;
    use crate::{List, Menu, SegmentControl};

    #[test]
    fn test_selector_length() {
        // Test selector length on empty selectors
        let list = List::default();
        let menu = Menu::default();
        let segment = SegmentControl::default();

        assert_eq!(list.length(), 0);
        assert_eq!(menu.length(), 0);
        assert_eq!(segment.length(), 0);
    }

    #[test]
    fn test_selector_clear() {
        // Test clear on empty selectors (should not panic)
        let list = List::default();
        let menu = Menu::default();
        let segment = SegmentControl::default();

        list.clear();
        menu.clear();
        segment.clear();
    }
}

#[cfg(test)]
mod icon_tests {
    use crate::Icon;
    use crate::prelude::*;

    #[test]
    fn test_icon_with_standard() {
        // Test Icon::with_standard method signature
        let icon = Icon::default();

        // This should compile (actual icon setting requires EFL)
        let _chained = icon.with_standard("icon-name");
    }
}

#[cfg(test)]
mod list_tests {
    use crate::List;
    use crate::prelude::*;

    #[test]
    fn test_list_selected() {
        // Test list selected item on empty list
        let list = List::default();
        let selected = list.selected();
        assert!(selected.0.is_none());
    }

    #[test]
    fn test_list_first() {
        // Test list first item on empty list
        let list = List::default();
        let first = list.first();
        assert!(first.0.is_none());
    }

    #[test]
    fn test_list_value_empty() {
        // Test list value on empty list
        let list = List::default();
        assert_eq!(list.value(), -1);
    }
}

#[cfg(test)]
mod menu_tests {
    use crate::Menu;
    use crate::prelude::*;

    #[test]
    fn test_menu_selected() {
        // Test menu selected item on empty menu
        let menu = Menu::default();
        let selected = menu.selected();
        assert!(selected.0.is_none());
    }

    #[test]
    fn test_menu_first() {
        // Test menu first item on empty menu
        let menu = Menu::default();
        let first = menu.first();
        assert!(first.0.is_none());
    }

    #[test]
    fn test_menu_value_empty() {
        // Test menu value on empty menu
        let menu = Menu::default();
        assert_eq!(menu.value(), 0);
    }
}

#[cfg(test)]
mod segment_control_tests {
    use crate::SegmentControl;
    use crate::prelude::*;

    #[test]
    fn test_segment_control_selected() {
        // Test segment control selected item when empty
        let segment = SegmentControl::default();
        let selected = segment.selected();
        assert!(selected.0.is_none());
    }
}

#[cfg(test)]
mod panes_tests {
    use crate::Panes;
    use crate::prelude::*;

    #[test]
    fn test_panes_content_parts() {
        // Test panes content parts
        let panes = Panes::default();

        // Initially both parts should be None
        let left = panes.content("left");
        let right = panes.content("right");

        assert!(left.is_none());
        assert!(right.is_none());
    }
}

#[cfg(test)]
mod error_handling_tests {
    use crate::error::{CStringExt, EflError, EflResult};
    use std::ffi::CString;

    #[test]
    fn test_cstring_extension_on_str() {
        let result: EflResult<CString> = "hello".to_cstring();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string_lossy(), "hello");
    }

    #[test]
    fn test_cstring_extension_on_string() {
        let s = String::from("world");
        let result: EflResult<CString> = s.to_cstring();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string_lossy(), "world");
    }

    #[test]
    fn test_cstring_error_propagation() {
        let bad = "bad\0string";
        let result: EflResult<CString> = bad.to_cstring();
        assert!(result.is_err());

        if let Err(EflError::NullByte(_)) = result {
            // Expected
        } else {
            panic!("Expected NullByte error");
        }
    }

    #[test]
    fn test_nonnull_from_ptr_success() {
        use crate::error::nonnull_from_ptr;
        use std::ptr;

        let mut value = 42;
        let result = nonnull_from_ptr(&mut value, "test context");
        assert!(result.is_ok());
    }

    #[test]
    fn test_nonnull_from_ptr_failure() {
        use crate::error::nonnull_from_ptr;

        let ptr: *mut i32 = std::ptr::null_mut();
        let result = nonnull_from_ptr(ptr, "test context");
        assert!(result.is_err());

        if let Err(EflError::NullWidget(ctx)) = result {
            assert_eq!(ctx, "test context");
        } else {
            panic!("Expected NullWidget error");
        }
    }

    #[test]
    fn test_nonnull_from_option_success() {
        use crate::error::nonnull_from_option;
        use std::ptr::NonNull;

        let mut value = 42;
        let opt = NonNull::new(&mut value);
        let result = nonnull_from_option(opt, "test context");
        assert!(result.is_ok());
    }

    #[test]
    fn test_nonnull_from_option_failure() {
        use crate::error::nonnull_from_option;
        use std::ptr::NonNull;

        let opt: Option<NonNull<i32>> = None;
        let result = nonnull_from_option(opt, "test context");
        assert!(result.is_err());

        if let Err(EflError::EmptyWidget(ctx)) = result {
            assert_eq!(ctx, "test context");
        } else {
            panic!("Expected EmptyWidget error");
        }
    }
}

// Documentation tests
#[cfg(doctest)]
/// Example of creating a basic window with a button
///
/// ```ignore
/// use efltk::{prelude::*, Window, Button};
///
/// fn main() {
///     run(|| {
///         let win = Window::new()
///             .with_title("Hello")
///             .with_size(300, 200);
///         
///         let btn = Button::new(&win)
///             .with_text("Click me")
///             .with_callback(|_| {
///                 println!("Button clicked!");
///             });
///         
///         win
///     });
/// }
/// ```
pub mod doc_tests {}
