use crate::raylib::*;

pub mod raylib;

// @see https://github.com/raysan5/raylib-games/blob/master/classics/src/arkanoid.c
// @see https://github.com/deltaphc/raylib-rs/blob/master/samples/arkanoid.rs

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;
const PLAYER_MAX_LIFE: i32 = 5;
const LINES_OF_BRICKS: i32 = 5;
const BRICKS_PER_LINE: i32 = 20;
const INITIAL_DOWN_POSITION: f32 = 50.0;

struct Player {
    position: Vector2,
    size: Vector2,
    life: i32,
}

struct Ball {
    position: Vector2,
    direction: Vector2,
    speed: f32,
    radius: f32,
    active: bool,
}

struct Brick {
    position: Vector2,
    active: bool,
}

struct Game {
    player: Player,
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
                life: 0,
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
    game.player.life = PLAYER_MAX_LIFE;

    game.ball.position = Vector2 { x: (SCREEN_WIDTH / 2) as f32, y: (SCREEN_HEIGHT * 7 / 8 - 30) as f32 };
    game.ball.direction = Vector2 { x: 0.0, y: 0.0 };
    game.ball.speed = 5.0;
    game.ball.radius = 7.0;
    game.ball.active = false;

    game.brick_size = Vector2 { x: (SCREEN_WIDTH / BRICKS_PER_LINE) as f32, y: 40.0 };

    game.pause = false;
    game.game_over = false;

    for i in 0..LINES_OF_BRICKS {
        let y = (i as f32) * game.brick_size.y + INITIAL_DOWN_POSITION;

        for j in 0..BRICKS_PER_LINE {
            let x = (j as f32) * game.brick_size.x + game.brick_size.x / 2.0;

            let brick = Brick {
                position: Vector2 { x, y },
                active: true,
            };

            game.bricks.push(brick);
        }
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
        if is_key_down(Key::Left) {
            game.player.position.x -= 5.0;
        }
        if game.player.position.x - game.player.size.x / 2.0 <= 0.0 {
            game.player.position.x = game.player.size.x / 2.0;
        }
        if is_key_down(Key::Right) {
            game.player.position.x += 5.0;
        }
        if game.player.position.x + game.player.size.x / 2.0 >= SCREEN_WIDTH as f32 {
            game.player.position.x = SCREEN_WIDTH as f32 - game.player.size.x / 2.0;
        }

        // Ball launching logic
        if !game.ball.active {
            if is_key_pressed(Key::Space) {
                game.ball.active = true;
                game.ball.direction = Vector2 { x: 0.0, y: -1.0 };
                game.ball.speed = 5.0;
            }
        }

        // Ball movement
        if game.ball.active {
            game.ball.position.x += game.ball.direction.x * game.ball.speed;
            game.ball.position.y += game.ball.direction.y * game.ball.speed;
        }
        else {
            game.ball.position = Vector2 { 
                x: game.player.position.x, 
                y: (SCREEN_HEIGHT as f32) * 7.0 / 8.0 - 30.0,
            };
        }

        // Collision logic: ball vs walls
        if game.ball.position.x + game.ball.radius >= SCREEN_WIDTH as f32 || 
            game.ball.position.x - game.ball.radius <= 0.0 {
            game.ball.direction.x *= -1.0;
        }
        if game.ball.position.y - game.ball.radius <= 0.0 {
            game.ball.direction.y *= -1.0;
        }
        if game.ball.position.y + game.ball.radius >= SCREEN_HEIGHT as f32 {
            game.ball.active = false;
            game.ball.direction = Vector2 { x: 0.0, y: 0.0 };
            game.player.life -= 1;
        }

        // Collision logic: ball vs player
        if check_collision_circle_rec(
            &Circle { 
                center: game.ball.position.clone(), 
                radius: game.ball.radius, 
            },
            &Rectangle {
                x: game.player.position.x - game.player.size.x / 2.0,
                y: game.player.position.y - game.player.size.y / 2.0,
                width: game.player.size.x,
                height: game.player.size.y,
            }
        ) {
            game.ball.direction.y *= -1.0;
            game.ball.direction.x = (game.ball.position.x - game.player.position.x) / (game.player.size.x / 2.0);

            game.ball.direction.normalize();
        }

        // Collision logic: ball vs bricks
        for brick in game.bricks.iter_mut() {
            if brick.active {
                if check_collision_circle_rec(
                    &Circle { 
                        center: game.ball.position.clone(), 
                        radius: game.ball.radius, 
                    },
                    &Rectangle {
                        x: brick.position.x - game.brick_size.x / 2.0,
                        y: brick.position.y - game.brick_size.y / 2.0,
                        width: game.brick_size.x,
                        height: game.brick_size.y,
                    }
                ) {
                    brick.active = false;

                    // Hit Left
                    if game.ball.position.x > brick.position.x + game.brick_size.x / 2.0 {
                        game.ball.direction.x *= -1.0;
                    }
                    // Hit Right
                    else if game.ball.position.x < brick.position.x - game.brick_size.x / 2.0 {
                        game.ball.direction.x *= -1.0;
                    }
                    // Hit Top
                    else if game.ball.position.y > brick.position.y + game.brick_size.y / 2.0 {
                        game.ball.direction.y *= -1.0;
                    }
                    // Hit Bottom
                    else if game.ball.position.y < brick.position.y - game.brick_size.y / 2.0 {
                        game.ball.direction.y *= -1.0;
                    }

                    break;
                }
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
        draw_rectangle(
            (game.player.position.x - game.player.size.x / 2.0) as i32, 
            (game.player.position.y - game.player.size.y / 2.0) as i32, 
            game.player.size.x as i32, 
            game.player.size.y as i32, 
            &BLACK);
        
        // Draw player lives
        for i in 0..game.player.life {
            draw_rectangle(20 + 40 * i, SCREEN_HEIGHT - 30, 35, 10, &LIGHTGRAY);
        }

        // Draw ball
        if game.ball.active {
            draw_circle_v(&game.ball.position, game.ball.radius, &MAROON);
        }

        // Draw bricks
        for (i, brick) in game.bricks.iter().enumerate() {
            if brick.active {
                let color = {
                    let line = (i as i32) / BRICKS_PER_LINE;
                    
                    let (first_color, second_color) = if line % 2 == 0 {
                        (GRAY, DARKGRAY)
                    }
                    else {
                        (DARKGRAY, GRAY)
                    };

                    if i % 2 == 0 {
                        first_color
                    }
                    else {
                        second_color
                    }
                };

                draw_rectangle(
                    (brick.position.x - game.brick_size.x / 2.0) as i32, 
                    (brick.position.y - game.brick_size.y / 2.0) as i32, 
                    game.brick_size.x as i32, 
                    game.brick_size.y as i32, 
                    &color);
            }
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
