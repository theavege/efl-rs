//! Unit tests for efl-rs widgets and traits.
//!
//! These tests verify types, trait bounds, and empty-wrapper behaviour.
//! They do not require a running EFL display.

#[cfg(test)]
mod widget_tests {
    use crate::prelude::*;
    use crate::{
        Actionslider, Bg, Box, Bubble, Button, Calendar, Check, Clock, ColorSelector, Combobox,
        Conformant, Ctxpopup, Datetime, Dayselector, Diskselector, Entry, FileEntry, FileSelector,
        Flip, Frame, Gengrid, Genlist, Glview, Grid, Hover, Hoversel, Icon, Image, Index, Label,
        Layout, List, Map, Menu, Multibuttonentry, Naviframe, Notify, Panel, Panes, Photo,
        Photocam, Popup, ProgressBar, Radio, Scroller, SegmentControl, Separator, Slider,
        Slideshow, Spinner, Table, Toolbar, Video, Web, Window,
    };

    #[test]
    fn test_widget_types_exist() {
        let _ = (
            Box::default(),
            Button::default(),
            Calendar::default(),
            Check::default(),
            Clock::default(),
            ColorSelector::default(),
            Entry::default(),
            FileEntry::default(),
            FileSelector::default(),
            Frame::default(),
            Icon::default(),
            Image::default(),
            Label::default(),
            List::default(),
            Menu::default(),
            Naviframe::default(),
            Panes::default(),
            Popup::default(),
            ProgressBar::default(),
            Radio::default(),
            Scroller::default(),
            SegmentControl::default(),
            Separator::default(),
            Slider::default(),
            Spinner::default(),
            Table::default(),
            Window::default(),
        );
    }

    #[test]
    fn test_default_widgets_are_unset() {
        assert!(!Box::default().is_set());
        assert!(!Button::default().is_set());
        assert!(!Calendar::default().is_set());
        assert!(!Check::default().is_set());
        assert!(!Clock::default().is_set());
        assert!(!ColorSelector::default().is_set());
        assert!(!Entry::default().is_set());
        assert!(!FileEntry::default().is_set());
        assert!(!FileSelector::default().is_set());
        assert!(!Frame::default().is_set());
        assert!(!Icon::default().is_set());
        assert!(!Image::default().is_set());
        assert!(!Label::default().is_set());
        assert!(!List::default().is_set());
        assert!(!Menu::default().is_set());
        assert!(!Naviframe::default().is_set());
        assert!(!Panes::default().is_set());
        assert!(!Popup::default().is_set());
        assert!(!ProgressBar::default().is_set());
        assert!(!Radio::default().is_set());
        assert!(!Scroller::default().is_set());
        assert!(!SegmentControl::default().is_set());
        assert!(!Separator::default().is_set());
        assert!(!Slider::default().is_set());
        assert!(!Spinner::default().is_set());
        assert!(!Table::default().is_set());
        assert!(!Window::default().is_set());
    }

    #[test]
    fn test_widget_ext_trait_bounds() {
        fn requires_widget_ext<T: WidgetExt>(_: &T) {}

        requires_widget_ext(&Box::default());
        requires_widget_ext(&Button::default());
        requires_widget_ext(&Calendar::default());
        requires_widget_ext(&Check::default());
        requires_widget_ext(&Clock::default());
        requires_widget_ext(&ColorSelector::default());
        requires_widget_ext(&Entry::default());
        requires_widget_ext(&FileEntry::default());
        requires_widget_ext(&FileSelector::default());
        requires_widget_ext(&Frame::default());
        requires_widget_ext(&Icon::default());
        requires_widget_ext(&Image::default());
        requires_widget_ext(&Label::default());
        requires_widget_ext(&List::default());
        requires_widget_ext(&Menu::default());
        requires_widget_ext(&Naviframe::default());
        requires_widget_ext(&Panes::default());
        requires_widget_ext(&Popup::default());
        requires_widget_ext(&ProgressBar::default());
        requires_widget_ext(&Radio::default());
        requires_widget_ext(&Scroller::default());
        requires_widget_ext(&SegmentControl::default());
        requires_widget_ext(&Separator::default());
        requires_widget_ext(&Slider::default());
        requires_widget_ext(&Spinner::default());
        requires_widget_ext(&Table::default());
        requires_widget_ext(&Window::default());
    }

