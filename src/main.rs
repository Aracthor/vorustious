mod graphic;
mod maths;
mod structure;

use graphic::camera::Camera;
use graphic::frame_limiter::FrameLimiter;
use graphic::material::Material;
use graphic::windowing::window::Window;
use maths::matrix::Mat4f;
use maths::vector::Vect3f;
use structure::Structure;

fn main() {
    const WINDOW_WIDTH:u32 = 800;
    const WINDOW_HEIGHT:u32 = 600;

    let mut window = Window::create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "Vorustious");

    let mut material = Material::create("shaders/hello_texture.vert", "shaders/hello_texture.frag");

    let texture = graphic::cube::cube_texture([0x40, 0x40, 0x40, 0xFF], [0x80, 0x80, 0x80, 0xFF]);
    material.add_texture(texture);

    let mesh = graphic::cube::cube_mesh(material);

    let perspective_matrix = {
        let fov = 80.0_f32.to_radians();
        let aspect = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;
        let z_near = 0.1;
        let z_far = 1000.0;
        Mat4f::perspective(fov, aspect, z_near, z_far)
    };
    let mut camera = Camera::new(Vect3f::new([-1.0, 0.0, 0.0]));

    let structure = Structure::new(-2, 4, -1, 1, -1, 0);

    let mut frame_limiter = FrameLimiter::new(60.0);
    while !window.should_close() {
        window.clear();

        let view_matrix = &camera.view_matrix();
        structure.for_each_voxel(|x, y, z| {
            let model_matrix = Mat4f::translation(Vect3f::new([x as f32, y as f32, z as f32]));
            mesh.draw(&perspective_matrix, view_matrix, &model_matrix);
        });

        window.update();
        frame_limiter.limit();
        camera.update_from_events(&window.event_handler());
    }
}
