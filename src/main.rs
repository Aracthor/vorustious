mod graphic;

use graphic::shader::Shader;
use graphic::vertex_objects::VertexArrayObject;

fn main() {
    let mut window = graphic::window::Window::create_window(800, 600, "Vorustious");

    let shader = Shader::create_shader_program("shaders/hello_triangle.vert", "shaders/hello_triangle.frag");

    let vertices: Vec<f32> = [
        -0.5, 0.5, 0.0,
        0.5, 0.5, 0.0,
        0.5, -0.5, 0.0,

        -0.5, 0.5, 0.0,
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
    ].to_vec();
    let mut vao = VertexArrayObject::create();
    vao.set_vertices(vertices);

    while !window.should_close() {
        window.clear();

        shader.use_program();
        vao.draw();

        window.refresh();
        window.update_events();
    }
}
