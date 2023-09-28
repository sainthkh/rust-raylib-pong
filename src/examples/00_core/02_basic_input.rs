use crate::raylib::*;

pub mod raylib;

fn main() {
    let screen_width = 800;
    let screen_height = 600;

    init_window(screen_width, screen_height, "Hello, World!");

    let mut ball_position = Vector2 { x: screen_width as f32 / 2.0, y: screen_height as f32 / 2.0 };

    set_target_fps(60);

    while !window_should_close() {

        if is_key_down(Key::Right) {
            ball_position.x += 2.0;
        }
        if is_key_down(Key::Left) {
            ball_position.x -= 2.0;
        }
        if is_key_down(Key::Down) {
            ball_position.y += 2.0;
        }
        if is_key_down(Key::Up) {
            ball_position.y -= 2.0;
        }

        begin_drawing();

        clear_background(&RAYWHITE);

        draw_text("move the ball with arrow keys!", 10, 10, 20, &Color { r: 0, g: 0, b: 0, a: 255 });

        draw_circle_v(&ball_position, 50.0, &MAROON);

        end_drawing();
    }

    close_window();
}
