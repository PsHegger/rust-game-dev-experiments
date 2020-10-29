use piston_window::{ellipse, Context, G2d};
use std::option::Option::Some;
use utils::entity::{Renderable, Updatable};
use Vector2d;
use Velocity;

#[derive(Copy, Clone, Debug)]
pub struct Ball {
    pub id: u32,
    pub velocity: Velocity,
    pub pos: Vector2d,
    pub radius: f64,
    pub color: [f32; 4],
}

impl Ball {
    pub fn mv(&mut self, delta_time: f64) {
        self.pos.x += delta_time * self.velocity.x();
        self.pos.y += delta_time * self.velocity.y();
    }

    pub fn update_wall_collision(&mut self, width: u32, height: u32) {
        let w = width as f64;
        let h = height as f64;
        if self.pos.x >= w - self.radius {
            self.pos.x = w - self.radius;
            self.velocity.negate_x();
        }

        if self.pos.x < self.radius {
            self.pos.x = self.radius;
            self.velocity.negate_x();
        }

        if self.pos.y >= h - self.radius {
            self.pos.y = h - self.radius;
            self.velocity.negate_y();
        }

        if self.pos.y < self.radius {
            self.pos.y = self.radius;
            self.velocity.negate_y();
        }
    }

    pub fn collides_with_ball(self, b: &Ball) -> bool {
        (b.id != self.id) && (b.pos - self.pos).length() <= (b.radius + self.radius)
    }
}

impl Renderable for Ball {
    fn render(self, c: Context, g: &mut G2d) {
        let rect: [f64; 4] = [
            self.pos.x - self.radius,
            self.pos.y - self.radius,
            self.radius * 2.0,
            self.radius * 2.0,
        ];
        ellipse(self.color, rect, c.transform, g);
    }
}

impl Updatable for Ball {
    type Args = BallUpdateArgs;

    fn update(&mut self, args: Self::Args) {
        self.mv(args.dt);
        self.update_wall_collision(args.width, args.height);

        if let Some(balls) = args.balls {
            for j in 0..balls.len() {
                let b = balls[j];
                if !self.collides_with_ball(&b) {
                    continue;
                }
                let d = (self.pos - b.pos).length() - (self.radius + b.radius);
                let new_pos = self.pos + self.velocity.dir * d;
                self.pos.x = new_pos.x;
                self.pos.y = new_pos.y;
                let mut n = b.pos - self.pos;
                n.normalize();
                self.velocity.reflect(&n);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct BallUpdateArgs {
    pub dt: f64,
    pub width: u32,
    pub height: u32,
    pub balls: Option<Vec<Ball>>,
}

#[derive(Copy, Clone, Debug)]
pub struct BallSettings {
    min_velocity: f64,
    max_velocity: f64,
    max_x: f64,
    max_y: f64,
    min_radius: f64,
    max_radius: f64,
    color: Option<[f32; 4]>,
}

#[allow(dead_code)]
impl BallSettings {
    pub fn new() -> BallSettings {
        BallSettings {
            min_velocity: 500.0,
            max_velocity: 1000.0,
            max_x: 100.0,
            max_y: 100.0,
            min_radius: 10.0,
            max_radius: 20.0,
            color: None,
        }
    }

    pub fn set_velocity(&mut self, min: f64, max: f64) -> &mut BallSettings {
        self.min_velocity = min;
        self.max_velocity = max;
        self
    }

    pub fn set_max_pos(&mut self, x: f64, y: f64) -> &mut BallSettings {
        self.max_x = x;
        self.max_y = y;
        self
    }

    pub fn set_radius(&mut self, min: f64, max: f64) -> &mut BallSettings {
        self.min_radius = min;
        self.max_radius = max;
        self
    }

    pub fn build(self, id: u32) -> Ball {
        let mut dir = Vector2d {
            x: rand::random::<f64>() * 2.0 - 1.0,
            y: rand::random::<f64>() * 2.0 - 1.0,
        };
        dir.normalize();

        let color = if let Some(c) = self.color {
            c
        } else {
            [rand::random(), rand::random(), rand::random(), 1.0]
        };

        Ball {
            id,
            velocity: Velocity {
                dir,
                speed: rand::random::<f64>() * (self.max_velocity - self.min_velocity)
                    + self.min_velocity,
            },
            pos: Vector2d {
                x: rand::random::<f64>() * self.max_x,
                y: rand::random::<f64>() * self.max_y,
            },
            radius: rand::random::<f64>() * (self.max_radius - self.min_radius) + self.min_radius,
            color,
        }
    }
}
