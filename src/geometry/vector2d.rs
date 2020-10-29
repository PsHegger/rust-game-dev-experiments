use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vector2d {
    pub x: f64,
    pub y: f64,
}

impl ops::Add<Vector2d> for Vector2d {
    type Output = Vector2d;

    fn add(self, rhs: Vector2d) -> Self::Output {
        Vector2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Vector2d> for Vector2d {
    type Output = Vector2d;

    fn sub(self, rhs: Vector2d) -> Self::Output {
        Vector2d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<f64> for Vector2d {
    type Output = Vector2d;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

// Dot product
impl ops::Mul<Vector2d> for Vector2d {
    type Output = f64;

    fn mul(self, rhs: Vector2d) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl ops::Div<f64> for Vector2d {
    type Output = Vector2d;

    fn div(self, rhs: f64) -> Self::Output {
        Vector2d {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

#[allow(dead_code)]
impl Vector2d {
    pub fn zero() -> Vector2d {
        Vector2d { x: 0.0, y: 0.0 }
    }

    pub fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&mut self) {
        let length = self.length();
        if length != 0.0 {
            self.x /= length;
            self.y /= length;
        }
    }

    pub fn reflect(&mut self, n: &Vector2d) {
        let dot_product = (*self) * (*n);
        let a = (*n) * 2.0 * dot_product;
        self.x = self.x - a.x;
        self.y = self.y - a.y;
    }
}
