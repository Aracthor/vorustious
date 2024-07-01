use std::collections::HashSet;
use std::collections::VecDeque;

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

fn close_coords(coords: Vect3i) -> [Vect3i; 6] {
    [
        coords + Vect3i::new([ 1, 0, 0]),
        coords + Vect3i::new([-1, 0, 0]),
        coords + Vect3i::new([0,  1, 0]),
        coords + Vect3i::new([0, -1, 0]),
        coords + Vect3i::new([0, 0,  1]),
        coords + Vect3i::new([0, 0, -1]),
    ]
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

        let distance_normalized = distance.normalize();
        let roll = other.roll() * Vect3f::dot(distance_normalized, Vect3f::new([1.0, 0.0, 0.0])).abs();
        let pitch = other.pitch() * Vect3f::dot(distance_normalized, Vect3f::new([0.0, 1.0, 0.0])).abs();
        let yaw = other.yaw() * Vect3f::dot(distance_normalized, Vect3f::new([0.0, 0.0, 1.0])).abs();

        Self {
            repere: new_repere,
            structure: structure,
            weapons: vec![],
            movement: new_movement,
            rotation: Vect3f::new([roll, pitch, yaw]),
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

    pub fn update_dead_voxels(&mut self) -> Vec<Body> {
        let mut new_bodies: Vec<Body> = vec![];
        let mut destroyed_coords = vec![];
        self.structure.for_each_voxel_mut(|coords, voxel: &mut Option<Voxel>| {
            if voxel.unwrap().life <= 0.0 {
                *voxel = None;
                destroyed_coords.push(coords);
            }
        });

        let coords_to_check = {
            let mut result = vec![];
            for coord in destroyed_coords {
                for coord_to_check in close_coords(coord) {
                    if self.structure.has_voxel_on_coords(coord_to_check) {
                        result.push(coord_to_check);
                    }
                }
            }
            result
        };

        // Cut in separate bodies disjoincted structures.
        let mut coords_to_reach: HashSet<Vect3i> = coords_to_check.clone().into_iter().collect();
        let mut jointed_coords: Vec<HashSet<Vect3i>> = vec![];
        while !coords_to_reach.is_empty() {
            let first_coords = coords_to_reach.iter().next().unwrap().clone();
            let mut coords_to_explore: VecDeque<Vect3i> = Default::default();
            let mut explored_coords: HashSet<Vect3i> = Default::default();
            coords_to_explore.push_back(first_coords);
            coords_to_reach.remove(&first_coords);
            explored_coords.insert(first_coords);
            while !coords_to_explore.is_empty() && (!coords_to_reach.is_empty() || !jointed_coords.is_empty())  {
                let coords = coords_to_explore.pop_front().unwrap();
                for close_coords in close_coords(coords) {
                    if self.structure.has_voxel_on_coords(close_coords) && !explored_coords.contains(&close_coords) {
                        coords_to_reach.remove(&close_coords);
                        coords_to_explore.push_back(close_coords);
                        explored_coords.insert(close_coords);
                    }
                }
            }
            jointed_coords.push(explored_coords);
        }
        if jointed_coords.len() > 1 {
            for join in jointed_coords {
                if !join.contains(&Vect3i::zero()) {
                    let mut new_structure = Structure::new_empty();
                    for coords in join {
                        let voxel = self.structure.remove_voxel(coords);
                        new_structure.add_voxel(coords, voxel);
                    }
                    let new_center = new_structure.recenter();
                    let translation = Vect3f::new([new_center[0] as f32, new_center[1] as f32, new_center[2] as f32]);
                    let mut new_body = Body::new_from_other(new_structure, translation, self);

                    // Debug to see result
                    new_body.add_to_movement(translation.normalize());
                    new_bodies.push(new_body);
                }
            }
        }
        if !new_bodies.is_empty() {
            self.structure.recalculate_box();
        }
        new_bodies
    }
}
