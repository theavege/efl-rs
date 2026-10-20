use efltk::prelude::*;
use efltk::{Button, Label, Timer, Window};
use std::{cell::Cell, rc::Rc};

struct DropFlag(Rc<Cell<bool>>);

impl Drop for DropFlag {
    fn drop(&mut self) {
        self.0.set(true);
    }
}

#[test]
fn fallible_text_and_window_apis_work_with_live_widgets() {
    assert!(matches!(
        Window::try_new("bad\0id", "title"),
        Err(EflError::NullByte(_))
    ));

    let timer_called = Rc::new(Cell::new(0));
    let timer_closure_dropped = Rc::new(Cell::new(false));
    run_with_guard(|| {
        let window = Window::try_new("runtime-test", "Runtime test").unwrap();
        let label = Label::new(&window).try_with_text("ready").unwrap();
        assert_eq!(label.text(), "ready");

        assert!(matches!(
            label.try_set_text("bad\0text"),
            Err(EflError::NullByte(_))
        ));
        assert_eq!(label.text(), "ready");
        assert!(matches!(
            Label::default().try_set_text("text"),
            Err(EflError::EmptyWidget(_))
        ));

        let button_called = Rc::new(Cell::new(false));
        let button_closure_dropped = Rc::new(Cell::new(false));
        let button_drop_flag = DropFlag(Rc::clone(&button_closure_dropped));
        let button = Button::new(&window).with_callback({
            let button_called = Rc::clone(&button_called);
            move |_| {
                let _ = &button_drop_flag;
                button_called.set(true);
            }
        });
        button.do_callback();
        assert!(button_called.get());
        button.del();
        assert!(button_closure_dropped.get());

        let timer_drop_flag = DropFlag(Rc::clone(&timer_closure_dropped));
        let timer_calls = Rc::clone(&timer_called);
        let timer = Timer::new(0.1, move || {
            let _ = &timer_drop_flag;
            timer_calls.set(timer_calls.get() + 1);
            exit();
            false
        });
        (window, timer)
    });
    assert_eq!(timer_called.get(), 1);
    assert!(timer_closure_dropped.get());
}
