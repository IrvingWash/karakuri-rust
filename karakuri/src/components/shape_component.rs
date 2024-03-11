use crate::{math::Vector2, utils::Color};

pub struct ShapeComponent {
    pub size: Vector2,
    pub color: Color,
}

impl ShapeComponent {
    pub fn new(size: Vector2, color: Color) -> ShapeComponent {
        ShapeComponent { size, color }
    }
}
