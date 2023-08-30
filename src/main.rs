mod graphic;
mod maths;

use std::time::Duration;
use std::time::Instant;

use graphic::material::Material;
use maths::matrix::Mat4f;
use maths::vector::Vect3f;

fn main() {
    const WINDOW_WIDTH:u32 = 800;
    const WINDOW_HEIGHT:u32 = 600;

    let mut window = graphic::window::Window::create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "Vorustious");

    let mut material = Material::create("shaders/hello_texture.vert", "shaders/hello_texture.frag");

    let texture = graphic::cube::cube_texture([0x40, 0x40, 0x40, 0xFF], [0x80, 0x80, 0x80, 0xFF]);
    material.add_texture(texture);

    let mesh = graphic::cube::cube_mesh(material);

    let perspective_matrix = {
        let half_width:f32 = 1.6;
        let half_height:f32 = 1.2;
        let right = half_width;
        let left = -half_width;
        let bottom = -half_height;
        let top = half_height;
        let z_near = 0.1;
        let z_far = 1000.0;
        Mat4f::orthographic(left, right, bottom, top, z_near, z_far)
    };
    let view_matrix = {
        let eye = Vect3f::new([1.0, 1.0, 1.0]);
        let target = Vect3f::new([0.0, 0.0, 0.0]);
        let up = Vect3f::new([0.0, 1.0, 0.0]);
        Mat4f::look_at(eye, target, up)
    };

    const MIN_FRAME_TIME_IN_SECS: f32 = 1.0 / 60.0;
    let mut clock = Instant::now();
    while !window.should_close() {
        window.clear();

        mesh.draw(&perspective_matrix, &view_matrix);

        window.refresh();
        let time_to_sleep = MIN_FRAME_TIME_IN_SECS - clock.elapsed().as_secs_f32();
        if time_to_sleep > 0.0 {
            std::thread::sleep(Duration::from_secs_f32(time_to_sleep));
        }
        clock = Instant::now();
        window.update_events();
    }
}
