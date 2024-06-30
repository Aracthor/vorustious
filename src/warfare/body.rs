use crate::maths::matrix::Mat4f;
use crate::maths::segment::Segm3f;
use crate::maths::vector::Vect3f;
use crate::maths::vector::Vect3i;
use crate::voxels::structure::Structure;
use crate::voxels::voxel::Voxel;
use super::projectile::Projectile;
use super::weapon::Weapon;

pub struct Body {
    repere: Mat4f,
    structure: Structure,
    weapons: Vec<(Vect3f, Weapon)>,
    movement: Vect3f,
    rotation: Vect3f,
}

impl Body {
    pub fn new(structure: Structure, repere: Mat4f) -> Self {
        Self {
            repere: repere,
            structure: structure,
            weapons: vec![],
            movement: Vect3f::zero(),
            rotation: Vect3f::zero(),
        }
    }

    pub fn new_from_other(structure: Structure, distance: Vect3f, other: &Body) -> Self {
        let new_repere = other.repere().clone() * Mat4f::translation(distance);
        let repere_without_translation = {
            let mut repere = other.repere().clone();
            repere[3][0] = 0.0;
            repere[3][1] = 0.0;
            repere[3][2] = 0.0;
            repere
        };
        let radius_sq = distance.length_sq();
        let local_distance = repere_without_translation.clone() * distance.normalize();
        let angular_speed_x_sq = other.roll() * other.roll();
        let angular_speed_y_sq = other.pitch() * other.pitch();
        let angular_speed_z_sq = other.yaw() * other.yaw();
        let speed_from_roll_centrifugal = Vect3f::cross(local_distance, Vect3f::new([-1.0, 0.0, 0.0])) * f32::sqrt(radius_sq * angular_speed_x_sq);
        let speed_from_yaw_centrifugal = Vect3f::cross(local_distance, Vect3f::new([0.0, -1.0, 0.0])) * f32::sqrt(radius_sq * angular_speed_y_sq);
        let speed_from_pitch_centrifugal = Vect3f::cross(local_distance, Vect3f::new([0.0, 0.0, -1.0])) * f32::sqrt(radius_sq * angular_speed_z_sq);
        let new_movement = other.movement + speed_from_roll_centrifugal + speed_from_yaw_centrifugal + speed_from_pitch_centrifugal;

        Self {
            repere: new_repere,
            structure: structure,
            weapons: vec![],
            movement: new_movement,
            rotation: Vect3f::zero(),
        }
    }

    pub fn repere(&self) -> &Mat4f {
        &self.repere
    }

    pub fn structure(&self) -> &Structure {
        &self.structure
    }

    pub fn yaw(&self) -> f32 {
        self.rotation[2]
    }

    pub fn pitch(&self) -> f32 {
        self.rotation[1]
    }

    pub fn roll(&self) -> f32 {
        self.rotation[0]
    }

    pub fn structure_mut(&mut self) -> &mut Structure {
        &mut self.structure
    }

    pub fn for_first_voxel_in_segment<F: FnMut(&mut Option<Voxel>, &Vect3i)>(&mut self, segment: Segm3f, f: F) -> bool {
        let segment_in_repere = segment.transform(&self.repere.inverse());
        self.structure.for_first_voxel_in_segment(segment_in_repere, f)
    }

    pub fn add_to_movement(&mut self, movement: Vect3f) {
        self.movement += movement;
    }

    pub fn add_yaw_rotation(&mut self, yaw: f32) {
        self.rotation[2] += yaw;
    }

    pub fn add_pitch_rotation(&mut self, pitch: f32) {
        self.rotation[1] += pitch;
    }

    pub fn add_roll_rotation(&mut self, roll: f32) {
        self.rotation[0] += roll;
    }

    pub fn scale_movement(&mut self, scale: f32) {
        self.movement *= scale;
    }

    pub fn scale_rotation(&mut self, scale: f32) {
        self.rotation *= scale;
    }

    pub fn apply_movement(&mut self, elapsed_time: f32) {
        self.repere = Mat4f::translation(self.movement * elapsed_time)
        * self.repere.clone()
        * Mat4f::rotation_around_z(self.rotation[2] * elapsed_time)
        * Mat4f::rotation_around_y(self.rotation[1] * elapsed_time)
        * Mat4f::rotation_around_x(self.rotation[0] * elapsed_time)
    }

    pub fn add_weapon(&mut self, position: Vect3f, weapon: Weapon) {
        self.weapons.push((position, weapon));
    }

    pub fn shoot(&mut self) -> Vec<Projectile> {
        let mut result = vec![];
        let direction = self.repere.forward();
        for weapon in &mut self.weapons {
            let position = self.repere.clone() * weapon.0;
            let projectile = weapon.1.shoot(position, direction);
            if projectile.is_some() {
                result.push(projectile.unwrap());
            }
        }
        result
    }
}
