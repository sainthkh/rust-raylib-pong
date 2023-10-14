use crate::consts::{
    SCREEN_WIDTH, SCREEN_HEIGHT,
    PLAYER_SPEED,
    PADDLE_GRAY, BALL_SPEED,
};

use crate::raylib::{
    Color, Vector2, Rectangle, Circle, 
    check_collision_circle_rec,
    draw_rectangle,
    draw_circle_v,
    is_key_down, Key,
    MAROON,
};

pub enum Turn {
    Player,
    Enemy,
}

pub struct Player {
    pub position: Vector2,
    pub size: Vector2,
    pub point: i32,
    pub speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            position: Vector2 { x: (SCREEN_WIDTH / 2) as f32 , y: (SCREEN_HEIGHT * 7 / 8) as f32 },
            size: Vector2 { x: (SCREEN_WIDTH / 10) as f32, y: 20.0 },
            point: 0,
            speed: PLAYER_SPEED,
        }
    }
}

impl Player {
    pub fn collider(&self) -> Rectangle {
        Rectangle::from(&self.position, &self.size)
    }
}

pub fn move_player(player: &mut Player, delta_time: f32) {
    let mut direction = Vector2 { x: 0.0, y: 0.0 };

    if is_key_down(Key::Left) {
        direction.x -= 1.0;
    }
    
    if is_key_down(Key::Right) {
        direction.x += 1.0;
    }

    direction.normalize();

    player.position += direction * player.speed * delta_time;

    if player.position.x - player.size.x / 2.0 <= 0.0 {
        player.position.x = player.size.x / 2.0;
    }

    if player.position.x + player.size.x / 2.0 >= SCREEN_WIDTH as f32 {
        player.position.x = SCREEN_WIDTH as f32 - player.size.x / 2.0;
    }
}

pub fn draw_player(player: &Player) {
    draw_rectangle(
        (player.position.x - player.size.x / 2.0) as i32, 
        (player.position.y - player.size.y / 2.0) as i32, 
        player.size.x as i32, 
        player.size.y as i32, 
        &PADDLE_GRAY,
    );
}

pub struct Enemy {
    pub position: Vector2,
    pub size: Vector2,
    pub point: i32,
    pub speed: f32,
}

pub fn move_enemy(enemy: &mut Enemy, ball: &Ball, delta_time: f32) {
    if ball.active {
        if ball.position.x < enemy.position.x {
            enemy.position.x -= enemy.speed * delta_time;
        }
        else if ball.position.x > enemy.position.x {
            enemy.position.x += enemy.speed * delta_time;
        }
    }
}

pub fn draw_enemy(enemy: &Enemy) {
    draw_rectangle(
        (enemy.position.x - enemy.size.x / 2.0) as i32, 
        (enemy.position.y - enemy.size.y / 2.0) as i32, 
        enemy.size.x as i32, 
        enemy.size.y as i32, 
        &PADDLE_GRAY,
    );
}

impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            position: Vector2 { x: (SCREEN_WIDTH / 2) as f32, y: (SCREEN_HEIGHT / 8) as f32 },
            size: Vector2 { x: (SCREEN_WIDTH / 10) as f32, y: 20.0 },
            point: 0,
            speed: PLAYER_SPEED,
        }
    }
}

impl Enemy {
    pub fn collider(&self) -> Rectangle {
        Rectangle::from(&self.position, &self.size)
    }
}

pub struct Ball {
    pub position: Vector2,
    pub direction: Vector2,
    pub speed: f32,
    pub radius: f32,
    pub active: bool,
}

impl Ball {
    pub fn collider(&self) -> Circle {
        Circle::from(&self.position, self.radius)
    }
}

impl Default for Ball {
    fn default() -> Self {
        Ball {
            position: Vector2 { x: 0.0, y: 0.0 },
            direction: Vector2 { x: 0.0, y: 0.0 },
            speed: BALL_SPEED,
            radius: 7.0,
            active: false,
        }
    }
}

pub fn init_ball(ball: &mut Ball, turn: &Turn) {
    ball.position = match turn {
        Turn::Player => Vector2 { x: (SCREEN_WIDTH / 2) as f32, y: (SCREEN_HEIGHT * 7 / 8 - 30) as f32 },
        Turn::Enemy => Vector2 { x: (SCREEN_WIDTH / 2) as f32, y: (SCREEN_HEIGHT / 8 + 30) as f32 },
    };

    ball.direction = match turn {
        Turn::Player => Vector2 { x: 0.0, y: -1.0 },
        Turn::Enemy => Vector2 { x: 0.0, y: 1.0 },
    };
}

pub fn collide_ball(ball: &mut Ball, rectangle: &Rectangle) -> bool {
    check_collision_circle_rec(
        &ball.collider(),
        rectangle
    )
}

pub fn move_ball(ball: &mut Ball, delta_time: f32) {
    if ball.active {
        ball.position += ball.direction.clone() * ball.speed * delta_time;
    }
}

pub fn draw_ball(ball: &Ball) {
    draw_circle_v(&ball.position, ball.radius, &MAROON);
}

pub struct Brick {
    pub position: Vector2,
    pub active: bool,
    pub color: Color,
    pub size: Vector2,
}

impl Brick {
    pub fn collider(&self) -> Rectangle {
        Rectangle::from(&self.position, &self.size)
    }
}

pub fn draw_brick(brick: &Brick) {
    draw_rectangle(
        (brick.position.x - brick.size.x / 2.0) as i32, 
        (brick.position.y - brick.size.y / 2.0) as i32, 
        brick.size.x as i32, 
        brick.size.y as i32, 
        &brick.color,
    );
}
