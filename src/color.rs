use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl From<[f32; 4]> for Color {
    fn from(value: [f32; 4]) -> Self {
        Self::new(value[0], value[1], value[2], value[3])
    }
}

// Color-Color Ops
impl AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}

impl SubAssign<Color> for Color {
    fn sub_assign(&mut self, rhs: Color) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
        self.a -= rhs.a;
    }
}

impl MulAssign<Color> for Color {
    fn mul_assign(&mut self, rhs: Color) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
        self.a *= rhs.a;
    }
}

impl DivAssign<Color> for Color {
    fn div_assign(&mut self, rhs: Color) {
        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
        self.a /= rhs.a;
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(mut self, rhs: Color) -> Self::Output {
        self += rhs;
        self
    }
}

impl Sub<Color> for Color {
    type Output = Color;

    fn sub(mut self, rhs: Color) -> Self::Output {
        self -= rhs;
        self
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(mut self, rhs: Color) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Div<Color> for Color {
    type Output = Color;

    fn div(mut self, rhs: Color) -> Self::Output {
        self /= rhs;
        self
    }
}

// Color-f32 Ops
impl Add<f32> for Color {
    type Output = Color;

    fn add(mut self, rhs: f32) -> Self::Output {
        self += rhs;
        self
    }
}

impl Sub<f32> for Color {
    type Output = Color;

    fn sub(mut self, rhs: f32) -> Self::Output {
        self -= rhs;
        self
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(mut self, rhs: f32) -> Self::Output {
        self /= rhs;
        self
    }
}

impl AddAssign<f32> for Color {
    fn add_assign(&mut self, rhs: f32) {
        self.r += rhs;
        self.g += rhs;
        self.b += rhs;
        self.a += rhs;
    }
}

impl SubAssign<f32> for Color {
    fn sub_assign(&mut self, rhs: f32) {
        self.r -= rhs;
        self.g -= rhs;
        self.b -= rhs;
        self.a -= rhs;
    }
}

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
        self.a *= rhs;
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, rhs: f32) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
        self.a /= rhs;
    }
}
