use crate::core::InputResult;

use super::{RigidBodyComponent, ShapeComponent, TransformComponent};

pub trait BehaviorComponent {
    fn on_start(
        &mut self,
        transform: &mut TransformComponent,
        rigid_body: &mut Option<RigidBodyComponent>,
        shape: &mut Option<ShapeComponent>,
    );
    fn on_update(
        &mut self,
        delta_time: f64,
        input_result: &InputResult,
        transform: &mut TransformComponent,
        rigid_body: &mut Option<RigidBodyComponent>,
        shape: &mut Option<ShapeComponent>,
    );
    fn on_destroy(
        &mut self,
        transform: &mut TransformComponent,
        rigid_body: &mut Option<RigidBodyComponent>,
        shape: &mut Option<ShapeComponent>,
    );
}
