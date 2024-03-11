use karakuri::{
    components::{RigidBodyComponent, ShapeComponent, TransformComponent},
    math::Vector2,
    utils::{Color, Resolution},
    Engine,
};

fn main() {
    let mut engine = Engine::new(
        Some("Sonic"),
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
    );

    engine.start();
}