    #[test]
    fn test_widget_ext_tooltip_cursor_disabled() {
        fn uses_hygiene<T: WidgetExt>(w: &T) {
            let _ = w.disabled();
        }
        uses_hygiene(&Label::default());
        uses_hygiene(&Button::default());
        uses_hygiene(&ProgressBar::default());
        uses_hygiene(&Window::default());

        let label = Label::default();
        label.set_tooltip("hint");
        assert!(!label.set_cursor(Cursor::Hand1));
        label.set_disabled(true);
        assert!(!label.disabled());
        let _ = label
            .with_tooltip("hint")
            .with_cursor(Cursor::Xterm)
            .with_disabled(false);
    }

    #[test]
    fn test_container_ext_implementations() {
        fn requires_container_ext<T: ContainerExt>(_: &T) {}

        requires_container_ext(&Box::default());
        requires_container_ext(&Frame::default());
        requires_container_ext(&Naviframe::default());
        requires_container_ext(&Panes::default());
        requires_container_ext(&Popup::default());
        requires_container_ext(&Scroller::default());
        requires_container_ext(&Table::default());
        requires_container_ext(&Window::default());
    }

    #[test]
    fn test_empty_table_scroller_image_are_noop() {
        let table = Table::default();
        table.pack(&Button::default(), 0, 0, 1, 1);
        table.unpack(&Button::default());
        table.clear(false);
        assert!(!table.child_at::<Button>(0, 0).is_set());

        Scroller::default().set_policy(ScrollPolicy::Off, ScrollPolicy::Off);
        Scroller::default().region_show(0, 0, 1, 1);
        let _ = Scroller::default().with_bounce(false, false);

        assert!(!Image::default().set_file("missing.png"));
        assert_eq!(Image::default().object_size(), (0, 0));
    }

    #[test]
    fn test_orient_ext_implementations() {
        fn requires_orient_ext<T: OrientExt>(_: &T) {}

        requires_orient_ext(&Box::default());
        requires_orient_ext(&Panes::default());
        requires_orient_ext(&Separator::default());
        requires_orient_ext(&Slider::default());
    }

    #[test]
    fn test_text_ext_implementations() {
        fn requires_text_ext<T: TextExt>(_: &T) {}

        requires_text_ext(&Button::default());
        requires_text_ext(&Check::default());
        requires_text_ext(&Entry::default());
        requires_text_ext(&Frame::default());
        requires_text_ext(&Label::default());
        requires_text_ext(&Radio::default());
    }

    #[test]
    fn test_input_ext_implementations() {
        fn requires_input_ext_bool<T: InputExt<bool>>(_: &T) {}
        fn requires_input_ext_i32<T: InputExt<i32>>(_: &T) {}
        fn requires_input_ext_f64<T: InputExt<f64>>(_: &T) {}
        fn requires_input_ext_string<T: InputExt<String>>(_: &T) {}
        fn requires_input_ext_color<T: InputExt<(i32, i32, i32, i32)>>(_: &T) {}

        requires_input_ext_bool(&Button::default());
        requires_input_ext_bool(&Check::default());
        requires_input_ext_bool(&Frame::default());

        requires_input_ext_i32(&List::default());
        requires_input_ext_i32(&Menu::default());
        requires_input_ext_i32(&Radio::default());
        requires_input_ext_i32(&SegmentControl::default());

        requires_input_ext_string(&Entry::default());

        requires_input_ext_f64(&Slider::default());
        requires_input_ext_f64(&Spinner::default());

        requires_input_ext_color(&ColorSelector::default());
    }

