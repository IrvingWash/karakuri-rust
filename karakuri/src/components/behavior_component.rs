use crate::{core::InputResult, entity::Entity};

use super::{RigidBodyComponent, ShapeComponent, TransformComponent};

pub trait BehaviorComponent {
    fn on_start(
        &mut self,
        entities: &Vec<Option<Entity>>,
        transform_components: &mut Vec<Option<TransformComponent>>,
        rigid_body_components: &mut Vec<Option<RigidBodyComponent>>,
        shape_components: &mut Vec<Option<ShapeComponent>>,
    );
    fn on_update(
        &mut self,
        entity: Entity,
        delta_time: f64,
        input_result: &InputResult,
        entities: &Vec<Option<Entity>>,
        transform_components: &mut Vec<Option<TransformComponent>>,
        rigid_body_components: &mut Vec<Option<RigidBodyComponent>>,
        shape_components: &mut Vec<Option<ShapeComponent>>,
    );
    fn on_destroy(
        &mut self,
        entities: &Vec<Option<Entity>>,
        transform_components: &mut Vec<Option<TransformComponent>>,
        rigid_body_components: &mut Vec<Option<RigidBodyComponent>>,
        shape_components: &mut Vec<Option<ShapeComponent>>,
    );
}
