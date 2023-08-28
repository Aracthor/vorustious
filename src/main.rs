mod graphic;

use graphic::shader::Shader;
use graphic::mesh::Mesh;

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
    let mesh = Mesh::create(vertices, shader);

    while !window.should_close() {
        window.clear();

        mesh.draw();

        window.refresh();
        window.update_events();
    }
}