    #[test]
    fn test_selector_ext_implementations() {
        fn requires_selector_ext<T: SelectorExt>(_: &T) {}

        requires_selector_ext(&List::default());
        requires_selector_ext(&Menu::default());
        requires_selector_ext(&SegmentControl::default());
    }

    #[test]
    fn test_ranger_ext_implementations() {
        fn requires_ranger_ext<T: RangerExt>(_: &T) {}

        requires_ranger_ext(&Slider::default());
        requires_ranger_ext(&Spinner::default());
    }

    #[test]
    fn test_specialized_trait_implementations() {
        fn requires_box_ext<T: BoxExt>(_: &T) {}
        fn requires_button_ext<T: ButtonExt>(_: &T) {}
        fn requires_calendar_ext<T: CalendarExt>(_: &T) {}
        fn requires_check_ext<T: CheckExt>(_: &T) {}
        fn requires_clock_ext<T: ClockExt>(_: &T) {}
        fn requires_color_sel_ext<T: ColorSelExt>(_: &T) {}
        fn requires_entry_ext<T: EntryExt>(_: &T) {}
        fn requires_file_entry_ext<T: FileEntryExt>(_: &T) {}
        fn requires_file_sel_ext<T: FileSelExt>(_: &T) {}
        fn requires_frame_ext<T: FrameExt>(_: &T) {}
        fn requires_icon_ext<T: IconExt>(_: &T) {}
        fn requires_image_ext<T: ImageExt>(_: &T) {}
        fn requires_label_ext<T: LabelExt>(_: &T) {}
        fn requires_list_ext<T: ListExt>(_: &T) {}
        fn requires_menu_ext<T: MenuExt>(_: &T) {}
        fn requires_naviframe_ext<T: NaviframeExt>(_: &T) {}
        fn requires_panes_ext<T: PanesExt>(_: &T) {}
        fn requires_popup_ext<T: PopupExt>(_: &T) {}
        fn requires_progress_bar_ext<T: ProgressBarExt>(_: &T) {}
        fn requires_radio_ext<T: RadioExt>(_: &T) {}
        fn requires_scroller_ext<T: ScrollerExt>(_: &T) {}
        fn requires_segment_control_ext<T: SegmentControlExt>(_: &T) {}
        fn requires_separator_ext<T: SeparatorExt>(_: &T) {}
        fn requires_slider_ext<T: SliderExt>(_: &T) {}
        fn requires_spinner_ext<T: SpinnerExt>(_: &T) {}
        fn requires_table_ext<T: TableExt>(_: &T) {}
        fn requires_window_ext<T: WindowExt>(_: &T) {}

        requires_box_ext(&Box::default());
        requires_button_ext(&Button::default());
        requires_calendar_ext(&Calendar::default());
        requires_check_ext(&Check::default());
        requires_clock_ext(&Clock::default());
        requires_color_sel_ext(&ColorSelector::default());
        requires_entry_ext(&Entry::default());
        requires_file_entry_ext(&FileEntry::default());
        requires_file_sel_ext(&FileSelector::default());
        requires_frame_ext(&Frame::default());
        requires_icon_ext(&Icon::default());
        requires_image_ext(&Image::default());
        requires_label_ext(&Label::default());
        requires_list_ext(&List::default());
        requires_menu_ext(&Menu::default());
        requires_naviframe_ext(&Naviframe::default());
        requires_panes_ext(&Panes::default());
        requires_popup_ext(&Popup::default());
        requires_progress_bar_ext(&ProgressBar::default());
        requires_radio_ext(&Radio::default());
        requires_scroller_ext(&Scroller::default());
        requires_segment_control_ext(&SegmentControl::default());
        requires_separator_ext(&Separator::default());
        requires_slider_ext(&Slider::default());
        requires_spinner_ext(&Spinner::default());
        requires_table_ext(&Table::default());
        requires_window_ext(&Window::default());
    }

