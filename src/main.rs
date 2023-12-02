mod graphic;
mod maths;
mod structure;
mod projectile;
mod voxel;
mod weapon;

#[cfg(test)]
mod unit_tests;

use graphic::renderer::Renderer;
use graphic::camera::Camera;
use graphic::windowing::window::Window;
use maths::segment::Segm3f;
use maths::matrix::Mat4f;
use maths::vector::Vect3f;
use maths::vector::Vect3i;
use projectile::Projectile;
use structure::Structure;
use voxel::Voxel;
use weapon::Weapon;

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
    let mut ghost_cube_position: Option<Vect3i> = None;

    let mut structure = Structure::new(-2, 4, -1, 1, -1, 0);
    let mut projectiles: Vec<Projectile> = vec![];
    let mut weapon = Weapon::new(1.0 / 10.0, 20.0);

    while !window.should_close() {
        camera.update_from_events(&window.event_handler());

        if window.event_handler().is_mouse_button_pressed(graphic::windowing::event_handler::MouseButton::Middle) {
            structure.apply_transformation(Mat4f::translation(Vect3f::new([0.05, 0.0, -0.05])));
            structure.apply_transformation(Mat4f::rotation_around_x(0.03));
            structure.apply_transformation(Mat4f::rotation_around_y(0.013));
            structure.apply_transformation(Mat4f::rotation_around_z(0.02));
        }

        if window.event_handler().is_mouse_button_pressed(graphic::windowing::event_handler::MouseButton::Left) {
            let projectile_start = camera.position();
            let projectile_direction = camera.forward();
            let projectile = weapon.shoot(projectile_start, projectile_direction);
            if projectile.is_some() {
                projectiles.push(projectile.unwrap());
            }
        }

        if ghost_cube_position.is_some() && window.event_handler().is_mouse_button_just_released(graphic::windowing::event_handler::MouseButton::Right) {
            structure.add_voxel(ghost_cube_position.unwrap(), Voxel{});
        }
        if window.event_handler().is_mouse_button_pressed(graphic::windowing::event_handler::MouseButton::Right) {
            let segment = Segm3f::new(camera.position(), camera.position() + camera.forward() * 4.0);
            ghost_cube_position = structure.outside_voxel_coords(segment);
        } else {
            ghost_cube_position = None;
        }

        projectiles.retain_mut(|projectile| {
            let segment_start = projectile.position();
            projectile.moove(1.0);
            let segment_end = projectile.position();
            let mut hit = false;
            if !projectile.is_out_of_max_range() {
                let segment = Segm3f::new(segment_start, segment_end);
                hit = structure.for_first_voxel_in_segment(segment, |voxel: &mut Option<Voxel>, _face| {
                    *voxel = None;
                });
            }
            !hit && !projectile.is_out_of_max_range()
        });

        window.clear();

        let projection_view_matrix = projection_matrix.clone() * camera.view_matrix();
        renderer.render_frame(&projection_view_matrix, &structure, &projectiles, ghost_cube_position);

        window.update();
    }
}
