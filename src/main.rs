mod graphic;
mod maths;
mod structure;

use graphic::renderer::Renderer;
use graphic::camera::Camera;
use graphic::windowing::window::Window;
use maths::segment::Segm3f;
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

    let mut structure = Structure::new(-2, 4, -1, 1, -1, 0);

    while !window.should_close() {
        camera.update_from_events(&window.event_handler());
        let projection_view_matrix = projection_matrix * camera.view_matrix();

        if window.event_handler().is_mouse_button_pressed(graphic::windowing::event_handler::MouseButton::Left) {
            let ray_start = camera.position();
            let ray_direction = camera.forward();
            let segment = Segm3f::new(ray_start, ray_start + ray_direction * 100.0);
            structure.for_voxels_in_segment(segment, |voxel: &mut bool| {*voxel = false });
        }

        window.clear();
        renderer.render_frame(&projection_view_matrix, &structure);
        window.update();
    }
}
