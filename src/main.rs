mod graphic;
mod maths;

use graphic::material::Material;
use maths::matrix::Mat4f;

fn main() {
    const WINDOW_WIDTH:u32 = 800;
    const WINDOW_HEIGHT:u32 = 600;

    let mut window = graphic::window::Window::create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "Vorustious");

    let mut material = Material::create("shaders/hello_texture.vert", "shaders/hello_texture.frag");

    let texture = graphic::cube::cube_texture([0x40, 0x40, 0x40, 0xFF], [0x80, 0x80, 0x80, 0xFF]);
    material.add_texture(texture);

    let mesh = graphic::cube::cube_mesh(material);

    let perspective_matrix = {
        let half_width:f32 = 0.8;
        let half_height:f32 = 0.6;
        let right = half_width;
        let left = -half_width;
        let bottom = -half_height;
        let top = half_height;
        let z_near = 0.1;
        let z_far = 1000.0;
        Mat4f::orthographic(left, right, bottom, top, z_near, z_far)
    };

    while !window.should_close() {
        window.clear();

        mesh.draw(&perspective_matrix);

        window.refresh();
        window.update_events();
    }
}
