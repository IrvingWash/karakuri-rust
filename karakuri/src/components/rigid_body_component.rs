use crate::math::Vector2;

pub struct RigidBodyComponent {
    pub velocity: Vector2,
    pub mass: f64,
}

impl Default for RigidBodyComponent {
    fn default() -> Self {
        RigidBodyComponent::new(None, None)
    }
}

impl RigidBodyComponent {
    pub fn new(velocity: Option<Vector2>, mass: Option<f64>) -> RigidBodyComponent {
        RigidBodyComponent {
            velocity: velocity.unwrap_or(Vector2::zero()),
            mass: mass.unwrap_or(1.),
        }
    }
}
