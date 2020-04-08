

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T
}

/// Standard Arithmetic
impl<T: std::marker::Copy + std::clone::Clone + std::ops::Add<Output=T>> std::ops::Add for Vec2<T> {
    // v1 + v2
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Self{x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl<T: std::marker::Copy + std::clone::Clone + std::ops::Sub<Output=T>> std::ops::Sub for Vec2<T> {
    // v1 - v2
    type Output = Vec2<T>;

    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        Self{x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl<T: std::marker::Copy + std::clone::Clone + std::ops::Mul<Output=T>> std::ops::Mul<T> for Vec2<T> {
    // v * c
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self{x: self.x * rhs, y: self.y * rhs}
    }
}

impl<T: std::marker::Copy + std::clone::Clone + std::ops::Div<Output=T>> std::ops::Div<T> for Vec2<T> {
    // v / c
    type Output = Vec2<T>;

    fn div(self, rhs: T) -> Self::Output {
        Self{x: self.x / rhs, y: self.y / rhs}
    }
}

/// Vector Arithmetic
impl<T: std::marker::Copy + std::clone::Clone + std::ops::Add<Output=T> + std::ops::Mul<Output=T>> std::ops::Mul for Vec2<T> {
    // v1 * v2
    type Output = T;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<T> Vec2<T>
where
    T:
    std::fmt::Debug +
    std::marker::Copy +
    std::clone::Clone +
{
    pub fn new(x: T, y: T) -> Self {
        Self{x, y}
    }
}