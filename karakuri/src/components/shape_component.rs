use crate::{math::Vector2, utils::Color};

pub struct ShapeComponent {
    size: Vector2,
    color: Color,
}

impl ShapeComponent {
    pub fn new(size: Vector2, color: Color) -> ShapeComponent {
        ShapeComponent { size, color }
    }
}
