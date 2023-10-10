extern crate libc;
use std::ffi::CString;
use std::ops;
use std::time::Instant;

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

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign<Vector2> for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Vector2 {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Vector2 {
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&mut self) {
        let length = self.length();

        if length < 0.00001 {
            return;
        }

        self.x /= length;
        self.y /= length;
    }

    pub fn angle(a: &Vector2, b: &Vector2) -> f32 {
        let dot = a.x * b.x + a.y * b.y;
        
        dot.acos()
    }
}

#[repr(C)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Clone for Rectangle {
    fn clone(&self) -> Rectangle {
        Rectangle {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }
}

impl Rectangle {
    pub fn from(position: &Vector2, size: &Vector2) -> Rectangle {
        Rectangle {
            x: position.x - size.x / 2.0,
            y: position.y - size.y / 2.0,
            width: size.x,
            height: size.y,
        }
    }
}

pub struct Circle {
    pub center: Vector2,
    pub radius: f32,
}

pub struct Time {
    start: Instant,
    old_elapsed: f32,
}

impl Default for Time {
    fn default() -> Time {
        Time {
            start: Instant::now(),
            old_elapsed: 0.0,
        }
    }
}

impl Time {
    pub fn delta_time(&mut self) -> f32 {
        let duration = self.start.elapsed();
        let current = duration.as_secs_f32();
        let delta = current - self.old_elapsed;

        self.old_elapsed = current;

        delta
    }
}

pub enum Key {
    P = 80,
    Space = 32,
    Enter = 257,
    Right = 262,
    Left = 263,
    Down = 264,
    Up = 265,
}

pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
pub const DARKGRAY: Color = Color { r: 80, g: 80, b: 80, a: 255 };
pub const LIGHTGRAY: Color = Color { r: 200, g: 200, b: 200, a: 255 };
pub const GRAY: Color = Color { r: 130, g: 130, b: 130, a: 255 };
pub const MAROON: Color = Color { r: 190, g: 33, b: 55, a: 255 };
pub const RAYWHITE: Color = Color { r: 245, g: 245, b: 245, a: 255 };

pub trait Scene {
    fn init(&mut self);
    fn frame(&mut self, delta_time: f32) {
        self.update(delta_time);
        self.draw();
    }
    fn update(&mut self, delta_time: f32);
    fn draw(&self);
    fn close(&mut self) {
    }
}

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

    fn MeasureText(text: *const libc::c_char, font_size: i32) -> i32;

    fn IsKeyDown(key: i32) -> bool;
    fn IsKeyPressed(key: i32) -> bool;

    fn DrawCircleV(center: Vector2, radius: f32, color: Color);
    fn DrawRectangle(x: i32, y: i32, width: i32, height: i32, color: Color);

    fn CheckCollisionCircleRec(center: Vector2, radius: f32, rec: Rectangle) -> bool;
}

pub fn init_window(width: i32, height: i32, title: &str) {
    let c_title = CString::new(title).unwrap();

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
    let c_text = CString::new(text).unwrap();
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

pub fn measure_text(text: &str, font_size: i32) -> i32 {
    let c_text = CString::new(text).unwrap();

    unsafe {
        MeasureText(c_text.as_ptr() as *const libc::c_char, font_size)
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

pub fn draw_rectangle(x: i32, y: i32, width: i32, height: i32, color: &Color) {
    let c_color = color.clone();

    unsafe {
        DrawRectangle(x, y, width, height, c_color);
    }
}

pub fn check_collision_circle_rec(circle: &Circle, rec: &Rectangle) -> bool {
    let c_center = circle.center.clone();
    let c_rec = rec.clone();

    unsafe {
        CheckCollisionCircleRec(c_center, circle.radius, c_rec)
    }
}
