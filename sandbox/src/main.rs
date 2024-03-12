use karakuri::{
    components::{BehaviorComponent, RigidBodyComponent, ShapeComponent, TransformComponent},
    math::Vector2,
    utils::{Color, Resolution},
    Engine, Entity, InputResult,
};

fn main() {
    let mut engine = Engine::new(
        Some("Sonic The Hedgehog"),
        Some(Resolution::new(800, 600)),
        Some(Color::blue()),
        Some(60),
        Some(20),
    );

    let scene = engine.scene();

    scene.add_entity(
        Some(TransformComponent::new(
            Some(Vector2::new(100., 100.)),
            None,
            None,
        )),
        Some(RigidBodyComponent::new(None, None)),
        Some(ShapeComponent::new(
            Vector2::new(100., 100.),
            Color::white(),
        )),
        Some(Box::new(Sonic::new())),
    );

    engine.start();
}

struct Sonic {
    speed: f64,
}

impl Sonic {
    pub fn new() -> Sonic {
        Sonic { speed: 200. }
    }
}

impl BehaviorComponent for Sonic {
    fn on_start(
        &mut self,
        _entities: &Vec<Option<Entity>>,
        _transform_components: &mut Vec<Option<TransformComponent>>,
        _rigid_body_components: &mut Vec<Option<RigidBodyComponent>>,
        _shape_components: &mut Vec<Option<ShapeComponent>>,
    ) {
    }
    fn on_destroy(
        &mut self,
        _entities: &Vec<Option<Entity>>,
        _transform_components: &mut Vec<Option<TransformComponent>>,
        _rigid_body_components: &mut Vec<Option<RigidBodyComponent>>,
        _shape_components: &mut Vec<Option<ShapeComponent>>,
    ) {
    }

    fn on_update(
        &mut self,
        entity: Entity,
        delta_time: f64,
        input_result: &InputResult,
        _entities: &Vec<Option<Entity>>,
        transform_components: &mut Vec<Option<TransformComponent>>,
        rigid_body_components: &mut Vec<Option<RigidBodyComponent>>,
        _shape_components: &mut Vec<Option<ShapeComponent>>,
    ) {
        let rigid_body = rigid_body_components[entity].as_mut().unwrap();
        let transform = transform_components[entity].as_mut().unwrap();

        if input_result.w {
            rigid_body.velocity.y = -self.speed;
        }

        if input_result.a {
            rigid_body.velocity.x = -self.speed;
        }

        if input_result.s {
            rigid_body.velocity.y = self.speed;
        }

        if input_result.d {
            rigid_body.velocity.x = self.speed;
        }

        transform
            .position
            .add(&rigid_body.velocity.to_scaled(delta_time));

        rigid_body.velocity.reset();
    }
}
