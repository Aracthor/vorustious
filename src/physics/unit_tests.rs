use std::f32::consts::PI;

use super::body::Body;
use super::collision;
use crate::maths::matrix::Mat4f;
use crate::maths::vector::Vect3f;
use crate::maths::vector::Vect3i;
use crate::voxels::structure::Structure;
use crate::voxels::voxel::Voxel;
use crate::voxels::voxel::VoxelID;

const TEST_VOXEL: Voxel = Voxel{
    life: 2.0,
    id: VoxelID::LightHull,
};

const TEST_DEAD_VOXEL: Voxel = Voxel{
    life: 0.0,
    id: VoxelID::ShipCore,
};


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
fn body_cube_intersection_high_precision() {
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
    let result = collision::intersection_high_precision(&body_a, &body_b);
    assert!(result.len() == 1);
    assert!(result[0] == (Vect3i::new([1, 1, 1]), Vect3i::new([-1, -1, -1])));

    // Same, in the other side.
    let body_a = Body::new(structure_square.clone(), Mat4f::translation(Vect3f::new([-2.0, 3.0, -2.0])));
    let body_b = Body::new(structure_square.clone(), Mat4f::translation(Vect3f::new([-4.5, 0.5, -4.5])));
    let result = collision::intersection_high_precision(&body_a, &body_b);
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
    let body_a = Body::new(structure_square.clone(), Mat4f::identity());
    let body_b = Body::new(structure_square.clone(), Mat4f::translation(Vect3f::new([0.0, 0.0, 4.0])) * Mat4f::rotation_around_y(PI / 5.0) * Mat4f::rotation_around_x(PI / 4.0));
    let result = collision::intersection_high_precision(&body_a, &body_b);
    assert!(result.len() == 1);
    assert!(result[0] == (Vect3i::new([0, 0, 1]), Vect3i::new([1, -1, -1])));
}

#[test]
fn body_cube_intersection_low_precision() {
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
    let body_b = Body::new(structure_square.clone(), Mat4f::translation(Vect3f::new([2.4, 2.4, 2.4])));
    let result = collision::intersection_low_precision(&body_a, &body_b);
    assert!(result.len() == 1);
    assert!(result[0] == (Vect3i::new([1, 1, 1]), Vect3i::new([-1, -1, -1])));

    // Same, in the other side.
    let body_a = Body::new(structure_square.clone(), Mat4f::translation(Vect3f::new([-2.0, 3.0, -2.0])));
    let body_b = Body::new(structure_square.clone(), Mat4f::translation(Vect3f::new([-4.4, 0.6, -4.4])));
    let result = collision::intersection_low_precision(&body_a, &body_b);
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
    let body_b = Body::new(structure_square.clone(), Mat4f::translation(Vect3f::new([0.0, 0.0, 3.2])) * Mat4f::rotation_around_y(PI / 5.0) * Mat4f::rotation_around_x(PI / 4.0));
    let result = collision::intersection_low_precision(&body_a, &body_b);
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
    assert!(collision::intersection_high_precision(&body_a, &body_b).is_empty());
    assert!(collision::intersection_low_precision(&body_a, &body_b).is_empty());

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
    let body_b = Body::new(structure_h.clone(), Mat4f::translation(Vect3f::new([0.0, 0.0, 6.4])));
    let expected_result_1 = (Vect3i::new([-3, 0, 3]), Vect3i::new([-3, 0, -3]));
    let expected_result_2 = (Vect3i::new([3, 0, 3]), Vect3i::new([3, 0, -3]));
    let result = collision::intersection_high_precision(&body_a, &body_b);
    assert!(result.len() == 2);
    assert!(
        (result[0] == expected_result_1 && result[1] == expected_result_2) ||
        (result[0] == expected_result_2 && result[1] == expected_result_1));
    let result = collision::intersection_low_precision(&body_a, &body_b);
    assert!(result.len() == 2);
    assert!(
        (result[0] == expected_result_1 && result[1] == expected_result_2) ||
        (result[0] == expected_result_2 && result[1] == expected_result_1));
}

#[test]
fn elastic_collision_same_mass() {
    let restitution = 1.0;
    let structure = Structure::new(-1, 1, 0, 0, 0, 0, TEST_VOXEL);
    let mut body_a = Body::new(structure.clone(), Mat4f::translation(Vect3f::new([-1.2, 0.0, 0.0])));
    let mut body_b = Body::new(structure.clone(), Mat4f::translation(Vect3f::new([1.2, 0.0, 0.0])));

    body_a.set_velocity(Vect3f::new([1.0, 0.0, 0.0]));
    body_b.set_velocity(Vect3f::new([0.0, 0.0, 0.0]));
    collision::apply_collision_if_any(&mut body_a, &mut body_b, restitution);
    assert!(body_a.velocity() == Vect3f::new([0.0, 0.0, 0.0]));
    assert!(body_b.velocity() == Vect3f::new([1.0, 0.0, 0.0]));

    body_a.set_velocity(Vect3f::new([1.0, 0.0, 0.0]));
    body_b.set_velocity(Vect3f::new([-1.0, 0.0, 0.0]));
    collision::apply_collision_if_any(&mut body_a, &mut body_b, restitution);
    assert!(body_a.velocity() == Vect3f::new([-1.0, 0.0, 0.0]));
    assert!(body_b.velocity() == Vect3f::new([1.0, 0.0, 0.0]));
}

