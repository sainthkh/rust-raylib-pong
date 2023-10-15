use crate::raylib::{
    Scene, SceneResult, Rectangle,
    draw_text,
    gui_button,
    RAYWHITE,
};

pub struct Level1 {
    // player: Player,
    // enemy: Enemy,
    // ball: Ball,
    // bricks: Vec<Brick>,
}

impl Level1 {
    pub fn new() -> Self {
        Self {
            // player: Player::default(),
            // enemy: Enemy::default(),
            // ball: Ball::default(),
            // bricks: Vec::new(),
        }
    }
}

impl Scene for Level1 {
    fn frame(&mut self, delta_time: f32) -> SceneResult {
        draw_text("Hello, level 1!", 12, 12, 20, &RAYWHITE);

        gui_button(&Rectangle {
            x: 12.0,
            y: 60.0,
            width: 100.0,
            height: 40.0,
        }, "Go to level 2!");

        SceneResult::OnGoing
    }
}