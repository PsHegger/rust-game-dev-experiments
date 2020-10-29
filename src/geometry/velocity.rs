use Vector2d;

#[derive(Copy, Clone, Debug)]
pub struct Velocity {
    pub dir: Vector2d,
    pub speed: f64,
}

impl Velocity {
    pub fn x(self) -> f64 {
        self.speed * self.dir.x
    }

    pub fn y(self) -> f64 {
        self.speed * self.dir.y
    }

    pub fn negate_x(&mut self) {
        self.dir.x = -self.dir.x;
    }

    pub fn negate_y(&mut self) {
        self.dir.y = -self.dir.y;
    }

    pub fn reflect(&mut self, n: &Vector2d) {
        self.dir.reflect(n);
    }
}
