pub mod grid;
pub mod macros;
pub mod vector2;

pub use vector2::{Vector2, Vector2f};

#[inline]
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

#[inline]
pub fn clamp_f32_normalized(value: f32) -> f32 {
    clamp(value, 0.0, 1.0)
}

#[inline]
pub fn clamp_f64_normalized(value: f64) -> f64 {
    clamp(value, 0.0, 1.0)
}