    #[test]
    fn test_alpha8_remaining_widgets() {
        fn requires_widget_ext<T: WidgetExt>(_: &T) {}
        fn requires_container_ext<T: ContainerExt>(_: &T) {}
        fn requires_selector_ext<T: SelectorExt>(_: &T) {}
        fn requires_bg_ext<T: BgExt>(_: &T) {}
        fn requires_panel_ext<T: PanelExt>(_: &T) {}
        fn requires_notify_ext<T: NotifyExt>(_: &T) {}
        fn requires_photo_ext<T: PhotoExt>(_: &T) {}
        fn requires_datetime_ext<T: DatetimeExt>(_: &T) {}
        fn requires_hoversel_ext<T: HoverselExt>(_: &T) {}
        fn requires_diskselector_ext<T: DiskselectorExt>(_: &T) {}
        fn requires_toolbar_ext<T: ToolbarExt>(_: &T) {}
        fn requires_genlist_ext<T: GenlistExt>(_: &T) {}
        fn requires_input_tm<T: InputExt<crate::Tm>>(_: &T) {}

        let widgets = (
            Bg::default(),
            Datetime::default(),
            Diskselector::default(),
            Genlist::default(),
            Hoversel::default(),
            Notify::default(),
            Panel::default(),
            Photo::default(),
            Toolbar::default(),
        );
        assert!(!widgets.0.is_set());
        assert!(!widgets.1.is_set());
        assert!(!widgets.2.is_set());
        assert!(!widgets.3.is_set());
        assert!(!widgets.4.is_set());
        assert!(!widgets.5.is_set());
        assert!(!widgets.6.is_set());
        assert!(!widgets.7.is_set());
        assert!(!widgets.8.is_set());

        requires_widget_ext(&widgets.0);
        requires_container_ext(&Notify::default());
        requires_container_ext(&Panel::default());
        requires_selector_ext(&Diskselector::default());
        requires_selector_ext(&Genlist::default());
        requires_selector_ext(&Toolbar::default());
        requires_bg_ext(&Bg::default());
        requires_panel_ext(&Panel::default());
        requires_notify_ext(&Notify::default());
        requires_photo_ext(&Photo::default());
        requires_datetime_ext(&Datetime::default());
        requires_hoversel_ext(&Hoversel::default());
        requires_diskselector_ext(&Diskselector::default());
        requires_toolbar_ext(&Toolbar::default());
        requires_genlist_ext(&Genlist::default());
        requires_input_tm(&Datetime::default());

        assert!(!Bg::default().set_file("missing.png"));
        Panel::default().toggle();
        assert!(!Panel::default().hidden());
        Notify::default().set_timeout(1.0);
        assert!(!Photo::default().set_file("missing.png"));
        DatetimeExt::set_value(&Datetime::default(), crate::Tm::default());
        Diskselector::default().clear();
        Hoversel::default().clear();
        Toolbar::default().clear();
        Genlist::default().clear();
        assert_eq!(Genlist::default().length(), 0);
        assert_eq!(Hoversel::default().length(), 0);
        assert_eq!(Diskselector::default().value(), -1);
        assert_eq!(Toolbar::default().value(), -1);
        assert_eq!(Genlist::default().value(), -1);
    }

