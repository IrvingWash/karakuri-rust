use crate::{
    logger,
    math::Vector2,
    utils::{Color, Resolution},
};
use sdl2::{pixels::Color as SdlColor, rect::Rect, render::Canvas, video::Window, Sdl};

pub struct Renderer {
    canvas: Canvas<Window>,
    clear_color: SdlColor,
}

impl Renderer {
    pub fn new(
        sdl: &Sdl,
        title: &str,
        resolution: Option<Resolution>,
        clear_color: Option<Color>,
    ) -> Self {
        let window = Self::open_window(sdl, title, resolution.unwrap_or(Resolution::new(800, 600)));

        let clear_color = clear_color.unwrap_or(Color::new(0, 0, 200, 255));

        Self {
            canvas: Self::create_canvas(window),
            clear_color: SdlColor::RGBA(
                clear_color.red,
                clear_color.green,
                clear_color.blue,
                clear_color.alpha,
            ),
        }
    }

    pub fn start_frame(&mut self) {
        self.clear_screen();
    }

    pub fn finish_frame(&mut self) {
        self.canvas.present();
    }

    pub fn filled_rectangle(&mut self, position: &Vector2, size: &Vector2, color: &Color) {
        self.canvas.set_draw_color(SdlColor::RGBA(
            color.red,
            color.green,
            color.blue,
            color.alpha,
        ));

        let view = Rect::new(
            position.x as i32,
            position.y as i32,
            size.x as u32,
            size.y as u32,
        );

        self.canvas.fill_rect(view).unwrap_or_else(|e| {
            logger::log_error(format!("Failed to draw SDL2 rectangle: {}", e).as_str());
        });
    }

    fn clear_screen(&mut self) {
        self.canvas.set_draw_color(self.clear_color);

        self.canvas.clear();
    }

    fn open_window(sdl: &Sdl, title: &str, resolution: Resolution) -> Window {
        let video_subsystem = sdl.video().unwrap_or_else(|e| {
            logger::log_fatal(format!("Failed to get SDL2 video subsystem: {}", e).as_str());
        });

        video_subsystem
            .window(title, resolution.width, resolution.height)
            .position_centered()
            .borderless()
            .fullscreen()
            .build()
            .unwrap_or_else(|e| {
                logger::log_fatal(format!("Failed to create SDL2 window: {}", e).as_str())
            })
    }

    fn create_canvas(window: Window) -> Canvas<Window> {
        window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap_or_else(|e| {
                logger::log_fatal(format!("Failed to create SDL2 canvas: {}", e).as_str())
            })
    }
}
