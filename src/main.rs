mod graphic;
mod maths;
mod voxels;

mod editor;
mod projectile;
mod weapon;

use editor::Editor;
use graphic::renderer::Renderer;
use graphic::camera::Camera;
use graphic::windowing::window::Window;
use graphic::windowing::event_handler::MouseButton;
use maths::segment::Segm3f;
use projectile::Projectile;
use voxels::body::Body;
use voxels::voxel::Voxel;
use weapon::Weapon;

fn main() {
    const WINDOW_WIDTH:u32 = 800;
    const WINDOW_HEIGHT:u32 = 600;

    let mut window = Window::create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "Vorustious");
    let mut renderer = Renderer::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);

    let mut camera = Camera::new();
    let mut editor = Editor::new();

    let mut projectiles: Vec<Projectile> = vec![];
    let mut weapon = Weapon::new(1.0 / 10.0, 0.5, 20.0);

    while !window.should_close() {
        camera.update_from_events(&window.event_handler());
        editor.update_from_events(&camera, &window.event_handler());

        if window.event_handler().is_mouse_button_pressed(MouseButton::Left) {
            let projectile_start = camera.position();
            let projectile_direction = camera.forward();
            let projectile = weapon.shoot(projectile_start, projectile_direction);
            if projectile.is_some() {
                projectiles.push(projectile.unwrap());
            }
        }

        projectiles.retain_mut(|projectile| {
            let segment_start = projectile.position();
            projectile.moove(1.0);
            let segment_end = projectile.position();
            let mut hit = false;
            if !projectile.is_out_of_max_range() {
                let segment = Segm3f::new(segment_start, segment_end);
                hit = editor.structure.for_first_voxel_in_segment(segment, |voxel: &mut Option<Voxel>| {
                    voxel.as_mut().unwrap().life -= projectile.damage();
                });
            }
            !hit && !projectile.is_out_of_max_range()
        });

        editor.structure.for_each_voxel_mut(|_x, _y, _z, voxel: &mut Option<Voxel>| {
            if voxel.unwrap().life <= 0.0 {
                *voxel = None;
            }
        });

        window.clear();

        let body = Body::new(editor.structure.clone());
        renderer.render_frame(camera.view_matrix(), &body, &projectiles, &editor);

        window.update();
    }
}
