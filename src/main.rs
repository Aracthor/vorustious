mod graphic;
mod maths;
mod voxels;
mod warfare;

mod editor;

use editor::Editor;
use graphic::camera::Camera;
use graphic::renderer::Renderer;
use graphic::windowing::event_handler::EventHandler;
use graphic::windowing::window::Window;
use graphic::windowing::event_handler::Key;
use graphic::windowing::event_handler::MouseButton;
use maths::matrix::Mat4f;
use maths::vector::Vect3f;
use voxels::structure::Structure;
use voxels::catalog::VoxelCatalog;
use warfare::battle::Battle;
use warfare::body::Body;
use warfare::weapon::Weapon;

fn run_editor(window: &mut Window, renderer: &mut Renderer) {
    let mut camera = Camera::new();
    let mut editor = Editor::new();

    while !window.should_close() {
        camera.update_from_events(&window.event_handler());
        editor.update_from_events(&camera, &window.event_handler());

        window.clear();

        let bodies = vec![Body::new(editor.structure.clone(), Mat4f::identity())];
        renderer.render_frame(camera.view_matrix(), bodies.iter().collect(), &vec![], Some(&editor));

        window.update();
    }
}

fn update_player_body_from_events(player_body: &mut Body, event_handler: &EventHandler) {
    let forward = player_body.repere().forward();
    let right = player_body.repere().right();
    if event_handler.is_key_pressed(Key::W) {
        player_body.add_to_movement(forward);
    }
    if event_handler.is_key_pressed(Key::S) {
        player_body.add_to_movement(-forward);
    }
    if event_handler.is_key_pressed(Key::D) {
        player_body.add_to_movement(-right);
    }
    if event_handler.is_key_pressed(Key::A) {
        player_body.add_to_movement(right);
    }

    if event_handler.is_key_pressed(Key::E) {
        player_body.add_roll_rotation(0.1);
    }
    if event_handler.is_key_pressed(Key::Q) {
        player_body.add_roll_rotation(-0.1);
    }
    let cursor_movement = event_handler.cursor_movement();
    player_body.add_yaw_rotation(-cursor_movement.0 as f32 / 100.0);
    player_body.add_pitch_rotation(cursor_movement.1 as f32 / 100.0);

    player_body.scale_rotation(0.9);
    player_body.scale_movement(0.95);
}

fn run_battle(window: &mut Window, renderer: &mut Renderer) {
    let mut battle = Battle::new();

    let voxel_catalog = VoxelCatalog::create();
    let structure_file_content = std::fs::read_to_string("structures/tie.vors").expect(&format!("Unable to read structures/tie.vors"));
    let tie_fighter_structure = Structure::deserialize(&voxel_catalog, &structure_file_content);
    battle.add_inert_body(Body::new(tie_fighter_structure.clone(), Mat4f::identity()));

    let player_repere = Mat4f::translation(Vect3f::new([-20.0, 0.0, 0.0]));
    battle.set_player_body(Body::new(tie_fighter_structure.clone(), player_repere));

    let mut pause = false;

    let tick_elapsed_time = 1.0 / 60.0; // Rather than real elapsed time in order to keep determinism.
    while !window.should_close() {

        update_player_body_from_events(battle.player_body_mut().unwrap(), window.event_handler());

        if window.event_handler().is_key_just_pressed(Key::P) {
            pause = !pause;
        }

        if !pause {
            battle.update(tick_elapsed_time);
        }

        window.clear();
        let view_matrix = {
            let player_body = battle.player_body().unwrap();
            let position = player_body.repere().position();
            let forward = player_body.repere().forward();
            let up = player_body.repere().up();
            Mat4f::look_at(position, position + forward.normalize(), up)
        };
        renderer.render_frame(view_matrix, battle.bodies(), &battle.projectiles(), None);
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
