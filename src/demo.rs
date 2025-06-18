use crate::graphic::renderer::main_renderer::DemoMainRenderer;
use crate::graphic::windowing::window::Window;
use crate::maths::matrix::Mat4f;
use crate::maths::vector::Vect3f;
use crate::physics::body::Body;
use crate::voxels::structure::Structure;
use crate::voxels::voxel::Voxel;
use crate::voxels::voxel::VoxelID;
use crate::warfare::battle::Battle;

pub fn run_demo() {
    const WINDOW_WIDTH:u32 = 800;
    const WINDOW_HEIGHT:u32 = 600;

    let mut window = Window::create_window(WINDOW_WIDTH, WINDOW_HEIGHT, "Vorustious - Editor");
    let mut renderer = DemoMainRenderer::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32, 85.0_f32.to_radians(), 3.0, 1000.0);

    let mut battle = Battle::new();

    let camera_position = Vect3f::new([10.0, 0.0, 0.0]);
    let camera_target = Vect3f::new([0.0, 0.0, 0.0]);
    let camera_up = Vect3f::new([0.0, 0.0, 1.0]);
    let view_matrix = Mat4f::look_at(camera_position, camera_target, camera_up);

    let grey_voxel = Voxel{id: VoxelID::LightHull, life: 5.0};

    let block_struct = Structure::new(-1, 1, -1, 1, -1, 1, grey_voxel);
    let block_repere = Mat4f::translation(Vect3f::new([0.0, 3.0, 5.0]));
    let mut body = Body::new(block_struct, block_repere);
    body.set_velocity(Vect3f::new([0.0, 0.0, -3.0]));
    battle.add_inert_body(body);

    let stick_struct = Structure::new(0, 0, -4, 4, 0, 0, grey_voxel);
    let stick_repere = Mat4f::identity();
    let body = Body::new(stick_struct, stick_repere);
    battle.add_inert_body(body);

    while !window.should_close() {
        window.clear();

        battle.update(1.0 / 60.0);
        renderer.render_frame(view_matrix.clone(), battle.bodies());

        window.update();
    }

}
