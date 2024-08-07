use std::f32::consts::PI;

use crate::maths::matrix::Mat4f;
use crate::maths::vector::Vect3f;
use crate::maths::vector::Vect3i;
use crate::voxels::structure::Structure;
use crate::voxels::voxel::Voxel;
use crate::voxels::voxel::VoxelID;
use super::body::Body;
use super::battle::Battle;
use super::projectile::Projectile;

const TEST_VOXEL: Voxel = Voxel{
    life: 2.0,
    id: VoxelID::LightHull,
};

const TEST_DEAD_VOXEL: Voxel = Voxel{
    life: 0.0,
    id: VoxelID::ShipCore,
};

#[test]
fn projectile_movement() {
    let initial_position = Vect3f::new([0.0, 0.0, 2.0]);
    let movement = Vect3f::new([10.0, -20.0, 0.0]);
    let mut projectile = Projectile::new(initial_position, movement, 1.0, f32::MAX);
    projectile.moove(2.0);
    projectile.moove(2.0);
    projectile.moove(2.0);
    let expected_position = initial_position + movement * 6.0;
    assert!(projectile.position() == expected_position);
}

#[test]
fn projectile_max_distance() {
    let initial_position = Vect3f::zero();
    let movement = Vect3f::new([10.0, 0.0, 0.0]);
    let max_distance = 25.0;
    let mut projectile = Projectile::new(initial_position, movement, 1.0, max_distance);
    projectile.moove(1.0);
    assert!(!projectile.is_out_of_max_range());
    projectile.moove(1.0);
    assert!(!projectile.is_out_of_max_range());
    projectile.moove(1.0);
    assert!(projectile.is_out_of_max_range());
}

#[test]
fn projectile_damage() {
    let mut battle = Battle::new();
    let structure = Structure::new(-1, 1, -1, 1, -1, 1, TEST_VOXEL);
    battle.add_inert_body(Body::new(structure, Mat4f::identity()));

    let initial_position = Vect3f::new([-10.0, 1.0, 1.0]);
    let movement = Vect3f::new([10.0, 0.0, 0.0]);
    battle.add_projectile(Projectile::new(initial_position, movement, 1.0, f32::MAX));
    battle.update(1.0);
    assert!(battle.bodies()[0].structure().get_voxel(Vect3i::new([-1, 1, 1])).unwrap().life == 1.0);

    battle.add_projectile(Projectile::new(initial_position, movement, 1.0, f32::MAX));
    battle.update(1.0);
    assert!(battle.bodies()[0].structure().get_voxel(Vect3i::new([-1, 1, 1])).is_none());
}

#[test]
fn projectile_damage_on_moving_body() {
    let mut battle = Battle::new();
    let structure = Structure::new(-1, 1, -1, 1, -1, 1, TEST_VOXEL);
    let mut expected_structure = structure.clone();
    let mut body = Body::new(structure, Mat4f::identity());
    body.add_to_movement(Vect3f::new([0.0, 0.4, 0.0]));
    battle.add_inert_body(body);

    let initial_position = Vect3f::new([-10.0, 0.0, 0.0]);
    let movement = Vect3f::new([10.0, 0.0, 0.0]);
    battle.add_projectile(Projectile::new(initial_position, movement, 1.0, f32::MAX));
    battle.update(1.0);
    expected_structure.set_voxel(-1, 0, 0, Some(Voxel{life: 1.0, id: VoxelID::LightHull}));
    assert!(battle.bodies()[0].structure().clone() == expected_structure);

    battle.add_projectile(Projectile::new(initial_position, movement, 1.0, f32::MAX));
    battle.update(1.0);
    expected_structure.set_voxel(-1, -1, 0, Some(Voxel{life: 1.0, id: VoxelID::LightHull}));
    assert!(battle.bodies()[0].structure().clone() == expected_structure);

    battle.add_projectile(Projectile::new(initial_position, movement, 1.0, f32::MAX));
    battle.update(1.0);
    expected_structure.set_voxel(-1, -1, 0, None);
    assert!(battle.bodies()[0].structure().clone() == expected_structure);

    battle.add_projectile(Projectile::new(initial_position, movement, 1.0, f32::MAX));
    battle.update(1.0);
    // Nothing new, the projectile should have missed.
    assert!(battle.bodies()[0].structure().clone() == expected_structure);
}

