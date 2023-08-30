mod graphic;
mod maths;

use std::time::Duration;
use std::time::Instant;

use graphic::camera::Camera;
use graphic::material::Material;
use graphic::windowing::event_handler;
use graphic::windowing::window::Window;
use maths::matrix::Mat4f;
use maths::vector::Vect3f;

fn update_events(event_handler: &event_handler::EventHandler, camera: &mut Camera) {
    const CAMERA_SPEED: f32 = 0.03;
    const CAMERA_SENSITIVITY: f32 = 0.005;
    let camera_forward = camera.forward();
    let camera_right = Vect3f::cross(camera_forward, camera.up());

    if event_handler.is_key_pressed(event_handler::Key::W) { camera.position += camera_forward * CAMERA_SPEED }
    if event_handler.is_key_pressed(event_handler::Key::S) { camera.position -= camera_forward * CAMERA_SPEED }
    if event_handler.is_key_pressed(event_handler::Key::A) { camera.position -= camera_right * CAMERA_SPEED }
    if event_handler.is_key_pressed(event_handler::Key::D) { camera.position += camera_right * CAMERA_SPEED }

    let cursor_movement = event_handler.cursor_movement();
    camera.angle_x -= cursor_movement.0 as f32 * CAMERA_SENSITIVITY;
    camera.angle_y -= cursor_movement.1 as f32 * CAMERA_SENSITIVITY;
    camera.angle_y = camera.angle_y.clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());
}

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
    let mut camera = Camera {
        position: Vect3f::new([-1.0, 0.0, 0.0]),
        angle_x: 0.0,
        angle_y: 0.0,
    };

    const MIN_FRAME_TIME_IN_SECS: f32 = 1.0 / 60.0;
    let mut clock = Instant::now();
    while !window.should_close() {
        window.clear();

        let matrix = Mat4f::translation(Vect3f::new([0.0, 1.0, 1.0]));
        mesh.draw(&perspective_matrix, &camera.view_matrix(), &matrix);

        window.update();
        let time_to_sleep = MIN_FRAME_TIME_IN_SECS - clock.elapsed().as_secs_f32();
        if time_to_sleep > 0.0 {
            std::thread::sleep(Duration::from_secs_f32(time_to_sleep));
        }
        clock = Instant::now();
        update_events(&window.event_handler(), &mut camera);
    }
}
