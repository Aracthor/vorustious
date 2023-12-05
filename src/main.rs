mod graphic;
mod maths;
mod voxels;
mod warfare;

mod editor;

use editor::Editor;
use graphic::renderer::Renderer;
use graphic::camera::Camera;
use graphic::windowing::window::Window;
use graphic::windowing::event_handler::MouseButton;
use maths::matrix::Mat4f;
use voxels::body::Body;
use voxels::structure::Structure;
use voxels::catalog::VoxelCatalog;
use warfare::battle::Battle;
use warfare::weapon::Weapon;

fn run_editor(window: &mut Window, renderer: &mut Renderer) {
    let mut camera = Camera::new();
    let mut editor = Editor::new();

    while !window.should_close() {
        camera.update_from_events(&window.event_handler());
        editor.update_from_events(&camera, &window.event_handler());

        window.clear();

        let bodies = vec![Body::new(editor.structure.clone(), Mat4f::identity())];
        renderer.render_frame(camera.view_matrix(), &bodies, &vec![], Some(&editor));

        window.update();
    }
}

fn run_battle(window: &mut Window, renderer: &mut Renderer) {
    let mut camera = Camera::new();
    let mut battle = Battle::new();

    let voxel_catalog = VoxelCatalog::create();
    let structure_file_content = std::fs::read_to_string("structures/proto.vors").expect(&format!("Unable to read structures/proto.vors"));
    battle.add_body(Body::new(Structure::deserialize(&voxel_catalog, &structure_file_content), Mat4f::identity()));
    let mut weapon = Weapon::new(1.0 / 10.0, 0.5, 50.0, 20.0);

    let tick_elapsed_time = 1.0 / 60.0; // Rather than real elapsed time in order to keep determinism.
    while !window.should_close() {
        camera.update_from_events(&window.event_handler());

        if window.event_handler().is_mouse_button_pressed(MouseButton::Left) {
            let projectile_start = camera.position();
            let projectile_direction = camera.forward();
            let projectile = weapon.shoot(projectile_start, projectile_direction);
            if projectile.is_some() {
                battle.add_projectile(projectile.unwrap());
            }
        }

        battle.update(tick_elapsed_time);

        window.clear();
        renderer.render_frame(camera.view_matrix(), &battle.bodies(), &battle.projectiles(), None);
        window.update();
    }

}

fn main() {
    const WINDOW_WIDTH:u32 = 800;
    const WINDOW_HEIGHT:u32 = 600;

    let mut window = Window::create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "Vorustious");
    let mut renderer = Renderer::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);

    let first_arg = std::env::args().nth(1);
    if first_arg.is_some() && first_arg.unwrap() == "editor" {
        run_editor(&mut window, &mut renderer);
    }
    else {
        run_battle(&mut window, &mut renderer);
    }
}
