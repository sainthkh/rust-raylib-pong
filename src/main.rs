use crate::raylib::*;

pub mod raylib;

const SCREEN_WIDTH: i32 = 450;
const SCREEN_HEIGHT: i32 = 800;
const BRICKS_PER_LINE: i32 = 18;

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
        }
        if is_key_down(Key::Right) {
            direction.x += 1.0;
        }
        if is_key_down(Key::Up) {
            direction.y -= 1.0;
        }
        if is_key_down(Key::Down) {
            direction.y += 1.0;
        }

        direction.normalize();

        self.position += direction * self.speed;
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

struct Game {
    player: Player,
    enemy: Player,
    ball: Ball,
    bricks: Vec<Brick>,
    brick_size: Vector2,
    pause: bool,
    game_over: bool,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            player: Player {
                position: Vector2 { x: 0.0, y: 0.0 },
                size: Vector2 { x: 0.0, y: 0.0 },
                point: 0,
                speed: 5.0,
                color: BLACK,
            },
            enemy: Player {
                position: Vector2 { x: 0.0, y: 0.0 },
                size: Vector2 { x: 0.0, y: 0.0 },
                point: 0,
                speed: 5.0,
                color: BLACK,
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
            pause: false,
            game_over: false,
        }
    }
}

fn main() {
    init_window(SCREEN_WIDTH, SCREEN_HEIGHT, "Alcanoid");

    set_target_fps(60);

    let mut game: Game = Game::default();

    init_game(&mut game);

    while !window_should_close() {

        update_draw_frame(&mut game);
    }

    close_window();
}

fn init_game(game: &mut Game)
{
    game.player.position = Vector2 { x: (SCREEN_WIDTH / 2) as f32 , y: (SCREEN_HEIGHT * 7 / 8) as f32 };
    game.player.size = Vector2 { x: (SCREEN_WIDTH / 10) as f32, y: 20.0 };
    game.player.point = 0;
    game.player.color = BLACK;

    game.enemy.position = Vector2 { x: (SCREEN_WIDTH / 2) as f32, y: (SCREEN_HEIGHT / 8) as f32 };
    game.enemy.size = Vector2 { x: (SCREEN_WIDTH / 10) as f32, y: 20.0 };
    game.enemy.point = 0;
    game.enemy.color = BLACK;

    game.ball.position = Vector2 { x: (SCREEN_WIDTH / 2) as f32, y: (SCREEN_HEIGHT * 7 / 8 - 30) as f32 };
    game.ball.direction = Vector2 { x: 0.0, y: 0.0 };
    game.ball.speed = 5.0;
    game.ball.radius = 7.0;
    game.ball.active = false;

    let brick_width = (SCREEN_WIDTH / BRICKS_PER_LINE) as f32;

    game.brick_size = Vector2 { 
        x: brick_width, 
        y: brick_width, 
    };

    game.pause = false;
    game.game_over = false;

    // Top bricks
    for i in 0..BRICKS_PER_LINE {
        let y = game.brick_size.y / 2.0;
        let x = (i as f32) * game.brick_size.x + game.brick_size.x / 2.0;

        game.bricks.push(Brick {
            position: Vector2 { x, y },
            active: true,
            color: if i % 2 == 0 { GRAY } else { DARKGRAY },
            size: game.brick_size.clone(),
        });
    }

    // Bottom bricks
    for i in 0..BRICKS_PER_LINE {
        let y = SCREEN_HEIGHT as f32 - game.brick_size.y / 2.0;
        let x = (i as f32) * game.brick_size.x + game.brick_size.x / 2.0;

        game.bricks.push(Brick {
            position: Vector2 { x, y },
            active: true,
            color: if i % 2 == 0 { DARKGRAY } else { GRAY },
            size: game.brick_size.clone(),
        });
    }
}

fn update_game(game: &mut Game)
{
    if !game.game_over {
        if is_key_pressed(Key::P) {
            game.pause = !game.pause;
        }

        if game.pause {
            return;
        }

        // Player movement
        game.player.movement();

        if !game.ball.active {
            if is_key_pressed(Key::Space) {
                game.ball.active = true;
                game.ball.direction = Vector2 { x: 0.0, y: -1.0 };
                game.ball.speed = 5.0;
            }
        }

        // Ball movement
        game.ball.movement();

        // Collision logic: ball vs walls
        if game.ball.position.x + game.ball.radius >= SCREEN_WIDTH as f32 || 
            game.ball.position.x - game.ball.radius <= 0.0 {
            game.ball.direction.x *= -1.0;
        }
        // if game.ball.position.y - game.ball.radius <= 0.0 {
        //     game.ball.direction.y *= -1.0;
        // }
        // if game.ball.position.y + game.ball.radius >= SCREEN_HEIGHT as f32 {
        //     game.ball.active = false;
        //     game.ball.direction = Vector2 { x: 0.0, y: 0.0 };
        //     game.player.life -= 1;
        // }

        // Collision logic: ball vs player
        if game.ball.collides(&game.player.collider()) {
            game.ball.direction.y *= -1.0;
            game.ball.direction.x = (game.ball.position.x - game.player.position.x) / (game.player.size.x / 2.0);

            game.ball.direction.normalize();
        }

        // Collision logic: ball vs bricks
        for brick in &mut game.bricks {
            if brick.active && game.ball.collides(&brick.collider()) {
                brick.active = false;
                game.ball.direction.y *= -1.0;

                break;
            }
        }
    }
    else {
        if is_key_pressed(Key::Enter) {
            init_game(game);
            game.game_over = false;
        }
    }
}

fn draw_game(game: &mut Game) {
    begin_drawing();

    clear_background(&RAYWHITE);

    if !game.game_over {
        // Draw player bar
        game.player.draw();
        game.enemy.draw();

        game.ball.draw();

        for brick in &game.bricks {
            brick.draw();
        }

        let pause_text = "GAME PAUSED";
        let font_size = 40;

        if game.pause {
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

fn update_draw_frame(game: &mut Game) {
    update_game(game);
    draw_game(game);
}
