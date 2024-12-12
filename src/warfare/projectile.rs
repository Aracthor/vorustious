use crate::maths::vector::Vect3f;

pub struct Projectile {
    position: Vect3f,
    velocity: Vect3f,
    damage: f32,
    max_range: f32,
    traveled_distance: f32,
}

impl Projectile {
    pub fn new(position: Vect3f, velocity: Vect3f, damage: f32, max_range: f32) -> Self {
        Self {
            position: position,
            velocity: velocity,
            damage: damage,
            max_range: max_range,
            traveled_distance: 0.0,
        }
    }

    pub fn position(&self) -> Vect3f {
        self.position
    }

    pub fn velocity(&self) -> Vect3f {
        self.velocity
    }

    pub fn damage(&self) -> f32 {
        self.damage
    }

    pub fn is_out_of_max_range(&self) -> bool {
        self.traveled_distance > self.max_range
    }

    pub fn moove(&mut self, speed: f32) {
        self.position += self.velocity * speed;
        self.traveled_distance += self.velocity.length() * speed;
    }
}
