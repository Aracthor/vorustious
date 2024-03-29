use std::time::Duration;
use std::time::Instant;

use crate::maths::vector::Vect3f;
use super::projectile::Projectile;

pub struct Weapon {
    firerate: Duration,
    damage: f32,
    projectile_speed: f32,
    max_range: f32,
    last_shot_time: Instant,
}

impl Weapon {
    pub fn new(firerate: f32, damage: f32, projectile_speed: f32, max_range: f32) -> Self {
        Self {
            firerate: Duration::from_secs_f32(firerate),
            damage: damage,
            projectile_speed: projectile_speed,
            max_range: max_range,
            last_shot_time: Instant::now(),
        }
    }

    pub fn shoot(&mut self, position: Vect3f, direction: Vect3f) -> Option<Projectile> {
        if self.last_shot_time.elapsed() > self.firerate {
            self.last_shot_time = Instant::now();
            return Some(Projectile::new(position, direction * self.projectile_speed, self.damage, self.max_range));
        }
        None
    }
}