    #[test]
    fn test_alpha8_more_widgets() {
        fn requires_widget_ext<T: WidgetExt>(_: &T) {}
        fn requires_container_ext<T: ContainerExt>(_: &T) {}
        fn requires_grid_ext<T: GridExt>(_: &T) {}
        fn requires_flip_ext<T: FlipExt>(_: &T) {}
        fn requires_hover_ext<T: HoverExt>(_: &T) {}
        fn requires_ctxpopup_ext<T: CtxpopupExt>(_: &T) {}
        fn requires_index_ext<T: IndexExt>(_: &T) {}
        fn requires_dayselector_ext<T: DayselectorExt>(_: &T) {}
        fn requires_actionslider_ext<T: ActionsliderExt>(_: &T) {}
        fn requires_bubble_ext<T: BubbleExt>(_: &T) {}
        fn requires_photocam_ext<T: PhotocamExt>(_: &T) {}

        assert!(!Actionslider::default().is_set());
        assert!(!Bubble::default().is_set());
        assert!(!Ctxpopup::default().is_set());
        assert!(!Dayselector::default().is_set());
        assert!(!Flip::default().is_set());
        assert!(!Grid::default().is_set());
        assert!(!Hover::default().is_set());
        assert!(!Index::default().is_set());
        assert!(!Photocam::default().is_set());

        requires_widget_ext(&Index::default());
        requires_container_ext(&Grid::default());
        requires_container_ext(&Flip::default());
        requires_container_ext(&Hover::default());
        requires_container_ext(&Bubble::default());
        requires_grid_ext(&Grid::default());
        requires_flip_ext(&Flip::default());
        requires_hover_ext(&Hover::default());
        requires_ctxpopup_ext(&Ctxpopup::default());
        requires_index_ext(&Index::default());
        requires_dayselector_ext(&Dayselector::default());
        requires_actionslider_ext(&Actionslider::default());
        requires_bubble_ext(&Bubble::default());
        requires_photocam_ext(&Photocam::default());

        Grid::default().pack(&Button::default(), 0, 0, 50, 50);
        Grid::default().clear(false);
        Flip::default().go(FlipMode::RotateY);
        Hover::default().set_target(&Button::default());
        Ctxpopup::default().dismiss();
        Index::default().clear();
        assert!(!Dayselector::default().day_selected(Weekday::Mon));
        Actionslider::default().set_magnet(ActionPos::All);
        assert!(!Photocam::default().set_file("missing.png"));
    }

    #[test]
    fn test_alpha8_heavy_widgets() {
        fn requires_widget_ext<T: WidgetExt>(_: &T) {}
        fn requires_container_ext<T: ContainerExt>(_: &T) {}
        fn requires_selector_ext<T: SelectorExt>(_: &T) {}
        fn requires_gengrid_ext<T: GengridExt>(_: &T) {}
        fn requires_slideshow_ext<T: SlideshowExt>(_: &T) {}
        fn requires_map_ext<T: MapExt>(_: &T) {}
        fn requires_video_ext<T: VideoExt>(_: &T) {}
        fn requires_web_ext<T: WebExt>(_: &T) {}
        fn requires_glview_ext<T: GlviewExt>(_: &T) {}
        fn requires_conformant_ext<T: ConformantExt>(_: &T) {}
        fn requires_layout_ext<T: LayoutExt>(_: &T) {}
        fn requires_multibuttonentry_ext<T: MultibuttonentryExt>(_: &T) {}
        fn requires_combobox_ext<T: ComboboxExt>(_: &T) {}

        assert!(!Combobox::default().is_set());
        assert!(!Conformant::default().is_set());
        assert!(!Gengrid::default().is_set());
        assert!(!Glview::default().is_set());
        assert!(!Layout::default().is_set());
        assert!(!Map::default().is_set());
        assert!(!Multibuttonentry::default().is_set());
        assert!(!Slideshow::default().is_set());
        assert!(!Video::default().is_set());
        assert!(!Web::default().is_set());

        requires_widget_ext(&Web::default());
        requires_container_ext(&Conformant::default());
        requires_container_ext(&Layout::default());
        requires_selector_ext(&Gengrid::default());
        requires_selector_ext(&Multibuttonentry::default());
        requires_selector_ext(&Combobox::default());
        requires_gengrid_ext(&Gengrid::default());
        requires_slideshow_ext(&Slideshow::default());
        requires_map_ext(&Map::default());
        requires_video_ext(&Video::default());
        requires_web_ext(&Web::default());
        requires_glview_ext(&Glview::default());
        requires_conformant_ext(&Conformant::default());
        requires_layout_ext(&Layout::default());
        requires_multibuttonentry_ext(&Multibuttonentry::default());
        requires_combobox_ext(&Combobox::default());

        Gengrid::default().clear();
        Slideshow::default().clear();
        Slideshow::default().next();
        Slideshow::default().previous();
        assert_eq!(Slideshow::default().length(), 0);
        assert_eq!(Map::default().zoom(), 0);
        Map::default().set_paused(true);
        Video::default().pause();
        Video::default().stop();
        assert!(!Video::default().set_file("missing.mp4"));
        assert!(!Web::default().set_url("https://example.com"));
        Glview::default().changed();
        assert!(Glview::default().gl_api().is_none());
        assert_eq!(Glview::default().gl_size(), (0, 0));
        Glview::default().with_init(|| {});
        Glview::default().with_render(|_, _, _| {});
        Glview::default().with_key_down(|_| {});
        Glview::default().with_tick(0.1, |_| {});
        assert!(!Layout::default().set_file("missing.edj", ""));
        Multibuttonentry::default().clear();
        Combobox::default().hover_end();
        assert!(!Combobox::default().expanded());
        assert_eq!(Gengrid::default().value(), -1);
        assert_eq!(Multibuttonentry::default().value(), -1);
        assert_eq!(Combobox::default().value(), -1);
    }
}

