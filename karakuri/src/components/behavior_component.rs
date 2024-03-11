use super::{RigidBodyComponent, TransformComponent};

pub trait BehaviorComponent {
    fn on_start(&mut self);
    fn on_update(
        &mut self,
        delta_time: f64,
        transform: &mut TransformComponent,
        rigid_body: &mut Option<RigidBodyComponent>,
    );
    fn on_destroy(&mut self);
}
