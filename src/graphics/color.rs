use crate::common::math;
use std::ops::Range;

pub type Color = [f32; 3];

pub const BLACK: Color = [0.0, 0.0, 0.0];
pub const WHITE: Color = [1.0, 1.0, 1.0];

pub const GRAY: Color = [0.5, 0.5, 0.5];
pub const LIGHT_GRAY: Color = [0.75, 0.75, 0.75];
pub const DARK_GRAY: Color = [0.25, 0.25, 0.25];

pub const RED: Color = [1.0, 0.0, 0.0];
pub const GREEN: Color = [0.0, 1.0, 0.0];
pub const BLUE: Color = [0.0, 0.0, 1.0];

const PRIMARY_MIN: f32 = 0.75;
const PRIMARY_MAX: f32 = 1.0;
const SECONDARY_MIN: f32 = 0.0;
const SECONDARY_MAX: f32 = 0.10;

pub fn random_grayscale() -> Color {
    let value = rand::random();
    [value, value, value]
}

pub fn random_red() -> Color {
    random_color_range(
        PRIMARY_MIN..PRIMARY_MAX,
        SECONDARY_MIN..SECONDARY_MAX,
        SECONDARY_MIN..SECONDARY_MAX,
    )
}

pub fn random_green() -> Color {
    random_color_range(
        SECONDARY_MIN..SECONDARY_MAX,
        PRIMARY_MIN..PRIMARY_MAX,
        SECONDARY_MIN..SECONDARY_MAX,
    )
}

pub fn random_blue() -> Color {
    random_color_range(
        SECONDARY_MIN..SECONDARY_MAX,
        SECONDARY_MIN..SECONDARY_MAX,
        PRIMARY_MIN..PRIMARY_MAX,
    )
}

pub fn random_color() -> Color {
    let r = rand::random();
    let g = rand::random();
    let b = rand::random();
    [r, g, b]
}

pub fn random_color_range(
    red_range: Range<f32>,
    green_range: Range<f32>,
    blue_range: Range<f32>,
) -> Color {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let r = math::clamp_f32_normalized(rng.gen_range(red_range.start, red_range.end));
    let g = math::clamp_f32_normalized(rng.gen_range(green_range.start, green_range.end));
    let b = math::clamp_f32_normalized(rng.gen_range(blue_range.start, blue_range.end));

    [r, g, b]
}
