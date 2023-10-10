use crate::raylib::{
    Vector2, Scene,
    GRAY, DARKGRAY, MAROON,
    is_key_pressed, Key,
    begin_drawing, end_drawing,
    clear_background, draw_text,
    measure_text,
};
use crate::consts::{
    SCREEN_HEIGHT, SCREEN_WIDTH, 
    BRICKS_PER_LINE, BALL_SPEED,
    PADDLE_GRAY, ELEGANT_BLACK,
};
use crate::objects::{
    Player, Ball, Brick, Turn,
};

pub struct Level0 {
    player: Player,
    enemy: Player,
    ball: Ball,
    bricks: Vec<Brick>,
    brick_size: Vector2,
    turn: Turn,
    pause: bool,
    game_over: bool,
}

impl Default for Level0 {
    fn default() -> Self {
        Level0 {
            player: Player {
                position: Vector2 { x: 0.0, y: 0.0 },
                size: Vector2 { x: 0.0, y: 0.0 },
                point: 0,
                speed: 300.0,
                color: PADDLE_GRAY,
            },
            enemy: Player {
                position: Vector2 { x: 0.0, y: 0.0 },
                size: Vector2 { x: 0.0, y: 0.0 },
                point: 0,
                speed: 300.0,
                color: PADDLE_GRAY,
            },
            ball: Ball {
                position: Vector2 { x: 0.0, y: 0.0 },
                direction: Vector2 { x: 0.0, y: 0.0 },
                speed: 0.0,
                radius: 0.0,
                active: false,
            },
            bricks: Vec::new(),
            brick_size: Vector2 { x: 0.0, y: 0.0 },
            turn: Turn::Player,
            pause: false,
            game_over: false,
        }
    }
}

impl Scene for Level0 {
    fn init(&mut self) {
        self.player.position = Vector2 { x: (SCREEN_WIDTH / 2) as f32 , y: (SCREEN_HEIGHT * 7 / 8) as f32 };
        self.player.size = Vector2 { x: (SCREEN_WIDTH / 10) as f32, y: 20.0 };
        self.player.point = 0;
        self.player.color = PADDLE_GRAY;

        self.enemy.position = Vector2 { x: (SCREEN_WIDTH / 2) as f32, y: (SCREEN_HEIGHT / 8) as f32 };
        self.enemy.size = Vector2 { x: (SCREEN_WIDTH / 10) as f32, y: 20.0 };
        self.enemy.point = 0;
        self.enemy.color = PADDLE_GRAY;

        self.ball.position = Vector2 { x: (SCREEN_WIDTH / 2) as f32, y: (SCREEN_HEIGHT * 7 / 8 - 30) as f32 };
        self.ball.direction = Vector2 { x: 0.0, y: 0.0 };
        self.ball.speed = BALL_SPEED;
        self.ball.radius = 7.0;
        self.ball.active = false;

        let brick_width = (SCREEN_WIDTH / BRICKS_PER_LINE) as f32;

        self.brick_size = Vector2 { 
            x: brick_width, 
            y: 20.0, 
        };

        self.pause = false;
        self.game_over = false;

        // Top bricks
        for i in 0..BRICKS_PER_LINE {
            let y = self.brick_size.y / 2.0;
            let x = (i as f32) * self.brick_size.x + self.brick_size.x / 2.0;

            self.bricks.push(Brick {
                position: Vector2 { x, y },
                active: true,
                color: if i % 2 == 0 { GRAY } else { DARKGRAY },
                size: self.brick_size.clone(),
            });
        }

        // Bottom bricks
        for i in 0..BRICKS_PER_LINE {
            let y = SCREEN_HEIGHT as f32 - self.brick_size.y / 2.0;
            let x = (i as f32) * self.brick_size.x + self.brick_size.x / 2.0;

            self.bricks.push(Brick {
                position: Vector2 { x, y },
                active: true,
                color: if i % 2 == 0 { DARKGRAY } else { GRAY },
                size: self.brick_size.clone(),
            });
        }
    }

    fn update(&mut self, delta_time: f32) {
        if !self.game_over {
            if is_key_pressed(Key::P) {
                self.pause = !self.pause;
            }

            if self.pause {
                return;
            }

            // Player movement
            self.player.movement(delta_time);

            if !self.ball.active {
                if is_key_pressed(Key::Space) {
                    self.ball.active = true;
                    self.ball.init_direction(&self.turn);
                    self.ball.init_position(&self.turn);
                    self.ball.speed = BALL_SPEED;
                } else {
                    return;
                }
            }

            // Ball movement
            self.ball.movement(delta_time);
            self.enemy.movement_by_ai(&self.ball, delta_time);

            // Collision logic: ball vs walls
            if self.ball.position.x + self.ball.radius >= SCREEN_WIDTH as f32 || 
                self.ball.position.x - self.ball.radius <= 0.0 {
                self.ball.direction.x *= -1.0;
            }
            if self.ball.position.y - self.ball.radius <= 0.0 {
                self.ball.active = false;
                self.player.point += 1;
                self.turn = Turn::Player;
            }
            if self.ball.position.y + self.ball.radius >= SCREEN_HEIGHT as f32 {
                self.ball.active = false;
                self.enemy.point += 1;
                self.turn = Turn::Enemy;
            }

            // Collision logic: ball vs player
            if self.ball.collides(&self.player.collider()) {
                self.ball.direction.y *= -1.0;
                self.ball.direction.x = (self.ball.position.x - self.player.position.x) / (self.player.size.x / 2.0);

                self.ball.direction.normalize();
            }
            
            // Collision logic: ball vs. enemy
            if self.ball.collides(&self.enemy.collider()) {
                self.ball.direction.y *= -1.0;
                self.ball.direction.x = (self.ball.position.x - self.enemy.position.x) / (self.enemy.size.x / 2.0);
                self.ball.direction.x /= 3.0;

                self.ball.direction.normalize();
            }

            // Collision logic: ball vs bricks
            for brick in &mut self.bricks {
                if brick.active && self.ball.collides(&brick.collider()) {
                    brick.active = false;
                    self.ball.direction.y *= -1.0;

                    break;
                }
            }
        }
        else {
            if is_key_pressed(Key::Enter) {
                self.init();
                self.game_over = false;
            }
        }
    }

    fn draw(&self) {
        begin_drawing();

        clear_background(&ELEGANT_BLACK);

        if !self.game_over {
            // Draw player points
            let player_points = format!("{}", self.player.point);
            let enemy_points = format!("{}", self.enemy.point);

            draw_text(
                &player_points, 
                20, 
                400,
                40, 
                &GRAY
            );

            draw_text(
                &enemy_points, 
                400, 
                360,
                40, 
                &GRAY
            );

            // Draw player bar
            self.player.draw();
            self.enemy.draw();

            self.ball.draw();

            for brick in &self.bricks {
                brick.draw();
            }

            let pause_text = "GAME PAUSED";
            let font_size = 40;

            if self.pause {
                draw_text(
                    &pause_text, 
                    SCREEN_WIDTH / 2 - measure_text(&pause_text, font_size) / 2, 
                    SCREEN_HEIGHT / 2 - 40, 
                    font_size, 
                    &GRAY
                );
            }
        } 
        else {
            draw_text("Press [ENTER] to Play", 10, 10, 20, &MAROON);
        }

        end_drawing();
    }
}
