use crate::raylib::*;

pub mod raylib;

const SCREEN_WIDTH: i32 = 450;
const SCREEN_HEIGHT: i32 = 800;
const BRICKS_PER_LINE: i32 = 6;
const BALL_SPEED: f32 = 10.0;

const ELEGANT_BLACK: Color = Color { r: 19, g: 19, b: 18, a: 255 };
const PADDLE_GRAY: Color = Color { r: 230, g: 230, b: 230, a: 255 };

struct Player {
    position: Vector2,
    size: Vector2,
    point: i32,
    speed: f32,

    color: Color,
}

impl Player {
    fn movement(&mut self) {
        let mut direction = Vector2 { x: 0.0, y: 0.0 };

        if is_key_down(Key::Left) {
            direction.x -= 1.0;

            if self.position.x - self.size.x / 2.0 <= 0.0 {
                self.position.x = self.size.x / 2.0;
            }
        }
        
        if is_key_down(Key::Right) {
            direction.x += 1.0;

            if self.position.x + self.size.x / 2.0 >= SCREEN_WIDTH as f32 {
                self.position.x = SCREEN_WIDTH as f32 - self.size.x / 2.0;
            }
        }

        direction.normalize();

        self.position += direction * self.speed;
    }

    fn movement_by_ai(&mut self, ball: &Ball) {
        if ball.active {
            if ball.position.x < self.position.x {
                self.position.x -= self.speed;
            }
            else if ball.position.x > self.position.x {
                self.position.x += self.speed;
            }
        }
    }

    fn collider(&self) -> Rectangle {
        Rectangle::from(&self.position, &self.size)
    }

    fn draw(&self) {
        draw_rectangle(
            (self.position.x - self.size.x / 2.0) as i32, 
            (self.position.y - self.size.y / 2.0) as i32, 
            self.size.x as i32, 
            self.size.y as i32, 
            &self.color);
    }
}

struct Ball {
    position: Vector2,
    direction: Vector2,
    speed: f32,
    radius: f32,
    active: bool,
}

impl Ball {
    fn movement(&mut self) {
        if self.active {
            self.position += self.direction.clone() * self.speed;
        }
    }

    fn collides(&self, rectangle: &Rectangle) -> bool {
        check_collision_circle_rec(
            &Circle {
                center: self.position.clone(),
                radius: self.radius,
            }, 
            rectangle
        )
    }

    fn init_position(&mut self, turn: &Turn) {
        self.position = match turn {
            Turn::Player => Vector2 { x: (SCREEN_WIDTH / 2) as f32, y: (SCREEN_HEIGHT * 7 / 8 - 30) as f32 },
            Turn::Enemy => Vector2 { x: (SCREEN_WIDTH / 2) as f32, y: (SCREEN_HEIGHT / 8 + 30) as f32 },
        }
    }

    fn init_direction(&mut self, turn: &Turn) {
        self.direction = match turn {
            Turn::Player => Vector2 { x: 0.0, y: -1.0 },
            Turn::Enemy => Vector2 { x: 0.0, y: 1.0 },
        }
    }

    fn draw(&self) {
        if self.active {
            draw_circle_v(&self.position, self.radius, &MAROON);
        }
    }
}

struct Brick {
    position: Vector2,
    active: bool,
    color: Color,
    size: Vector2,
}

impl Brick {
    fn collider(&self) -> Rectangle {
        Rectangle::from(&self.position, &self.size)
    }

    fn draw(&self) {
        if self.active {
            draw_rectangle(
                (self.position.x - self.size.x / 2.0) as i32, 
                (self.position.y - self.size.y / 2.0) as i32, 
                self.size.x as i32, 
                self.size.y as i32, 
                &self.color);
        }
    }
}

enum Turn {
    Player,
    Enemy,
}

struct Level {
    player: Player,
    enemy: Player,
    ball: Ball,
    bricks: Vec<Brick>,
    brick_size: Vector2,
    turn: Turn,
    pause: bool,
    game_over: bool,
}