#[test]
fn perfectly_inelastic_collision_same_mass() {
    let restitution = 0.0;
    let structure = Structure::new(-1, 1, 0, 0, 0, 0, TEST_VOXEL);
    let mut body_a = Body::new(structure.clone(), Mat4f::translation(Vect3f::new([-1.2, 0.0, 0.0])));
    let mut body_b = Body::new(structure.clone(), Mat4f::translation(Vect3f::new([1.2, 0.0, 0.0])));

    body_a.set_velocity(Vect3f::new([1.0, 0.0, 0.0]));
    body_b.set_velocity(Vect3f::new([0.0, 0.0, 0.0]));
    collision::apply_collision_if_any(&mut body_a, &mut body_b, restitution);
    assert!(body_a.velocity() == Vect3f::new([0.5, 0.0, 0.0]));
    assert!(body_b.velocity() == Vect3f::new([0.5, 0.0, 0.0]));

    body_a.set_velocity(Vect3f::new([1.0, 0.0, 0.0]));
    body_b.set_velocity(Vect3f::new([-1.0, 0.0, 0.0]));
    collision::apply_collision_if_any(&mut body_a, &mut body_b, restitution);
    assert!(body_a.velocity() == Vect3f::new([0.0, 0.0, 0.0]));
    assert!(body_b.velocity() == Vect3f::new([0.0, 0.0, 0.0]));
}


#[test]
fn elastic_collision_different_mass() {
    let restitution = 1.0;
    let structure_a = Structure::new(-4, 1, 0, 0, 0, 0, TEST_VOXEL);
    let structure_b = Structure::new(-1, 1, 0, 0, 0, 0, TEST_VOXEL);
    let mut body_a = Body::new(structure_a, Mat4f::translation(Vect3f::new([-1.2, 0.0, 0.0])));
    let mut body_b = Body::new(structure_b, Mat4f::translation(Vect3f::new([1.2, 0.0, 0.0])));

    body_a.set_velocity(Vect3f::new([1.0, 0.0, 0.0]));
    body_b.set_velocity(Vect3f::new([0.0, 0.0, 0.0]));
    collision::apply_collision_if_any(&mut body_a, &mut body_b, restitution);
    assert!(body_a.velocity() == Vect3f::new([1.0 / 3.0, 0.0, 0.0]));
    assert!(body_b.velocity() == Vect3f::new([4.0 / 3.0, 0.0, 0.0]));

    body_a.set_velocity(Vect3f::new([1.0, 0.0, 0.0]));
    body_b.set_velocity(Vect3f::new([-1.0, 0.0, 0.0]));
    collision::apply_collision_if_any(&mut body_a, &mut body_b, restitution);
    assert!(body_a.velocity() == Vect3f::new([-1.0 / 3.0, 0.0, 0.0]));
    assert!(body_b.velocity() == Vect3f::new([5.0 / 3.0, 0.0, 0.0]));
}

#[test]
fn perfectly_inelastic_collision_different_mass() {
    let restitution = 0.0;
    let structure_a = Structure::new(-4, 1, 0, 0, 0, 0, TEST_VOXEL);
    let structure_b = Structure::new(-1, 1, 0, 0, 0, 0, TEST_VOXEL);
    let mut body_a = Body::new(structure_a, Mat4f::translation(Vect3f::new([-1.2, 0.0, 0.0])));
    let mut body_b = Body::new(structure_b, Mat4f::translation(Vect3f::new([1.2, 0.0, 0.0])));

    body_a.set_velocity(Vect3f::new([1.0, 0.0, 0.0]));
    body_b.set_velocity(Vect3f::new([0.0, 0.0, 0.0]));
    collision::apply_collision_if_any(&mut body_a, &mut body_b, restitution);
    assert!(body_a.velocity() == Vect3f::new([2.0 / 3.0, 0.0, 0.0]));
    assert!(body_b.velocity() == Vect3f::new([2.0 / 3.0, 0.0, 0.0]));

    body_a.set_velocity(Vect3f::new([1.0, 0.0, 0.0]));
    body_b.set_velocity(Vect3f::new([-1.0, 0.0, 0.0]));
    collision::apply_collision_if_any(&mut body_a, &mut body_b, restitution);
    assert!(body_a.velocity() == Vect3f::new([1.0 / 3.0, 0.0, 0.0]));
    assert!(body_b.velocity() == Vect3f::new([1.0 / 3.0, 0.0, 0.0]));
}
