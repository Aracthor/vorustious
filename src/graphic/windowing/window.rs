use glfw::Context;

use super::event_handler::EventHandler;
use crate::graphic::opengl::context;

pub struct Window {
    window: glfw::Window,
    event_handler: EventHandler,

    width: u32,
    height: u32,
}

impl Window {
    pub fn create_window(width: u32, height: u32, title: &str) -> Window {
        let mut core = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        core.window_hint(glfw::WindowHint::ContextVersion(3, 0));
        core.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::OpenGlEs));
        core.window_hint(glfw::WindowHint::Resizable(false));

        let (mut window, events) = core.create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.make_current();
        window.set_key_polling(true);
        let middle_pos = (width as f64 / 2.0, height as f64 / 2.0);
        window.set_cursor_pos(middle_pos.0, middle_pos.1);
        window.set_cursor_mode(glfw::CursorMode::Hidden);
        gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

        unsafe { context::start_gl_context(width.try_into().unwrap(), height.try_into().unwrap()) };

        Self {
            window: window,
            event_handler: EventHandler::new(core, events),

            width: width,
            height: height,
        }
    }

    pub fn clear(&self) {
        unsafe { context::clear_gl_context() };
    }

    pub fn update(&mut self) {
        self.window.swap_buffers();

        self.event_handler.update();

        let middle_pos = (self.width as f64 / 2.0, self.height as f64 / 2.0);
        let cursor_pos = self.window.get_cursor_pos();
        let cursor_movement = (cursor_pos.0 - middle_pos.0, cursor_pos.1 - middle_pos.1);
        self.event_handler.set_cursor_movement(cursor_movement);
        self.window.set_cursor_pos(middle_pos.0, middle_pos.1);
    }

    pub fn event_handler(&self) -> &EventHandler {
        &self.event_handler
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }
}
