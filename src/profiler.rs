use std::f32::consts::PI;
use std::time::Duration;
use std::time::Instant;

use crate::maths::boxes::Box3f;
use crate::maths::matrix::Mat4f;
use crate::maths::vector::Vect3f;
use crate::physics::body::Body;
use crate::physics::collision;
use crate::voxels::structure::Structure;
use crate::voxels::catalog::VoxelCatalog;

fn profile<F: Fn()>(f: F, name: &str) -> Duration {
    const COUNT: u32 = 10;

    let start = Instant::now();
    for _ in 0..COUNT {
        f();
    }
    let elapsed_time = start.elapsed() / COUNT;
    println!("{}: {} us.", name, elapsed_time.as_micros());
    elapsed_time
}

pub fn run_profiler() {
    let voxel_catalog = VoxelCatalog::create();
    let tie_fighter_structure = Structure::read_from_file(&voxel_catalog, "structures/tie.vors");

    assert!(tie_fighter_structure.get_box() == Box3f::from_min_max(Vect3f::new([-6.5, -8.5, -9.5]), Vect3f::new([6.5, 8.5, 9.5])));
    let fighter_a = Body::new(tie_fighter_structure.clone(), Mat4f::identity());
    let fighter_b = Body::new(tie_fighter_structure.clone(), Mat4f::translation(Vect3f::new([0.0, 5.0, 13.0])));
    let fighter_c = Body::new(tie_fighter_structure.clone(), Mat4f::translation(Vect3f::new([0.0, 8.5, 0.0])));
    let repere_d = Mat4f::translation(Vect3f::new([5.5, 8.5, 18.5])) * Mat4f::rotation_around_z(PI / 2.0);
    let fighter_d = Body::new(tie_fighter_structure.clone(), repere_d);

    let mut total_time = Duration::ZERO;
    total_time += profile(|| { collision::intersection(&fighter_a, &fighter_b); }, "fighters crossing without intersection");
    total_time += profile(|| { collision::intersection(&fighter_a, &fighter_c); }, "fighters side-by-side with a lot of intersections");
    total_time += profile(|| { collision::intersection(&fighter_a, &fighter_a); }, "fighters exactly at the same position");
    total_time += profile(|| { collision::intersection(&fighter_a, &fighter_d); }, "fighters slightly colliding");
    println!("Total time: {} us.", total_time.as_micros());
}
