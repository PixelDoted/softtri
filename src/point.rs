use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<[f32; 2]> for Point2D {
    fn from(value: [f32; 2]) -> Self {
        Self::new(value[0], value[1])
    }
}

// Point2D-Point2D Ops
impl AddAssign<Point2D> for Point2D {
    fn add_assign(&mut self, rhs: Point2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign<Point2D> for Point2D {
    fn sub_assign(&mut self, rhs: Point2D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl MulAssign<Point2D> for Point2D {
    fn mul_assign(&mut self, rhs: Point2D) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl DivAssign<Point2D> for Point2D {
    fn div_assign(&mut self, rhs: Point2D) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Add<Point2D> for Point2D {
    type Output = Point2D;

    fn add(mut self, rhs: Point2D) -> Self::Output {
        self += rhs;
        self
    }
}

impl Sub<Point2D> for Point2D {
    type Output = Point2D;

    fn sub(mut self, rhs: Point2D) -> Self::Output {
        self -= rhs;
        self
    }
}

impl Mul<Point2D> for Point2D {
    type Output = Point2D;

    fn mul(mut self, rhs: Point2D) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Div<Point2D> for Point2D {
    type Output = Point2D;

    fn div(mut self, rhs: Point2D) -> Self::Output {
        self /= rhs;
        self
    }
}

// Point2D-f32 Ops
impl Add<f32> for Point2D {
    type Output = Point2D;

    fn add(mut self, rhs: f32) -> Self::Output {
        self += rhs;
        self
    }
}

impl Sub<f32> for Point2D {
    type Output = Point2D;

    fn sub(mut self, rhs: f32) -> Self::Output {
        self -= rhs;
        self
    }
}

impl Mul<f32> for Point2D {
    type Output = Point2D;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Div<f32> for Point2D {
    type Output = Point2D;

    fn div(mut self, rhs: f32) -> Self::Output {
        self /= rhs;
        self
    }
}

impl AddAssign<f32> for Point2D {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl SubAssign<f32> for Point2D {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl MulAssign<f32> for Point2D {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl DivAssign<f32> for Point2D {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