#[test]
fn body_cut_in_half() {
    let mut structure = {
        let mut structure = Structure::new(-2, 2, 0, 0, 0, 0, TEST_VOXEL);
        structure.add_voxel(Vect3i::new([2, 1, 0]), TEST_VOXEL);
        structure.add_voxel(Vect3i::new([2, 1, 1]), TEST_VOXEL);
        structure.add_voxel(Vect3i::new([2, 0, 1]), TEST_VOXEL);
        structure.add_voxel(Vect3i::new([2, -1, 1]), TEST_VOXEL);
        structure.add_voxel(Vect3i::new([2, -1, 0]), TEST_VOXEL);
        structure.add_voxel(Vect3i::new([2, -1, -1]), TEST_VOXEL);
        structure.add_voxel(Vect3i::new([2, 0, -1]), TEST_VOXEL);
        structure.add_voxel(Vect3i::new([2, 1, -1]), TEST_VOXEL);
        structure
    };
    let expected_remaining_structure = {
        let mut structure = structure.clone();
        structure.set_voxel(2, 1, 1, None);
        structure.set_voxel(2, 1, 0, None);
        structure.set_voxel(2, 1, -1, None);
        structure.set_voxel(2, 0, 1, None);
        structure.set_voxel(2, 0, 0, None);
        structure.set_voxel(2, 0, -1, None);
        structure.set_voxel(2, -1, 1, None);
        structure.set_voxel(2, -1, 0, None);
        structure.set_voxel(2, -1, -1, None);
        structure.set_voxel(1, 0, 0, None);
        structure.recalculate_box();
        structure
    };
    let expected_new_structure = Structure::new(0, 0, -1, 1, -1, 1, TEST_VOXEL);

    structure.set_voxel(1, 0, 0, Some(Voxel{life: 0.0, id: VoxelID::LightHull}));
    let mut body = Body::new(structure, Mat4f::identity());
    let new_bodies = body.update_dead_voxels();

    assert!(body.structure().clone() == expected_remaining_structure);
    assert!(new_bodies.len() == 1);
    assert!(new_bodies[0].structure().clone() == expected_new_structure);
}

#[test]
fn body_cut_in_half_with_same_center() {
    let mut structure = Structure::new(-2, 2, -2, 2, 0, 0, TEST_VOXEL);
    structure.set_voxel(-1, -1, 0, Some(TEST_DEAD_VOXEL));
    structure.set_voxel(-1, 0, 0, Some(TEST_DEAD_VOXEL));
    structure.set_voxel(-1, 1, 0, Some(TEST_DEAD_VOXEL));
    structure.set_voxel(0, 1, 0, Some(TEST_DEAD_VOXEL));
    structure.set_voxel(1, 1, 0, Some(TEST_DEAD_VOXEL));
    structure.set_voxel(1, 0, 0, Some(TEST_DEAD_VOXEL));
    structure.set_voxel(1, -1, 0, Some(TEST_DEAD_VOXEL));
    structure.set_voxel(0, -1, 0, Some(TEST_DEAD_VOXEL));

    let expected_new_structure = {
        let mut new_structure = structure.clone();
        new_structure.set_voxel(-1, -1, 0, None);
        new_structure.set_voxel(-1, 0, 0, None);
        new_structure.set_voxel(-1, 1, 0, None);
        new_structure.set_voxel(0, 1, 0, None);
        new_structure.set_voxel(1, 1, 0, None);
        new_structure.set_voxel(1, 0, 0, None);
        new_structure.set_voxel(1, -1, 0, None);
        new_structure.set_voxel(0, -1, 0, None);
        new_structure.set_voxel(0, 0, 0, None);
        new_structure
    };

    let mut body = Body::new(structure, Mat4f::identity());
    let new_bodies = body.update_dead_voxels();

    assert!(body.structure().clone() == Structure::new(0, 0, 0, 0, 0, 0, TEST_VOXEL));
    assert!(new_bodies.len() == 1);
    assert!(new_bodies[0].structure().clone() == expected_new_structure);
}

#[test]
fn body_cut_in_half_with_father_body_becoming_empty() {
    let mut structure = Structure::new(-1, 1, 0, 0, 0, 0, TEST_VOXEL);
    structure.set_voxel(0, 0, 0, Some(TEST_DEAD_VOXEL));

    let mut body = Body::new(structure, Mat4f::identity());
    let new_bodies = body.update_dead_voxels();

    let expected_structure = {
        let mut structure = Structure::new(0, 0, 0, 0, 0, 0, TEST_VOXEL);
        structure.set_voxel(0, 0, 0, None);
        structure
    };

    assert!(body.structure().clone() == expected_structure);
    assert!(new_bodies.len() == 2);
}

