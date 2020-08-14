use crate::common::math::Vector2f;

#[derive(Default, Debug, Clone)]
pub struct Blob {
    pub id: usize,
    pub size: f64,
    pub position: Vector2f,
    pub velocity: Vector2f,
    pub acceleration: Vector2f,
}

impl Blob {
    pub fn new(id: usize, size: f64, position: impl Into<Vector2f>) -> Self {
        Self {
            id,
            size,
            position: position.into(),
            ..Default::default()
        }
    }

    pub fn contains_point(&self, point: &Vector2f) -> bool {
        point.x >= self.position.x 
            && point.x <= self.position.x + self.size
            && point.y >= self.position.y
            && point.y <= self.position.y + self.size
    }
}
