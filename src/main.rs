mod graphic;
mod maths;
mod structure;
mod projectile;

#[cfg(test)]
mod unit_tests;

use graphic::renderer::Renderer;
use graphic::camera::Camera;
use graphic::windowing::window::Window;
use maths::segment::Segm3f;
use maths::matrix::Mat4f;
use maths::vector::Vect3f;
use structure::Structure;
use projectile::Projectile;

use std::time::Duration;
use std::time::Instant;

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
    let mut projectiles: Vec<Projectile> = vec![];

    let mut clock = Instant::now();

    while !window.should_close() {
        camera.update_from_events(&window.event_handler());

        if window.event_handler().is_mouse_button_pressed(graphic::windowing::event_handler::MouseButton::Right) {
            structure.apply_transformation(Mat4f::translation(Vect3f::new([0.05, 0.0, -0.05])));
            structure.apply_transformation(Mat4f::rotation_around_x(0.03));
            structure.apply_transformation(Mat4f::rotation_around_y(0.013));
            structure.apply_transformation(Mat4f::rotation_around_z(0.02));
        }

        if window.event_handler().is_mouse_button_pressed(graphic::windowing::event_handler::MouseButton::Left) &&
            clock.elapsed() > Duration::from_secs_f32(1.0 / 10.0) {
            clock = Instant::now();
            let projectile_start = camera.position();
            let projectile_direction = camera.forward();
            let max_distance = 20.0;
            projectiles.push(Projectile::new(projectile_start, projectile_direction, max_distance));
        }

        projectiles.retain_mut(|projectile| {
            let segment_start = projectile.position();
            projectile.moove(1.0);
            let segment_end = projectile.position();
            let mut hit = false;
            if !projectile.is_out_of_max_range() {
                let segment = Segm3f::new(segment_start, segment_end);
                hit = structure.for_first_voxel_in_segment(segment, |voxel: &mut bool| {
                    *voxel = false;
                });
            }
            !hit && !projectile.is_out_of_max_range()
        });

        window.clear();

        let projection_view_matrix = projection_matrix.clone() * camera.view_matrix();
        renderer.render_frame(&projection_view_matrix, &structure, &projectiles);

        window.update();
    }
}
