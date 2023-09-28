extern crate libc;

#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Clone for Color {
    fn clone(&self) -> Color {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}

#[repr(C)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Clone for Vector2 {
    fn clone(&self) -> Vector2 {
        Vector2 {
            x: self.x,
            y: self.y,
        }
    }
}

pub enum Key {
    Enter = 257,
    Right = 262,
    Left = 263,
    Down = 264,
    Up = 265,
}

pub const MAROON: Color = Color { r: 190, g: 33, b: 55, a: 255 };
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

    fn IsKeyDown(key: i32) -> bool;
    fn IsKeyPressed(key: i32) -> bool;
    
    fn DrawCircleV(center: Vector2, radius: f32, color: Color);
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

pub fn clear_background(color: &Color) {
    let c_color = color.clone();

    unsafe {
        ClearBackground(c_color);
    }
}

pub fn draw_text(text: &str, x: i32, y: i32, font_size: i32, color: &Color) {
    let c_text = std::ffi::CString::new(text).unwrap();
    let c_color = color.clone();

    unsafe {
        DrawText(c_text.as_ptr() as *const libc::c_char, x, y, font_size, c_color);
    }
}

pub fn close_window() {
    unsafe {
        Raylib_CloseWindow();
    }
}

pub fn is_key_down(key: Key) -> bool {
    unsafe {
        IsKeyDown(key as i32)
    }
}

pub fn is_key_pressed(key: Key) -> bool {
    unsafe {
        IsKeyPressed(key as i32)
    }
}

pub fn draw_circle_v(center: &Vector2, radius: f32, color: &Color) {
    let c_center = center.clone();
    let c_color = color.clone();

    unsafe {
        DrawCircleV(c_center, radius, c_color);
    }
}