#[cfg(test)]
mod trait_method_tests {
    use crate::error::CStringExt;
    use crate::prelude::*;

    #[test]
    fn test_signal_enum() {
        assert_eq!(Signal::Changed.as_ref(), "changed");
        assert_eq!(Signal::Clicked.as_ref(), "clicked");
        assert_eq!(Signal::Selected.as_ref(), "selected");
        assert_eq!(Signal::Unfocused.as_ref(), "unfocused");
        assert_eq!(Signal::default(), Signal::Changed);
        assert_eq!(Signal::Clicked, Signal::Clicked);
        let _ = Signal::Changed;
        let copied = Signal::Clicked;
        assert_eq!(copied, Signal::Clicked);
    }

    #[test]
    fn test_align_enum_conversions() {
        assert_eq!(f64::from(Align::Fill), -1.0);
        assert_eq!(f64::from(Align::Left), 0.0);
        assert_eq!(f64::from(Align::Center), 0.5);
        assert_eq!(f64::from(Align::Right), 1.0);
        assert_eq!(Align::default(), Align::Fill);
    }

    #[test]
    fn test_cursor_enum() {
        assert_eq!(Cursor::Hand1.as_ref(), "hand1");
        assert_eq!(Cursor::Hand2.as_ref(), "hand2");
        assert_eq!(Cursor::Hand3.as_ref(), "hand3");
        assert_eq!(Cursor::Bogocity.as_ref(), "bogocity");
        assert_eq!(Cursor::Xterm.as_ref(), "xterm");
        assert_eq!(Cursor::default(), Cursor::Hand2);
    }

    #[test]
    fn test_scroll_policy_enum() {
        assert_eq!(ScrollPolicy::default(), ScrollPolicy::Auto);
        assert_ne!(ScrollPolicy::Auto, ScrollPolicy::On);
        assert_ne!(ScrollPolicy::On, ScrollPolicy::Off);
    }

    #[test]
    fn test_bg_option_enum() {
        assert_eq!(BgOption::default(), BgOption::Scale);
        assert_ne!(BgOption::Center, BgOption::Stretch);
        assert_ne!(BgOption::Tile, BgOption::Scale);
    }

    #[test]
    fn test_weekday_action_bubble_flip_enums() {
        assert_eq!(Weekday::default(), Weekday::Sun);
        assert_eq!(ActionPos::default(), ActionPos::Center);
        assert_eq!(BubblePos::default(), BubblePos::TopLeft);
        assert_eq!(FlipMode::default(), FlipMode::RotateY);
    }

    #[test]
    fn test_cstring_extension_valid_strings() {
        let result = "hello".to_cstring();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string_lossy(), "hello");

