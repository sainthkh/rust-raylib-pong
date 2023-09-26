use crate::raylib::*;

pub mod raylib;

fn main() {
    init_window(800, 600, "Hello, World!");

    set_target_fps(60);

    while !window_should_close() {
        begin_drawing();

        clear_background(RAYWHITE);

        draw_text("Hello, World!", 12, 12, 20, Color { r: 0, g: 0, b: 0, a: 255 });

        end_drawing();
    }

    close_window();
}
