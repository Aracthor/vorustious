mod graphic;
mod maths;
mod voxels;
mod projectile;
mod weapon;

use graphic::renderer::Renderer;
use graphic::camera::Camera;
use graphic::windowing::window::Window;
use graphic::windowing::event_handler::Key;
use graphic::windowing::event_handler::MouseButton;
use maths::segment::Segm3f;
use maths::matrix::Mat4f;
use maths::vector::Vect3f;
use maths::vector::Vect3i;
use projectile::Projectile;
use voxels::body::Body;
use voxels::structure::Structure;
use voxels::catalog::VoxelCatalog;
use voxels::voxel::Voxel;
use voxels::voxel::VoxelID;
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

    let voxel_catalog = VoxelCatalog::create();
    let voxel = voxel_catalog.create_voxel(VoxelID::LightHull);
    let structure = {
        let mut structure = Structure::new(-2, 4, -1, 1, -1, 0, voxel);
        structure.add_voxel(Vect3i::new([0, 0, 0]), voxel_catalog.create_voxel(VoxelID::ShipCore));
        structure
    };
    let mut body = Body::new(structure);
    let mut projectiles: Vec<Projectile> = vec![];
    let mut weapon = Weapon::new(1.0 / 10.0, 0.5, 20.0);
    let save_filename = "save.vors";

    while !window.should_close() {
        camera.update_from_events(&window.event_handler());

        if window.event_handler().is_key_just_pressed(Key::F5) {
            let str = body.structure().serialize();
            std::fs::write(save_filename, str).expect(&format!("Unable to save {save_filename}"));
            println!("Saved file '{save_filename}'");
        }
        if window.event_handler().is_key_just_pressed(Key::F9) {
            let str = std::fs::read_to_string(save_filename).expect(&format!("Unable to read {save_filename}"));
            body = Body::new(Structure::deserialize(&voxel_catalog, &str));
            println!("Loaded file '{save_filename}'");
        }

        if window.event_handler().is_mouse_button_pressed(MouseButton::Middle) {
            body.apply_transformation(Mat4f::translation(Vect3f::new([0.05, 0.0, -0.05])));
            body.apply_transformation(Mat4f::rotation_around_x(0.03));
            body.apply_transformation(Mat4f::rotation_around_y(0.013));
            body.apply_transformation(Mat4f::rotation_around_z(0.02));
        }

        if window.event_handler().is_mouse_button_pressed(MouseButton::Left) {
            let projectile_start = camera.position();
            let projectile_direction = camera.forward();
            let projectile = weapon.shoot(projectile_start, projectile_direction);
            if projectile.is_some() {
                projectiles.push(projectile.unwrap());
            }
        }

        if ghost_cube_position.is_some() && window.event_handler().is_mouse_button_just_released(MouseButton::Right) {
            body.structure_mut().add_voxel(ghost_cube_position.unwrap(), voxel);
        }
        if window.event_handler().is_mouse_button_pressed(MouseButton::Right) {
            let segment = Segm3f::new(camera.position(), camera.position() + camera.forward() * 4.0).transform(&body.repere().inverse());
            ghost_cube_position = body.structure_mut().outside_voxel_coords(segment);
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
                hit = body.for_first_voxel_in_segment(segment, |voxel: &mut Option<Voxel>| {
                    voxel.as_mut().unwrap().life -= projectile.damage();
                });
            }
            !hit && !projectile.is_out_of_max_range()
        });

        body.structure_mut().for_each_voxel_mut(|_x, _y, _z, voxel: &mut Option<Voxel>| {
            if voxel.unwrap().life <= 0.0 {
                *voxel = None;
            }
        });

        window.clear();

        let projection_view_matrix = projection_matrix.clone() * camera.view_matrix();
        renderer.render_frame(&projection_view_matrix, &body, &projectiles, ghost_cube_position);

        window.update();
    }
}
