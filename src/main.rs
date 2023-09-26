extern crate libc;

#[repr(C)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

const RAYWHITE: Color = Color { r: 245, g: 245, b: 245, a: 255 };

#[link(name = "raylib", kind = "static")]
extern "C" {
    fn InitWindow(width: i32, height: i32, title: *const libc::c_char);
    fn SetTargetFPS(fps: i32);
    fn WindowShouldClose() -> bool;
    fn BeginDrawing();
    fn EndDrawing();
    fn ClearBackground(color: Color);
    fn DrawText(text: *const libc::c_char, x: i32, y: i32, font_size: i32, color: Color);
    fn Raylib_CloseWindow();
}

fn init_window(width: i32, height: i32, title: &str) {
    let c_title = std::ffi::CString::new(title).unwrap();
    unsafe {
        InitWindow(width, height, c_title.as_ptr() as *const libc::c_char);
    }
}

fn set_target_fps(fps: i32) {
    unsafe {
        SetTargetFPS(fps);
    }
}

fn window_should_close() -> bool {
    unsafe {
        WindowShouldClose()
    }
}

fn begin_drawing() {
    unsafe {
        BeginDrawing();
    }
}

fn end_drawing() {
    unsafe {
        EndDrawing();
    }
}

fn clear_background(color: Color) {
    unsafe {
        ClearBackground(color);
    }
}

fn draw_text(text: &str, x: i32, y: i32, font_size: i32, color: Color) {
    let c_text = std::ffi::CString::new(text).unwrap();
    unsafe {
        DrawText(c_text.as_ptr() as *const libc::c_char, x, y, font_size, color);
    }
}

fn close_window() {
    unsafe {
        Raylib_CloseWindow();
    }
}

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
