use crate::{
    components::{BehaviorComponent, RigidBodyComponent, ShapeComponent, TransformComponent},
    core::{FpsController, InputController, Renderer},
    entity::Entity,
};

pub struct Scene {
    entities: Vec<Option<Entity>>,
    transform_components: Vec<Option<TransformComponent>>,
    rigid_body_components: Vec<Option<RigidBodyComponent>>,
    shape_components: Vec<Option<ShapeComponent>>,
    behavior_components: Vec<Option<Box<dyn BehaviorComponent>>>,
    free_ids: Vec<usize>,
    next_id: usize,
}

impl Default for Scene {
    fn default() -> Scene {
        Scene::new()
    }
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            entities: Vec::new(),
            transform_components: Vec::new(),
            rigid_body_components: Vec::new(),
            shape_components: Vec::new(),
            behavior_components: Vec::new(),
            free_ids: Vec::new(),
            next_id: 0,
        }
    }

    pub fn run(
        &mut self,
        fps_controller: &mut FpsController,
        input_controller: &mut InputController,
        renderer: &mut Renderer,
    ) {
        loop {
            let delta_time = fps_controller.cap_framerate();

            input_controller.process();

            let input_result = input_controller.result();

            if input_result.should_quit {
                break;
            }

            for entity in &self.entities {
                match entity {
                    None => (),
                    Some(entity) => {
                        let behavior = self.behavior_components[*entity].as_mut().unwrap();

                        let transform = self.transform_components[*entity].as_mut().unwrap();
                        let rigid_body = &mut self.rigid_body_components[*entity];
                        let shape = &mut self.shape_components[*entity];

                        behavior.on_update(delta_time, input_result, transform, rigid_body, shape);
                    }
                }
            }

            renderer.start_frame();
            for entity in &self.entities {
                match entity {
                    None => (),
                    Some(entity) => {
                        let transform = self.transform_components[*entity].as_ref().unwrap();
                        let shape = self.shape_components[*entity].as_ref().unwrap();

                        renderer.filled_rectangle(&transform.position, &shape.size, &shape.color)
                    }
                }
            }
            renderer.finish_frame();
        }
    }

    pub fn add_entity(
        &mut self,
        transform: Option<TransformComponent>,
        rigid_body: Option<RigidBodyComponent>,
        shape: Option<ShapeComponent>,
        behavior: Option<Box<dyn BehaviorComponent>>,
    ) {
        match self.free_ids.pop() {
            Some(id) => {
                self.entities[id] = Some(id);

                self.transform_components[id] = Some(transform.unwrap_or_default());
                self.rigid_body_components[id] = rigid_body;
                self.shape_components[id] = shape;
                self.behavior_components[id] = behavior;
            }
            None => {
                self.entities.push(Some(self.next_id));

                self.transform_components
                    .push(Some(transform.unwrap_or_default()));
                self.rigid_body_components.push(rigid_body);
                self.shape_components.push(shape);
                self.behavior_components.push(behavior);

                self.next_id += 1;
            }
        };
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        self.entities[entity] = None;
        self.transform_components[entity] = None;
        self.rigid_body_components[entity] = None;
        self.shape_components[entity] = None;
        self.behavior_components[entity] = None;

        self.free_ids.push(entity);
    }
}