        let result = String::from("world").to_cstring();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string_lossy(), "world");
    }

    #[test]
    fn test_cstring_extension_invalid_string() {
        let result = "hello\0world".to_cstring();
        assert!(result.is_err());
    }

    #[test]
    fn test_cstring_expect_valid() {
        let cstring = "valid_string".expect_cstring("test");
        assert_eq!(cstring.to_string_lossy(), "valid_string");
    }

    #[test]
    #[should_panic(expected = "Failed to create CString")]
    fn test_cstring_expect_invalid() {
        "invalid\0string".expect_cstring("test context");
    }

    #[test]
    fn test_panel_orient_enum() {
        let top = PanelOrient::Top as i32;
        let bottom = PanelOrient::Bottom as i32;
        let left = PanelOrient::Left as i32;
        let right = PanelOrient::Right as i32;

        assert_eq!(top, 0);
        assert!(bottom > top);
        assert!(left > bottom);
        assert!(right > left);
        assert_eq!(PanelOrient::default(), PanelOrient::Top);
    }
}

#[cfg(test)]
mod timer_tests {
    use crate::Timer;

    #[test]
    fn test_timer_creation() {
        let timer = Timer::default();
        assert!(timer.0.is_none());
        assert!(!timer.is_set());
    }

    #[test]
    fn test_timer_from_raw() {
        let timer = Timer::from(std::ptr::null_mut());
        assert!(timer.0.is_none());
        assert!(!timer.is_set());
    }

    #[test]
    fn test_timer_del_on_empty_is_noop() {
        Timer::default().del();
    }
}

#[cfg(test)]
mod widget_item_tests {
    use crate::WidgetItem;

    #[test]
    fn test_widget_item_default() {
        let item = WidgetItem::default();
        assert!(item.0.is_none());
        assert!(!item.is_set());
    }

    #[test]
    fn test_widget_item_from_raw_null() {
        let item = WidgetItem::from_raw(std::ptr::null_mut());
        assert!(item.0.is_none());
        assert!(!item.is_set());
    }

    #[test]
    fn test_widget_item_as_raw_panics_on_empty() {
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
        let original = Tm::default();
        let c_tm = original.to_tm();
        let converted = Tm::from_tm(c_tm);

        assert_eq!(original.sec, converted.sec);
        assert_eq!(original.min, converted.min);
        assert_eq!(original.hour, converted.hour);
        assert_eq!(original, converted);
    }
}

#[cfg(test)]
mod naviframe_tests {
    use crate::Naviframe;
    use crate::prelude::WidgetExt;

    #[test]
    fn test_naviframe_default() {
        let nav = Naviframe::default();
        assert!(nav.obj.is_none());
        assert_eq!(nav.lst.borrow().len(), 0);
        assert!(!nav.is_set());
    }

    #[test]
    fn test_naviframe_set_top_empty() {
        let nav = Naviframe::default();
        nav.set_top(0);
        assert_eq!(nav.lst.borrow().len(), 0);
    }

    #[test]
    fn test_naviframe_promote_empty() {
        let nav = Naviframe::default();
        nav.promote();
        assert!(!nav.is_set());
    }
}

#[cfg(test)]
mod selector_tests {
    use crate::prelude::*;
    use crate::{List, Menu, SegmentControl};

    #[test]
    fn test_selector_length() {
        let list = List::default();
        let menu = Menu::default();
        let segment = SegmentControl::default();

        assert_eq!(list.length(), 0);
        assert_eq!(menu.length(), 0);
        assert_eq!(segment.length(), 0);
    }

    #[test]
    fn test_selector_clear() {
        let list = List::default();
        let menu = Menu::default();
        let segment = SegmentControl::default();

        list.clear();
        menu.clear();
        segment.clear();
    }

    #[test]
    fn test_selector_value_empty() {
        assert_eq!(List::default().value(), -1);
        assert_eq!(Menu::default().value(), -1);
        assert_eq!(SegmentControl::default().value(), -1);
    }
}

#[cfg(test)]
mod list_tests {
    use crate::List;

