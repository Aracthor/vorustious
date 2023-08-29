use glfw::Context;

use super::opengl::context;

pub struct Window {
    core: glfw::Glfw,
    window: glfw::Window,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
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
            core: core,
            window: window,
            events: events,
        }
    }

    pub fn clear(&self) {
        unsafe { context::clear_gl_context() };
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
