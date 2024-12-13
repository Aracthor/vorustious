use std::collections::HashSet;
use std::collections::VecDeque;

use crate::maths::boxes::Box3i;
use crate::maths::boxes::Box3f;
use crate::maths::intersection;
use crate::maths::intersection::OBBAxis;
use crate::maths::matrix::Mat4f;
use crate::maths::segment::Segm3f;
use crate::maths::vector::Vect3f;
use crate::maths::vector::Vect3i;
use crate::voxels::structure::Structure;
use crate::voxels::voxel::Voxel;

pub struct Body {
    repere: Mat4f,
    structure: Structure,
    velocity: Vect3f,
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
            velocity: Vect3f::zero(),
            rotation: Vect3f::zero(),
        }
    }

    pub fn new_from_other(structure: Structure, distance: Vect3f, other: &Body) -> Self {
        if distance == Vect3f::zero() {
            return Self {
                repere: other.repere.clone(),
                structure: structure,
                velocity: other.velocity,
                rotation: other.rotation,
            };
        }

        let new_repere = other.repere().clone() * Mat4f::translation(distance);
        let radius_sq = distance.length_sq();
        let local_distance = other.repere().without_translation() * distance.normalize();
        let angular_speed_x_sq = other.roll() * other.roll();
        let angular_speed_y_sq = other.pitch() * other.pitch();
        let angular_speed_z_sq = other.yaw() * other.yaw();
        let velocity_from_roll_centrifugal = Vect3f::cross(local_distance, Vect3f::new([-1.0, 0.0, 0.0])) * f32::sqrt(radius_sq * angular_speed_x_sq);
        let velocity_from_yaw_centrifugal = Vect3f::cross(local_distance, Vect3f::new([0.0, -1.0, 0.0])) * f32::sqrt(radius_sq * angular_speed_y_sq);
        let velocity_from_pitch_centrifugal = Vect3f::cross(local_distance, Vect3f::new([0.0, 0.0, -1.0])) * f32::sqrt(radius_sq * angular_speed_z_sq);
        let velocity = other.velocity + velocity_from_roll_centrifugal + velocity_from_yaw_centrifugal + velocity_from_pitch_centrifugal;

        let distance_normalized = distance.normalize();
        let roll = other.roll() * Vect3f::dot(distance_normalized, Vect3f::new([1.0, 0.0, 0.0])).abs();
        let pitch = other.pitch() * Vect3f::dot(distance_normalized, Vect3f::new([0.0, 1.0, 0.0])).abs();
        let yaw = other.yaw() * Vect3f::dot(distance_normalized, Vect3f::new([0.0, 0.0, 1.0])).abs();

        Self {
            repere: new_repere,
            structure: structure,
            velocity,
            rotation: Vect3f::new([roll, pitch, yaw]),
        }
    }

    pub fn repere(&self) -> &Mat4f {
        &self.repere
    }

    pub fn structure(&self) -> &Structure {
        &self.structure
    }

    pub fn get_box(&self) -> Box3f {
        let voxel_box = self.structure.get_box();
        let mut result = Box3f::new();
        for corner in voxel_box.corners() {
            result.add(self.repere.clone() * corner);
        }
        result
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

    pub fn for_first_voxel_in_segment<F: FnMut(&mut Voxel, &Vect3i)>(&mut self, segment: Segm3f, f: F) -> bool {
        let segment_in_repere = segment.transform(&self.repere.inverse());
        self.structure.for_first_voxel_in_segment(segment_in_repere, f)
    }

    pub fn add_to_velocity(&mut self, velocity: Vect3f) {
        self.velocity += velocity;
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

    pub fn scale_velocity(&mut self, scale: f32) {
        self.velocity *= scale;
    }

    pub fn scale_rotation(&mut self, scale: f32) {
        self.rotation *= scale;
    }

    pub fn apply_velocity(&mut self, elapsed_time: f32) {
        self.repere = Mat4f::translation(self.velocity * elapsed_time)
        * self.repere.clone()
        * Mat4f::rotation_around_z(self.rotation[2] * elapsed_time)
        * Mat4f::rotation_around_y(self.rotation[1] * elapsed_time)
        * Mat4f::rotation_around_x(self.rotation[0] * elapsed_time)
    }

    pub fn update_dead_voxels(&mut self) -> Vec<Body> {
        let mut new_bodies: Vec<Body> = vec![];
        let destroyed_coords = self.structure.erase_dead_voxels();

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
                    new_body.add_to_velocity(if translation == Vect3f::zero() { Vect3f::new([0.0, 0.0, 1.0]) } else { translation.normalize() });
                    new_bodies.push(new_body);
                }
            }
        }
        if !new_bodies.is_empty() {
            self.structure.recalculate_box();
        }
        new_bodies
    }

    fn taking_box(&self) -> Box3f {
        let mut result = Box3f::from_min_max(Vect3f::all(-1.0), Vect3f::all(1.0));
        let structure_box = self.structure.get_box();
        let min_to_take = Vect3f::new([structure_box.min()[0] as f32 + 0.5, structure_box.min()[1] as f32 + 0.5, structure_box.min()[2] as f32 + 0.5]);
        let max_to_take = Vect3f::new([structure_box.max()[0] as f32 + 0.5, structure_box.max()[1] as f32 + 0.5, structure_box.max()[2] as f32 + 0.5]);
        let box_to_take_in = Box3f::from_min_max(min_to_take, max_to_take);
        while !result.contains_box(&box_to_take_in) {
            result = Box3f::from_min_max(result.min() * 2.0, result.max() * 2.0);
        }
        result
    }

    fn voxels_intersection(body_a: &Self, box_a: Box3f, body_b: &Self, box_b: Box3f, axis: &OBBAxis)-> Vec<(Vect3i, Vect3i)> {
        let recenter = Vect3f::all(0.5);
        let box_a_centered = Box3f::from_min_max(box_a.min() + recenter, box_a.max() + recenter);
        let box_b_centered = Box3f::from_min_max(box_b.min() + recenter, box_b.max() + recenter);
        if !intersection::obb_intersect_with_axis(box_a_centered.clone(), body_a.repere(), box_b_centered.clone(), body_b.repere(), axis) {
            return vec![];
        }

        let size_a = box_a.extent()[0];
        let size_b = box_b.extent()[0];
        if size_a == 1.0 && size_b == 1.0 {
            let box_a_center = box_a_centered.center();
            let box_b_center = box_b_centered.center();
            let voxel_a = Vect3i::new([box_a_center[0] as i32, box_a_center[1] as i32, box_a_center[2] as i32]);
            let voxel_b = Vect3i::new([box_b_center[0] as i32, box_b_center[1] as i32, box_b_center[2] as i32]);
            if body_a.structure.has_voxel_on_coords(voxel_a) && body_b.structure.has_voxel_on_coords(voxel_b) {
                return vec![(voxel_a, voxel_b)];
            }
            return vec![];
        }

        let mut result: Vec<(Vect3i, Vect3i)> = vec![];
        if size_a < size_b {
            for subbox in box_b.subdivide() {
                let min = subbox.min() + Vect3f::new([0.5, 0.5, 0.5]);
                let max = subbox.max() - Vect3f::new([0.5, 0.5, 0.5]);
                let subbox_i32 = Box3i::from_min_max(Vect3i::new([min[0] as i32, min[1] as i32, min[2] as i32]), Vect3i::new([max[0] as i32, max[1] as i32, max[2] as i32]));
                if body_b.structure.octtree().has_box(subbox_i32) {
                    result.extend(Self::voxels_intersection(body_a, box_a.clone(), body_b, subbox, axis));
                }
            }
        } else {
            for subbox in box_a.subdivide() {
                let min = subbox.min() + Vect3f::new([0.5, 0.5, 0.5]);
                let max = subbox.max() - Vect3f::new([0.5, 0.5, 0.5]);
                let subbox_i32 = Box3i::from_min_max(Vect3i::new([min[0] as i32, min[1] as i32, min[2] as i32]), Vect3i::new([max[0] as i32, max[1] as i32, max[2] as i32]));
                if body_a.structure.octtree().has_box(subbox_i32) {
                    result.extend(Self::voxels_intersection(body_a, subbox, body_b, box_b.clone(), axis));
                }
            }
        }
        result
    }

    pub fn intersection(body_a: &Self, body_b: &Self) -> Vec<(Vect3i, Vect3i)> {
        let axis = intersection::boxes_projection_axis(body_a.repere(), body_b.repere());
        let box_a = body_a.get_box();
        let box_b = body_b.get_box();
        if !box_a.intersects(&box_b) {
            return Default::default();
        }
        let englobing_box_a = body_a.taking_box();
        let englobing_box_b = body_b.taking_box();
        Self::voxels_intersection(body_a, englobing_box_a, body_b, englobing_box_b, &axis)
    }
}
