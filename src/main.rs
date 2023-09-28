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
    speed: Vector2,
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
                speed: Vector2 { x: 0.0, y: 0.0 },
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
    game.ball.speed = Vector2 { x: 0.0, y: 0.0 };
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
        draw_text("On Game", 10, 10, 20, &MAROON);
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
