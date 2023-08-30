use glfw::Context;

use super::event_handler::EventHandler;
use super::opengl::context;

pub struct Window {
    window: glfw::Window,
    pub event_handler: EventHandler,
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
        gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

        unsafe { context::start_gl_context(width.try_into().unwrap(), height.try_into().unwrap()) };

        Self {
            window: window,
            event_handler: EventHandler::new(core, events),
        }
    }

    pub fn clear(&self) {
        unsafe { context::clear_gl_context() };
    }

    pub fn refresh(&mut self) {
        self.window.swap_buffers();
    }

    pub fn should_close(&self) -> bool {
        return self.window.should_close();
    }
}
