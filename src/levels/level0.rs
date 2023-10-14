use crate::raylib::{
    Vector2, Color, Scene, SceneResult, Rectangle,
    GRAY, DARKGRAY, MAROON,
    is_key_pressed, Key,
    clear_background, draw_text,
    measure_text,
};
use crate::consts::{
    SCREEN_HEIGHT, SCREEN_WIDTH,
    BRICK_HEIGHT,
    BRICKS_PER_LINE,
    ELEGANT_BLACK,
};
use crate::objects::{
    Player, Enemy, Ball, Brick, Turn, 
    draw_player, draw_ball, draw_enemy, draw_brick,
    move_player, move_enemy, move_ball,
    init_ball, collide_ball,
};

pub struct Level0 {
    player: Player,
    enemy: Enemy,
    ball: Ball,
    bricks: Vec<Brick>,
    turn: Turn,
    
    pause: bool,
    game_over: bool,
}

impl Level0 {
    pub fn new() -> Self {
        Self {
            player: Player::default(),
            enemy: Enemy::default(),
            ball: Ball::default(),
            bricks: Vec::new(),
            turn: Turn::Player,
            
            pause: false,
            game_over: true,
        }
    }
}

fn init_brick_line(bricks: &mut Vec<Brick>, y: f32, bricks_per_line: i32, colors: &[Color]) {
    let brick_size = Vector2 {
        x: (SCREEN_WIDTH / bricks_per_line) as f32,
        y: BRICK_HEIGHT,
    };

    for i in 0..bricks_per_line {
        bricks.push(Brick {
            position: Vector2 {
                x: (i as f32) * brick_size.x + brick_size.x / 2.0,
                y,
            },
            size: brick_size.clone(),
            color: colors[(i as usize) % colors.len()].clone(),
            active: true,
        });
    }
}

impl Scene for Level0 {
    fn init(&mut self) {
        self.player = Player::default();
        self.enemy = Enemy::default();
        self.ball = Ball::default();
        self.bricks = Vec::new();
        self.turn = Turn::Player;
        self.pause = false;

        // Top bricks
        init_brick_line(&mut self.bricks, BRICK_HEIGHT / 2.0, BRICKS_PER_LINE, &[GRAY, DARKGRAY]);
        // Bottom bricks
        init_brick_line(&mut self.bricks, SCREEN_HEIGHT as f32 - BRICK_HEIGHT / 2.0, BRICKS_PER_LINE, &[DARKGRAY, GRAY]);
    }

    fn frame(&mut self, delta_time: f32) -> SceneResult {
        self.update(delta_time);
        self.draw();

        SceneResult::OnGoing
    }
}

impl Level0 {
    fn update(&mut self, delta_time: f32) {
        if self.game_over {
            if is_key_pressed(Key::Enter) {
                self.game_over = false;
                self.init();
            }
        } else {
            if is_key_pressed(Key::P) {
                self.pause = !self.pause;
            }

            if self.pause {
                return;
            }

            move_player(&mut self.player, delta_time);

            if !self.ball.active {
                if is_key_pressed(Key::Space) {
                    self.ball.active = true;
                    init_ball(&mut self.ball, &self.turn);
                } else {
                    return;
                }
            }

            move_ball(&mut self.ball, delta_time);
            move_enemy(&mut self.enemy, &self.ball, delta_time);

            on_collision_ball_walls(&mut self.ball, &mut self.player, &mut self.enemy, &mut self.turn);
            on_collision_ball_paddle(&mut self.ball, &self.player.collider());
            on_collision_ball_paddle(&mut self.ball, &self.enemy.collider());
            on_collision_ball_bricks(&mut self.ball, &mut self.bricks);
        }
    }

    fn draw(&self) {
        clear_background(&ELEGANT_BLACK);
        
        if self.game_over {
            draw_text_center("Press [ENTER] to Play", 20, &MAROON);
        } else {
            draw_point(self.player.point, 20, 450);
            draw_point(self.enemy.point, 400, 300);

            draw_player(&self.player);
            draw_enemy(&self.enemy);

            if self.ball.active {
                draw_ball(&self.ball);
            }

            for brick in &self.bricks {
                if brick.active {
                    draw_brick(brick);
                }
            }

            if self.pause {
                draw_text_center("PAUSE", 40, &MAROON);
            }
        }
    }
}

fn on_collision_ball_walls(ball: &mut Ball, player: &mut Player, enemy: &mut Enemy, turn: &mut Turn) {
    if ball.position.x + ball.radius >= SCREEN_WIDTH as f32 || 
        ball.position.x - ball.radius <= 0.0 {
        ball.direction.x *= -1.0;
    }
    if ball.position.y - ball.radius <= 0.0 {
        ball.active = false;
        player.point += 1;
        *turn = Turn::Player;
    }
    if ball.position.y + ball.radius >= SCREEN_HEIGHT as f32 {
        ball.active = false;
        enemy.point += 1;
        *turn = Turn::Enemy;
    }
}

fn on_collision_ball_paddle(ball: &mut Ball, paddle: &Rectangle) {
    if collide_ball(ball, paddle) {
        ball.direction.y *= -1.0;
        ball.direction.x = (ball.position.x - paddle.x) / (paddle.width / 2.0);
        ball.direction.x /= 3.0;

        ball.direction.normalize();
    }
}

fn on_collision_ball_bricks(ball: &mut Ball, bricks: &mut Vec<Brick>) {
    for brick in bricks {
        if brick.active && collide_ball(ball, &brick.collider()) {
            brick.active = false;
            ball.direction.y *= -1.0;

            break;
        }
    }
}

fn draw_text_center(text: &str, font_size: i32, color: &Color) {
    let x = SCREEN_WIDTH / 2 - measure_text(&text, font_size) / 2;
    let y = SCREEN_HEIGHT / 2;

    draw_text(text, x, y, font_size, color);
}

fn draw_point(point: i32, x: i32, y: i32) {
    draw_text(&format!("{}", point), x, y, 40, &GRAY);
}
