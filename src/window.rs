extern crate glfw;

use glfw::Context;

pub struct Window {
    core: glfw::Glfw,
    window: glfw::Window,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
}

impl Window {
    pub fn create_window(width: u32, height: u32, title: &str) -> Window {
        let mut core = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (mut window, events) = core.create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.make_current();
        window.set_key_polling(true);
        
        Self {
            core: core,
            window: window,
            events: events,
        }
    }

    pub fn refresh(&mut self) {
        self.window.swap_buffers();
    }

    pub fn update_events(&mut self) {
        self.core.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    self.window.set_should_close(true)
                },
                _ => {},
            }
        }
    }

    pub fn should_close(&self) -> bool {
        return self.window.should_close();
    }
}
