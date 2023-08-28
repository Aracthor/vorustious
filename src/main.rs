mod shader;
mod window;

use shader::Shader;

fn main() {
    let mut window = window::Window::create_window(800, 600, "Vorustious");

    let shader = Shader::create_shader_program("shaders/hello_triangle.vert", "shaders/hello_triangle.frag");

    while !window.should_close() {
        window.clear();
        window.refresh();
        window.update_events();

        shader.use_program();
    }
}
