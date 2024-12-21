mod graphic;
mod maths;
mod physics;
mod voxels;
mod warfare;

mod editor;
mod profiler;

use graphic::renderer::main_renderer::BattleRenderer;
use graphic::windowing::event_handler::EventHandler;
use graphic::windowing::window::Window;
use graphic::windowing::event_handler::Key;
use graphic::windowing::event_handler::MouseButton;
use maths::matrix::Mat4f;
use maths::vector::Vect3f;
use physics::body::Body;
use voxels::structure::Structure;
use voxels::catalog::VoxelCatalog;
use warfare::battle::Battle;
use warfare::ship::Ship;
use warfare::weapon::Weapon;

fn update_player_body_from_events(player_body: &mut Body, event_handler: &EventHandler) {
    let forward = player_body.repere().forward();
    let right = player_body.repere().right();
    if event_handler.is_key_pressed(Key::W) {
        player_body.add_to_velocity(forward);
    }
    if event_handler.is_key_pressed(Key::S) {
        player_body.add_to_velocity(-forward);
    }
    if event_handler.is_key_pressed(Key::D) {
        player_body.add_to_velocity(right);
    }
    if event_handler.is_key_pressed(Key::A) {
        player_body.add_to_velocity(-right);
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
    player_body.scale_velocity(0.95);
}

fn run_battle() {
    const WINDOW_WIDTH:u32 = 800;
    const WINDOW_HEIGHT:u32 = 600;

    let mut window = Window::create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "Vorustious");
    let mut renderer = BattleRenderer::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32, 85.0_f32.to_radians(), 3.0, 1000.0);

    let mut battle = Battle::new();

    let voxel_catalog = VoxelCatalog::create();
    let tie_fighter_structure = Structure::read_from_file(&voxel_catalog, "structures/tie.vors");
    battle.add_inert_body(Body::new(tie_fighter_structure, Mat4f::identity()));

    let player_ship = {
        let player_repere = Mat4f::translation(Vect3f::new([-20.0, 0.0, 0.0]));
        let player_structure = Structure::read_from_file(&voxel_catalog, "structures/x_wing.vors");
        let player_body = Body::new(player_structure, player_repere);
        let mut ship = Ship::new(player_body);
        ship.add_weapon(Vect3f::new([4.0, -4.0, -3.0]), Weapon::new(0.5, 1.0, 100.0, 1000.0));
        ship.add_weapon(Vect3f::new([4.0, 4.0, -3.0]), Weapon::new(0.5, 1.0, 100.0, 1000.0));
        ship.add_weapon(Vect3f::new([4.0, -4.0, 3.0]), Weapon::new(0.5, 1.0, 100.0, 1000.0));
        ship.add_weapon(Vect3f::new([4.0, 4.0, 3.0]), Weapon::new(0.5, 1.0, 100.0, 1000.0));
        ship
    };
    battle.set_player_ship(player_ship);

    let mut pause = false;

    let tick_elapsed_time = 1.0 / 60.0; // Rather than real elapsed time in order to keep determinism.
    while !window.should_close() {

        update_player_body_from_events(battle.player_ship_mut().unwrap().body_mut(), window.event_handler());

        if window.event_handler().is_mouse_button_pressed(MouseButton::Left) {
            let projectiles = battle.player_ship_mut().unwrap().shoot();
            battle.add_projectiles(projectiles);
        }

        if window.event_handler().is_key_just_pressed(Key::F1) {
            renderer.toggle_debug_bodies();
        }
        if window.event_handler().is_key_just_pressed(Key::F2) {
            renderer.toggle_debug_boxes();
        }
        if window.event_handler().is_key_just_pressed(Key::F3) {
            renderer.toggle_reticle_mode();
        }
        if window.event_handler().is_key_just_pressed(Key::F4) {
            renderer.toggle_gizmo();
        }
        if window.event_handler().is_key_just_pressed(Key::F8) {
            renderer.toggle_octtree();
        }

        if window.event_handler().is_key_just_pressed(Key::P) {
            pause = !pause;
        }

        if !pause {
            battle.update(tick_elapsed_time);
        }

        window.clear();
        let view_matrix = {
            let player_body = battle.player_ship().unwrap();
            let body_repere_without_translation = Mat4f::translation(-player_body.repere().position()) * player_body.repere().clone();
            let position = player_body.repere().position() + body_repere_without_translation * Vect3f::new([-10.0, 0.0, 5.0]);
            let forward = player_body.repere().forward();
            let up = player_body.repere().up();
            Mat4f::look_at(position, position + forward.normalize(), up)
        };
        renderer.render_frame(view_matrix, battle.bodies(), &battle.projectiles(), battle.player_ship().unwrap());
        window.update();
    }

}

fn main() {
    let first_arg = std::env::args().nth(1);
    if first_arg.is_some() && first_arg.as_ref().unwrap() == "editor" {
        editor::run_editor();
    } else if first_arg.is_some() && first_arg.as_ref().unwrap() == "profile" {
        profiler::run_profiler();
    }
    else {
        run_battle();
    }
}
