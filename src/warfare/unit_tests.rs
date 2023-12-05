use crate::maths::vector::Vect3f;
use super::projectile::Projectile;

#[test]
fn projectile_movement() {
    let initial_position = Vect3f::new([0.0, 0.0, 2.0]);
    let movement = Vect3f::new([1.0, -2.0, 0.0]);
    let projectile_speed = 10.0;
    let mut projectile = Projectile::new(initial_position, movement, 1.0, projectile_speed, f32::MAX);
    projectile.moove(2.0);
    projectile.moove(2.0);
    projectile.moove(2.0);
    let expected_position = initial_position + movement * projectile_speed * 6.0;
    assert!(projectile.position() == expected_position);
}

#[test]
fn projectile_max_distance() {
    let initial_position = Vect3f::zero();
    let movement = Vect3f::new([1.0, 0.0, 0.0]);
    let projectile_speed = 10.0;
    let max_distance = 25.0;
    let mut projectile = Projectile::new(initial_position, movement, 1.0, projectile_speed, max_distance);
    projectile.moove(1.0);
    assert!(!projectile.is_out_of_max_range());
    projectile.moove(1.0);
    assert!(!projectile.is_out_of_max_range());
    projectile.moove(1.0);
    assert!(projectile.is_out_of_max_range());
}
