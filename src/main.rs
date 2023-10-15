use crate::raylib::{
    init_window, set_target_fps, window_should_close, close_window,
    begin_drawing, end_drawing, clear_background, draw_text,
    Vector2, Color,
    Scene, SceneResult, SceneManager,
    RAYWHITE, BLACK,
};

use crate::consts::{
    SCREEN_WIDTH, SCREEN_HEIGHT,
};

use crate::levels::level0::Level0;
use crate::levels::level1::Level1;

pub mod raylib;
pub mod consts;
pub mod objects;
pub mod levels {
    pub mod level0;
    pub mod level1;
}

#[derive(Default)]
struct Player {
    position: Vector2,
    size: Vector2,
    point: i32,
    speed: f32,

    color: Color,
}

#[derive(Default)]
struct Enemy {
    position: Vector2,
    size: Vector2,
    point: i32,
    speed: f32,

    color: Color,
}

#[derive(Default)]
struct Ball {
    position: Vector2,
    direction: Vector2,
    speed: f32,
    radius: f32,
    active: bool,
}

#[derive(Default)]
struct Brick {
    position: Vector2,
    size: Vector2,
    color: Color,
    active: bool,
}

#[derive(Default)]
struct Accelerator {
    position: Vector2,
    size: Vector2,
    color: Color,
    active: bool,
}

#[derive(Default)]
struct Barrier {
    position: Vector2,
    size: Vector2,
    color: Color,
    active: bool,
}

#[derive(Default)]
struct Level2 {
    player: Player,
    enemy: Enemy,
    ball: Ball,
    bricks: Vec<Brick>,
    barrier: Barrier,
    life: i32,
}

impl Scene for Level2 {
    fn frame(&mut self, delta_time: f32) -> SceneResult {
        draw_text("Hello, level 2!", 12, 12, 20, &BLACK);

        SceneResult::OnGoing
    }
}

fn main() {
    init_window(SCREEN_WIDTH, SCREEN_HEIGHT, "Yet Another Pong");

    set_target_fps(60);

    let mut scene_manager = SceneManager::new();
    scene_manager.add(Box::new(Level0::new()));
    scene_manager.add(Box::new(Level1::new()));
    scene_manager.add(Box::new(Level2::default()));

    scene_manager.set(1);

    scene_manager.run();

    close_window();
}