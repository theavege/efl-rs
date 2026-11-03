use efltk::prelude::{Timer, exit};

pub fn smoke_timer() -> Option<Timer> {
    std::env::var_os("EFLTK_SMOKE_TEST").map(|_| {
        Timer::new(0.25, || {
            exit();
            false
        })
    })
}
