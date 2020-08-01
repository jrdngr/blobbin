pub type Vector2f = Vector2<f64>;

#[derive(Debug, Clone)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Vector2f {
    pub fn magnitude(&self) -> f64 {
        let sum_squares = (self.x * self.x) + (self.y * self.y);
        sum_squares.sqrt()
    }

    pub fn distance(&self, other: &Vector2f) -> f64 {
        self.vector_to(other).magnitude()
    }

    pub fn vector_to(&self, target: &Vector2f) -> Vector2f {
        self - target
    }

    pub fn normalized(&self) -> Vector2f {
        let magnitude = self.magnitude();
        Vector2f {
            x: self.x / magnitude,
            y: self.y / magnitude,
        }
    }
}

impl std::ops::Add<&Vector2f> for &Vector2f {
    type Output = Vector2f;

    fn add(self, rhs: &Vector2f) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign<&Vector2f> for Vector2f {
    fn add_assign(&mut self, rhs: &Vector2f) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Add<Vector2f> for &Vector2f {
    type Output = Vector2f;

    fn add(self, rhs: Vector2f) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign<Vector2f> for Vector2f {
    fn add_assign(&mut self, rhs: Vector2f) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::AddAssign<f64> for Vector2f {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl std::ops::Sub<&Vector2f> for &Vector2f {
    type Output = Vector2f;

    fn sub(self, rhs: &Vector2f) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::SubAssign<&Vector2f> for Vector2f {
    fn sub_assign(&mut self, rhs: &Vector2f) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Sub<Vector2f> for &Vector2f {
    type Output = Vector2f;

    fn sub(self, rhs: Vector2f) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::SubAssign<Vector2f> for Vector2f {
    fn sub_assign(&mut self, rhs: Vector2f) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::SubAssign<f64> for Vector2f {
    fn sub_assign(&mut self, rhs: f64) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl std::ops::Mul<f64> for &Vector2f {
    type Output = Vector2f;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Mul<f64> for Vector2f {
    type Output = Vector2f;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Mul<&Vector2f> for f64 {
    type Output = Vector2f;

    fn mul(self, rhs: &Vector2f) -> Self::Output {
        Vector2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl std::ops::Mul<Vector2f> for f64 {
    type Output = Vector2f;

    fn mul(self, rhs: Vector2f) -> Self::Output {
        Vector2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl std::ops::MulAssign<f64> for Vector2f {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl std::ops::Div<f64> for &Vector2f {
    type Output = Vector2f;

    fn div(self, rhs: f64) -> Self::Output {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::ops::Div<f64> for Vector2f {
    type Output = Vector2f;

    fn div(self, rhs: f64) -> Self::Output {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::ops::DivAssign<f64> for Vector2f {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T: Default> Default for Vector2<T> {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl<T> From<(T, T)> for Vector2<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

