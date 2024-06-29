use crate::maths::matrix::Mat4f;
use crate::maths::vector::Vect3f;
use crate::maths::vector::Vect3i;
use crate::voxels::body::Body;
use crate::voxels::structure::Structure;
use crate::voxels::voxel::Voxel;
use crate::voxels::voxel::VoxelID;
use super::battle::Battle;
use super::projectile::Projectile;


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
    let structure = Structure::new(-1, 1, -1, 1, -1, 1, Voxel{life: 2.0, id: VoxelID::LightHull});
    battle.add_body(Body::new(structure, Mat4f::identity()));

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
    let structure = Structure::new(-1, 1, -1, 1, -1, 1, Voxel{life: 2.0, id: VoxelID::LightHull});
    let mut expected_structure = structure.clone();
    let mut body = Body::new(structure, Mat4f::identity());
    body.add_to_movement(Vect3f::new([0.0, 0.4, 0.0]));
    battle.add_body(body);

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
fn structure_cut_in_half() {
    let mut battle = Battle::new();
    let structure = {
        let mut structure = Structure::new(-2, 2, 0, 0, 0, 0, Voxel{life: 2.0, id: VoxelID::LightHull});
        structure.add_voxel(Vect3i::new([2, 1, 0]), Voxel{life: 2.0, id: VoxelID::LightHull});
        structure.add_voxel(Vect3i::new([2, 1, 1]), Voxel{life: 2.0, id: VoxelID::LightHull});
        structure.add_voxel(Vect3i::new([2, 0, 1]), Voxel{life: 2.0, id: VoxelID::LightHull});
        structure.add_voxel(Vect3i::new([2, -1, 1]), Voxel{life: 2.0, id: VoxelID::LightHull});
        structure.add_voxel(Vect3i::new([2, -1, 0]), Voxel{life: 2.0, id: VoxelID::LightHull});
        structure.add_voxel(Vect3i::new([2, -1, -1]), Voxel{life: 2.0, id: VoxelID::LightHull});
        structure.add_voxel(Vect3i::new([2, 0, -1]), Voxel{life: 2.0, id: VoxelID::LightHull});
        structure.add_voxel(Vect3i::new([2, 1, -1]), Voxel{life: 2.0, id: VoxelID::LightHull});
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
        structure
    };
    let expected_new_structure = Structure::new(0, 0, -1, 1, -1, 1, Voxel{life: 2.0, id: VoxelID::LightHull});

    let body = Body::new(structure, Mat4f::identity());
    battle.add_body(body);

    let initial_position = Vect3f::new([1.0, 0.0, 2.0]);
    let movement = Vect3f::new([0.0, 0.0, -10.0]);
    battle.add_projectile(Projectile::new(initial_position, movement, 10.0, f32::MAX));

    battle.update(1.0);

    assert!(battle.bodies()[0].structure().clone() == expected_remaining_structure);
    assert!(battle.bodies()[1].structure().clone() == expected_new_structure);
}
