use sdl2::Sdl;

use crate::{
    core::{FpsController, InputController, Renderer},
    logger,
    scene::Scene,
    utils::{Color, Resolution},
};

pub struct Engine {
    fps_controller: FpsController,
    input_controller: InputController,
    renderer: Renderer,
    scene: Scene,
}

impl Engine {
    pub fn new(
        title: Option<&str>,
        resolution: Option<Resolution>,
        clear_color: Option<Color>,
        target_fps: Option<u32>,
        min_update_fps: Option<u32>,
    ) -> Engine {
        let sdl = Engine::init_sdl();

        Engine {
            scene: Scene::new(),
            renderer: Renderer::new(&sdl, title.unwrap_or("Sandbox"), resolution, clear_color),
            fps_controller: FpsController::new(
                sdl.timer().unwrap_or_else(|e| {
                    logger::log_fatal(format!("Failed to get SDL2 timer: {}", e).as_str())
                }),
                target_fps,
                min_update_fps,
            ),
            input_controller: InputController::new(sdl.event_pump().unwrap_or_else(|e| {
                logger::log_fatal(format!("Failed to get SDL2 event pump: {}", e).as_str())
            })),
        }
    }

    pub fn start(&mut self) {
        self.scene.run(
            &mut self.fps_controller,
            &mut self.input_controller,
            &mut self.renderer,
        );
    }

    pub fn scene(&mut self) -> &mut Scene {
        &mut self.scene
    }

    fn init_sdl() -> Sdl {
        sdl2::init().unwrap_or_else(|e| {
            logger::log_fatal(format!("Failed to initialize SDL2: {}", e).as_str())
        })
    }
}