impl Default for Level {
    fn default() -> Self {
        Level {
            player: Player {
                position: Vector2 { x: 0.0, y: 0.0 },
                size: Vector2 { x: 0.0, y: 0.0 },
                point: 0,
                speed: 5.0,
                color: PADDLE_GRAY,
            },
            enemy: Player {
                position: Vector2 { x: 0.0, y: 0.0 },
                size: Vector2 { x: 0.0, y: 0.0 },
                point: 0,
                speed: 5.0,
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

fn main() {
    init_window(SCREEN_WIDTH, SCREEN_HEIGHT, "Alcanoid");

    set_target_fps(60);

    let mut level: Level = Level::default();

    init_game(&mut level);

    while !window_should_close() {

        update_draw_frame(&mut level);
    }

    close_window();
}

fn init_game(level: &mut Level)
{
    level.player.position = Vector2 { x: (SCREEN_WIDTH / 2) as f32 , y: (SCREEN_HEIGHT * 7 / 8) as f32 };
    level.player.size = Vector2 { x: (SCREEN_WIDTH / 10) as f32, y: 20.0 };
    level.player.point = 0;
    level.player.color = PADDLE_GRAY;

    level.enemy.position = Vector2 { x: (SCREEN_WIDTH / 2) as f32, y: (SCREEN_HEIGHT / 8) as f32 };
    level.enemy.size = Vector2 { x: (SCREEN_WIDTH / 10) as f32, y: 20.0 };
    level.enemy.point = 0;
    level.enemy.color = PADDLE_GRAY;

    level.ball.position = Vector2 { x: (SCREEN_WIDTH / 2) as f32, y: (SCREEN_HEIGHT * 7 / 8 - 30) as f32 };
    level.ball.direction = Vector2 { x: 0.0, y: 0.0 };
    level.ball.speed = BALL_SPEED;
    level.ball.radius = 7.0;
    level.ball.active = false;

    let brick_width = (SCREEN_WIDTH / BRICKS_PER_LINE) as f32;

    level.brick_size = Vector2 { 
        x: brick_width, 
        y: 20.0, 
    };

    level.pause = false;
    level.game_over = false;

    // Top bricks
    for i in 0..BRICKS_PER_LINE {
        let y = level.brick_size.y / 2.0;
        let x = (i as f32) * level.brick_size.x + level.brick_size.x / 2.0;

        level.bricks.push(Brick {
            position: Vector2 { x, y },
            active: true,
            color: if i % 2 == 0 { GRAY } else { DARKGRAY },
            size: level.brick_size.clone(),
        });
    }

    // Bottom bricks
    for i in 0..BRICKS_PER_LINE {
        let y = SCREEN_HEIGHT as f32 - level.brick_size.y / 2.0;
        let x = (i as f32) * level.brick_size.x + level.brick_size.x / 2.0;

        level.bricks.push(Brick {
            position: Vector2 { x, y },
            active: true,
            color: if i % 2 == 0 { DARKGRAY } else { GRAY },
            size: level.brick_size.clone(),
        });
    }
}

fn update_game(level: &mut Level)
{
    if !level.game_over {
        if is_key_pressed(Key::P) {
            level.pause = !level.pause;
        }

        if level.pause {
            return;
        }

        // Player movement
        level.player.movement();

        if !level.ball.active {
            if is_key_pressed(Key::Space) {
                level.ball.active = true;
                level.ball.init_direction(&level.turn);
                level.ball.init_position(&level.turn);
                level.ball.speed = BALL_SPEED;
            } else {
                return;
            }
        }

        // Ball movement
        level.ball.movement();
        level.enemy.movement_by_ai(&level.ball);

        // Collision logic: ball vs walls
        if level.ball.position.x + level.ball.radius >= SCREEN_WIDTH as f32 || 
            level.ball.position.x - level.ball.radius <= 0.0 {
            level.ball.direction.x *= -1.0;
        }
        if level.ball.position.y - level.ball.radius <= 0.0 {
            level.ball.active = false;
            level.player.point += 1;
            level.turn = Turn::Player;
        }
        if level.ball.position.y + level.ball.radius >= SCREEN_HEIGHT as f32 {
            level.ball.active = false;
            level.enemy.point += 1;
            level.turn = Turn::Enemy;
        }

        // Collision logic: ball vs player
        if level.ball.collides(&level.player.collider()) {
            level.ball.direction.y *= -1.0;
            level.ball.direction.x = (level.ball.position.x - level.player.position.x) / (level.player.size.x / 2.0);

            level.ball.direction.normalize();
        }
        
        // Collision logic: ball vs. enemy
        if level.ball.collides(&level.enemy.collider()) {
            level.ball.direction.y *= -1.0;
            level.ball.direction.x = (level.ball.position.x - level.enemy.position.x) / (level.enemy.size.x / 2.0);
            level.ball.direction.x /= 3.0;

            level.ball.direction.normalize();
        }

        // Collision logic: ball vs bricks
        for brick in &mut level.bricks {
            if brick.active && level.ball.collides(&brick.collider()) {
                brick.active = false;
                level.ball.direction.y *= -1.0;

                break;
            }
        }
    }
    else {
        if is_key_pressed(Key::Enter) {
            init_game(level);
            level.game_over = false;
        }
    }
}

fn draw_game(level: &mut Level) {
    begin_drawing();

    clear_background(&ELEGANT_BLACK);

    if !level.game_over {
        // Draw player points
        let player_points = format!("{}", level.player.point);
        let enemy_points = format!("{}", level.enemy.point);

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
        level.player.draw();
        level.enemy.draw();

        level.ball.draw();

        for brick in &level.bricks {
            brick.draw();
        }

        let pause_text = "GAME PAUSED";
        let font_size = 40;

        if level.pause {
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

fn update_draw_frame(level: &mut Level) {
    update_game(level);
    draw_game(level);
}
