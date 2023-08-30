mod graphic;
mod maths;
mod structure;

use graphic::renderer::Renderer;
use graphic::camera::Camera;
use graphic::windowing::window::Window;
use maths::matrix::Mat4f;
use structure::Structure;

fn main() {
    const WINDOW_WIDTH:u32 = 800;
    const WINDOW_HEIGHT:u32 = 600;

    let mut window = Window::create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "Vorustious");
    let mut renderer = Renderer::new();

    let projection_matrix = {
        let fov = 80.0_f32.to_radians();
        let aspect = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;
        let z_near = 0.1;
        let z_far = 1000.0;
        Mat4f::perspective(fov, aspect, z_near, z_far)
    };
    let mut camera = Camera::new();

    let structure = Structure::new(-2, 4, -1, 1, -1, 0);

    while !window.should_close() {
        camera.update_from_events(&window.event_handler());
        let view_matrix = &camera.view_matrix();

        window.clear();
        renderer.render_frame(&projection_matrix, view_matrix, &structure);
        window.update();
    }
}
