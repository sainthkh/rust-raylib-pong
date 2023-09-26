extern crate libc;

#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub const RAYWHITE: Color = Color { r: 245, g: 245, b: 245, a: 255 };

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

pub fn init_window(width: i32, height: i32, title: &str) {
    let c_title = std::ffi::CString::new(title).unwrap();
    unsafe {
        InitWindow(width, height, c_title.as_ptr() as *const libc::c_char);
    }
}

pub fn set_target_fps(fps: i32) {
    unsafe {
        SetTargetFPS(fps);
    }
}

pub fn window_should_close() -> bool {
    unsafe {
        WindowShouldClose()
    }
}

pub fn begin_drawing() {
    unsafe {
        BeginDrawing();
    }
}

pub fn end_drawing() {
    unsafe {
        EndDrawing();
    }
}

pub fn clear_background(color: Color) {
    unsafe {
        ClearBackground(color);
    }
}

pub fn draw_text(text: &str, x: i32, y: i32, font_size: i32, color: Color) {
    let c_text = std::ffi::CString::new(text).unwrap();
    unsafe {
        DrawText(c_text.as_ptr() as *const libc::c_char, x, y, font_size, color);
    }
}

pub fn close_window() {
    unsafe {
        Raylib_CloseWindow();
    }
}