    #[test]
    fn test_list_selected() {
        let list = List::default();
        let selected = list.selected();
        assert!(selected.0.is_none());
    }

    #[test]
    fn test_list_first() {
        let list = List::default();
        let first = list.first();
        assert!(first.0.is_none());
    }
}

#[cfg(test)]
mod menu_tests {
    use crate::Menu;

    #[test]
    fn test_menu_selected() {
        let menu = Menu::default();
        let selected = menu.selected();
        assert!(selected.0.is_none());
    }

    #[test]
    fn test_menu_first() {
        let menu = Menu::default();
        let first = menu.first();
        assert!(first.0.is_none());
    }
}

#[cfg(test)]
mod segment_control_tests {
    use crate::SegmentControl;

    #[test]
    fn test_segment_control_selected() {
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
        let panes = Panes::default();
        assert!(panes.content("left").is_none());
        assert!(panes.content("right").is_none());
    }
}

#[cfg(test)]
mod color_selector_tests {
    use crate::ColorSelector;
    use crate::prelude::*;

    #[test]
    fn test_color_selector_empty_value() {
        let color = ColorSelector::default();
        assert_eq!(color.value(), (0, 0, 0, 0));
        color.set_value((1, 2, 3, 4));
        assert_eq!(color.value(), (0, 0, 0, 0));
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
        } else {
            panic!("Expected NullByte error");
        }
    }

    #[test]
    fn test_nonnull_from_ptr_success() {
        use crate::error::nonnull_from_ptr;

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

    #[test]
    fn test_error_display_variants() {
        assert!(EflError::NullTimer.to_string().contains("timer"));
        assert!(EflError::NullWidgetItem.to_string().contains("widget item"));
        assert!(EflError::Other("boom".into()).to_string().contains("boom"));
        assert!(EflError::null_widget("ctx").to_string().contains("ctx"));
    }

    #[test]
    fn test_error_source_for_nul() {
        use std::error::Error;
        let err = "a\0b".to_cstring().unwrap_err();
        assert!(err.source().is_some());
        assert!(EflError::NullTimer.source().is_none());
    }
}

#[cfg(test)]
mod crate_exports {
    #[test]
    fn test_error_types_are_reexported() {
        let _: Option<crate::EflError> = None;
        let _: Option<crate::EflResult<()>> = None;
        fn needs_cstring_ext<T: crate::CStringExt + ?Sized>(_: &T) {}
        needs_cstring_ext("ok");
    }
}

#[cfg(test)]
mod ffi_hygiene {
    fn assert_not_send<T>() {
        trait AmbiguousIfSend<A> {
            fn check() {}
        }
        impl<T> AmbiguousIfSend<()> for T {}
        impl<T: Send> AmbiguousIfSend<u8> for T {}
        let _ = <T as AmbiguousIfSend<_>>::check;
    }

    fn assert_not_sync<T>() {
        trait AmbiguousIfSync<A> {
            fn check() {}
        }
        impl<T> AmbiguousIfSync<()> for T {}
        impl<T: Sync> AmbiguousIfSync<u8> for T {}
        let _ = <T as AmbiguousIfSync<_>>::check;
    }

    #[test]
    fn wrappers_are_neither_send_nor_sync() {
        assert_not_send::<crate::Button>();
        assert_not_sync::<crate::Button>();
        assert_not_send::<crate::Window>();
        assert_not_sync::<crate::Window>();
        assert_not_send::<crate::Timer>();
        assert_not_sync::<crate::Timer>();
        assert_not_send::<crate::WidgetItem>();
        assert_not_sync::<crate::WidgetItem>();
        assert_not_send::<crate::Popup>();
        assert_not_sync::<crate::Popup>();
        assert_not_send::<crate::Genlist>();
        assert_not_sync::<crate::Genlist>();
    }

    #[test]
    fn catch_unwind_does_not_escape() {
        let panicked = std::panic::catch_unwind(|| {
            panic!("user callback");
        });
        assert!(panicked.is_err());
    }
}
