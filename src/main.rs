use crate::raylib::*;
use crate::consts::{
    SCREEN_HEIGHT, SCREEN_WIDTH, 
};
use crate::levels::level0::Level0;

pub mod raylib;
pub mod consts;
pub mod objects;
pub mod levels {
    pub mod level0;
}

fn main() {
    init_window(SCREEN_WIDTH, SCREEN_HEIGHT, "Alcanoid");

    set_target_fps(60);

    let mut level: Level0 = Level0::default();

    level.init();

    while !window_should_close() {
        level.frame();
    }

    close_window();
}
