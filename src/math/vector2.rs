use crate::vector2_impl;

pub type Vector2f = Vector2<f64>;
pub type Vector2i = Vector2<i32>;

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

vector2_impl!(f64);
vector2_impl!(i32);

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

