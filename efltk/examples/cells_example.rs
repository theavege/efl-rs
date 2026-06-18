#![forbid(unsafe_code)]

use efltk::prelude::*;

mod components {
    pub mod cells;
}

fn main() {
    components::cells::Cells::run("Cells", 600, 400);
}