#[test]
fn body_cube_intersection() {
    // Cubes with same axis, with a corner voxel half-mingled.
    //        +-------+
    //        |       |
    //        |       |
    //        |       |
    // +------++------+
    // |      ++
    // |       |
    // |       |
    // +-------+
    let structure_square = Structure::new(-1, 1, -1, 1, -1, 1, TEST_VOXEL);
    let body_a = Body::new(structure_square.clone(), Mat4f::identity());
    let body_b = Body::new(structure_square.clone(), Mat4f::translation(Vect3f::new([2.5, 2.5, 2.5])));
    let result = Body::intersection(&body_a, &body_b);
    assert!(result.len() == 1);
    assert!(result[0] == (Vect3i::new([1, 1, 1]), Vect3i::new([-1, -1, -1])));

    // Same, in the other side.
    let body_a = Body::new(structure_square.clone(), Mat4f::translation(Vect3f::new([-2.0, 3.0, -2.0])));
    let body_b = Body::new(structure_square.clone(), Mat4f::translation(Vect3f::new([-4.5, 0.5, -4.5])));
    let result = Body::intersection(&body_a, &body_b);
    assert!(result.len() == 1);
    assert!(result[0] == (Vect3i::new([-1, -1, -1]), Vect3i::new([1, 1, 1])));

    // A cube rotated with a corner on the top of another.
    //     +
    //    / \
    //  /     \
    // +       +
    //  \     /
    //    \ /
    // +---+---+
    // |       |
    // |       |
    // |       |
    // +-------+
    let body_a = Body::new(structure_square.clone(), Mat4f::translation(Vect3f::new([0.0, 0.0, 0.0])));
    let body_b = Body::new(structure_square.clone(), Mat4f::translation(Vect3f::new([0.0, 0.0, 4.0])) * Mat4f::rotation_around_y(PI / 5.0) * Mat4f::rotation_around_x(PI / 4.0));
    let result = Body::intersection(&body_a, &body_b);
    assert!(result.len() == 1);
    assert!(result[0] == (Vect3i::new([0, 0, 1]), Vect3i::new([1, -1, -1])));
}

#[test]
fn body_heavy_intersection() {
    let structure_h = {
        let mut structure = Structure::new(0, 0, 0, 0, 0, 0, TEST_VOXEL);
        structure.add_voxel(Vect3i::new([1, 0, 0]), TEST_VOXEL);
        structure.add_voxel(Vect3i::new([2, 0, 0]), TEST_VOXEL);
        structure.add_voxel(Vect3i::new([3, 0, 0]), TEST_VOXEL);
        structure.add_voxel(Vect3i::new([-1, 0, 0]), TEST_VOXEL);
        structure.add_voxel(Vect3i::new([-2, 0, 0]), TEST_VOXEL);
        structure.add_voxel(Vect3i::new([-3, 0, 0]), TEST_VOXEL);
        structure.add_voxel(Vect3i::new([-3, 0, 1]), TEST_VOXEL);
        structure.add_voxel(Vect3i::new([-3, 0, -1]), TEST_VOXEL);
        structure.add_voxel(Vect3i::new([-3, 0, 2]), TEST_VOXEL);
        structure.add_voxel(Vect3i::new([-3, 0, -2]), TEST_VOXEL);
        structure.add_voxel(Vect3i::new([-3, 0, 3]), TEST_VOXEL);
        structure.add_voxel(Vect3i::new([-3, 0, -3]), TEST_VOXEL);
        structure.add_voxel(Vect3i::new([3, 0, 1]), TEST_VOXEL);
        structure.add_voxel(Vect3i::new([3, 0, -1]), TEST_VOXEL);
        structure.add_voxel(Vect3i::new([3, 0, 2]), TEST_VOXEL);
        structure.add_voxel(Vect3i::new([3, 0, -2]), TEST_VOXEL);
        structure.add_voxel(Vect3i::new([3, 0, 3]), TEST_VOXEL);
        structure.add_voxel(Vect3i::new([3, 0, -3]), TEST_VOXEL);
        structure
    };

    // Two structure where their global boxes heavily collide, but their voxels don't.
    //     |     |
    //     |     |
    //     +-----+
    //     |     |
    //  |  |  |  |
    //  |     |
    //  +-----+
    //  |     |
    //  |     |
    let body_a = Body::new(structure_h.clone(), Mat4f::identity());
    let body_b = Body::new(structure_h.clone(), Mat4f::translation(Vect3f::new([1.5, 0.0, 4.5])));
    assert!(Body::intersection(&body_a, &body_b).is_empty());

    // Two structure with two intersections.
    //  |     |
    //  |     |
    //  +-----+
    //  |     |
    //  X     X
    //  |     |
    //  +-----+
    //  |     |
    //  |     |
    let body_a = Body::new(structure_h.clone(), Mat4f::identity());
    let body_b = Body::new(structure_h.clone(), Mat4f::translation(Vect3f::new([0.0, 0.0, 6.5])));
    let result = Body::intersection(&body_a, &body_b);
    assert!(result.len() == 2);
    let expected_result_1 = (Vect3i::new([-3, 0, 3]), Vect3i::new([-3, 0, -3]));
    let expected_result_2 = (Vect3i::new([3, 0, 3]), Vect3i::new([3, 0, -3]));
    assert!(
        (result[0] == expected_result_1 && result[1] == expected_result_2) ||
        (result[0] == expected_result_2 && result[1] == expected_result_1));
}


#[test]
fn battle_forget_empty_bodies() {
    let structure = Structure::new(-1, 1, 0, 0, 0, 0, TEST_DEAD_VOXEL);
    let body = Body::new(structure, Mat4f::identity());
    let mut battle = Battle::new();
    battle.add_inert_body(body);

    battle.update(1.0);
    assert!(battle.bodies().is_empty());
}
