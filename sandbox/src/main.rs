use karakuri::{
    components::{BehaviorComponent, RigidBodyComponent, ShapeComponent, TransformComponent},
    math::Vector2,
    utils::{Color, Resolution},
    Engine, InputResult,
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
        _transform: &mut TransformComponent,
        _rigid_body: &mut Option<RigidBodyComponent>,
        _shape: &mut Option<ShapeComponent>,
    ) {
    }

    fn on_destroy(
        &mut self,
        _transform: &mut TransformComponent,
        _rigid_body: &mut Option<RigidBodyComponent>,
        _shape: &mut Option<ShapeComponent>,
    ) {
    }

    fn on_update(
        &mut self,
        delta_time: f64,
        input_result: &InputResult,
        transform: &mut TransformComponent,
        rigid_body: &mut Option<RigidBodyComponent>,
        _shape: &mut Option<ShapeComponent>,
    ) {
        if input_result.w {
            rigid_body.as_mut().unwrap().velocity.y = -self.speed;
        }

        if input_result.a {
            rigid_body.as_mut().unwrap().velocity.x = -self.speed;
        }

        if input_result.s {
            rigid_body.as_mut().unwrap().velocity.y = self.speed;
        }

        if input_result.d {
            rigid_body.as_mut().unwrap().velocity.x = self.speed;
        }

        transform
            .position
            .add(&rigid_body.as_ref().unwrap().velocity.to_scaled(delta_time));

        rigid_body.as_mut().unwrap().velocity.reset();
    }
}
