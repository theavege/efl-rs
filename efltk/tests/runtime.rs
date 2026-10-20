use efltk::prelude::*;
use efltk::{Label, Timer, Window};

#[test]
fn fallible_text_and_window_apis_work_with_live_widgets() {
    assert!(matches!(
        Window::try_new("bad\0id", "title"),
        Err(EflError::NullByte(_))
    ));

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

        let timer = Timer::new(0.1, || {
            exit();
            false
        });
        (window, timer)
    });
}
