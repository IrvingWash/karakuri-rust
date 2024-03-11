use sdl2::{event::Event, keyboard::Keycode, EventPump};

pub struct InputResult {
    pub should_quit: bool,
}

impl InputResult {
    pub fn new() -> InputResult {
        InputResult { should_quit: false }
    }
}

pub struct InputController {
    event_pump: EventPump,
    result: InputResult,
}

impl InputController {
    pub fn new(event_pump: EventPump) -> Self {
        Self {
            event_pump,
            result: InputResult::new(),
        }
    }

    pub fn process(&mut self) {
        #[allow(clippy::never_loop)]
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => InputResult { should_quit: true },
                _ => InputResult { should_quit: false },
            };
        }
    }

    pub fn result(&self) -> &InputResult {
        &self.result
    }
}
