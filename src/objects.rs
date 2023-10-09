use crate::consts::{SCREEN_HEIGHT, SCREEN_WIDTH, };
use crate::raylib::{
    Color, Vector2, Rectangle, Circle, 
    check_collision_circle_rec,
    draw_rectangle,
    draw_circle_v,
    is_key_down, Key,
    MAROON
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

    pub color: Color,
}

impl Player {
    pub fn movement(&mut self) {
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

    pub fn movement_by_ai(&mut self, ball: &Ball) {
        if ball.active {
            if ball.position.x < self.position.x {
                self.position.x -= self.speed;
            }
            else if ball.position.x > self.position.x {
                self.position.x += self.speed;
            }
        }
    }

    pub fn collider(&self) -> Rectangle {
        Rectangle::from(&self.position, &self.size)
    }

    pub fn draw(&self) {
        draw_rectangle(
            (self.position.x - self.size.x / 2.0) as i32, 
            (self.position.y - self.size.y / 2.0) as i32, 
            self.size.x as i32, 
            self.size.y as i32, 
            &self.color);
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
    pub fn movement(&mut self) {
        if self.active {
            self.position += self.direction.clone() * self.speed;
        }
    }

    pub fn collides(&self, rectangle: &Rectangle) -> bool {
        check_collision_circle_rec(
            &Circle {
                center: self.position.clone(),
                radius: self.radius,
            }, 
            rectangle
        )
    }

    pub fn init_position(&mut self, turn: &Turn) {
        self.position = match turn {
            Turn::Player => Vector2 { x: (SCREEN_WIDTH / 2) as f32, y: (SCREEN_HEIGHT * 7 / 8 - 30) as f32 },
            Turn::Enemy => Vector2 { x: (SCREEN_WIDTH / 2) as f32, y: (SCREEN_HEIGHT / 8 + 30) as f32 },
        }
    }

    pub fn init_direction(&mut self, turn: &Turn) {
        self.direction = match turn {
            Turn::Player => Vector2 { x: 0.0, y: -1.0 },
            Turn::Enemy => Vector2 { x: 0.0, y: 1.0 },
        }
    }

    pub fn draw(&self) {
        if self.active {
            draw_circle_v(&self.position, self.radius, &MAROON);
        }
    }
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

    pub fn draw(&self) {
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
