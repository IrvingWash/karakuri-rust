use karakuri::{
    components::{BehaviorComponent, RigidBodyComponent, ShapeComponent, TransformComponent},
    math::Vector2,
    utils::{Color, Resolution},
    Engine,
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
        Some(Box::new(Sonic {})),
    );

    engine.start();
}

struct Sonic {}

impl BehaviorComponent for Sonic {
    fn on_start(&mut self) {}
    fn on_destroy(&mut self) {}
    fn on_update(
        &mut self,
        delta_time: f64,
        transform: &mut TransformComponent,
        rigid_body: &mut Option<RigidBodyComponent>,
    ) {
        rigid_body.as_mut().unwrap().velocity.x = 200.;

        transform
            .position
            .add(&rigid_body.as_ref().unwrap().velocity.to_scaled(delta_time));
    }
}
