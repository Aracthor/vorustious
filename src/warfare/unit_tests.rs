use crate::maths::matrix::Mat4f;
use crate::maths::vector::Vect3f;
use crate::maths::vector::Vect3i;
use crate::physics::body::Body;
use crate::voxels::structure::Structure;
use crate::voxels::voxel::Voxel;
use crate::voxels::voxel::VoxelID;
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
fn projectile_velocity() {
    let initial_position = Vect3f::new([0.0, 0.0, 2.0]);
    let velocity = Vect3f::new([10.0, -20.0, 0.0]);
    let mut projectile = Projectile::new(initial_position, velocity, 1.0, f32::MAX);
    projectile.moove(2.0);
    projectile.moove(2.0);
    projectile.moove(2.0);
    let expected_position = initial_position + velocity * 6.0;
    assert!(projectile.position() == expected_position);
}

#[test]
fn projectile_max_distance() {
    let initial_position = Vect3f::zero();
    let velocity = Vect3f::new([10.0, 0.0, 0.0]);
    let max_distance = 25.0;
    let mut projectile = Projectile::new(initial_position, velocity, 1.0, max_distance);
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
    let velocity = Vect3f::new([10.0, 0.0, 0.0]);
    battle.add_projectile(Projectile::new(initial_position, velocity, 1.0, f32::MAX));
    battle.update(1.0);
    assert!(battle.bodies()[0].structure().get_voxel(Vect3i::new([-1, 1, 1])).unwrap().life == 1.0);

    battle.add_projectile(Projectile::new(initial_position, velocity, 1.0, f32::MAX));
    battle.update(1.0);
    assert!(battle.bodies()[0].structure().get_voxel(Vect3i::new([-1, 1, 1])).is_none());
}

#[test]
fn projectile_damage_on_moving_body() {
    let mut battle = Battle::new();
    let structure = Structure::new(-1, 1, -1, 1, -1, 1, TEST_VOXEL);
    let mut expected_structure = structure.clone();
    let mut body = Body::new(structure, Mat4f::identity());
    body.add_to_velocity(Vect3f::new([0.0, 0.4, 0.0]));
    battle.add_inert_body(body);

    let initial_position = Vect3f::new([-10.0, 0.0, 0.0]);
    let velocity = Vect3f::new([10.0, 0.0, 0.0]);
    battle.add_projectile(Projectile::new(initial_position, velocity, 1.0, f32::MAX));
    battle.update(1.0);
    expected_structure.set_voxel(-1, 0, 0, Some(Voxel{life: 1.0, id: VoxelID::LightHull}));
    assert!(battle.bodies()[0].structure().clone() == expected_structure);

    battle.add_projectile(Projectile::new(initial_position, velocity, 1.0, f32::MAX));
    battle.update(1.0);
    expected_structure.set_voxel(-1, -1, 0, Some(Voxel{life: 1.0, id: VoxelID::LightHull}));
    assert!(battle.bodies()[0].structure().clone() == expected_structure);

    battle.add_projectile(Projectile::new(initial_position, velocity, 1.0, f32::MAX));
    battle.update(1.0);
    expected_structure.set_voxel(-1, -1, 0, None);
    assert!(battle.bodies()[0].structure().clone() == expected_structure);

    battle.add_projectile(Projectile::new(initial_position, velocity, 1.0, f32::MAX));
    battle.update(1.0);
    // Nothing new, the projectile should have missed.
    assert!(battle.bodies()[0].structure().clone() == expected_structure);
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
