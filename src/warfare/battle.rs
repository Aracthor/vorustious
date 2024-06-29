use std::collections::{HashSet, VecDeque};

use super::body::Body;
use super::projectile::Projectile;
use crate::maths::segment::Segm3f;
use crate::maths::matrix::Mat4f;
use crate::maths::vector::Vect3f;
use crate::maths::vector::Vect3i;
use crate::voxels::structure::Structure;
use crate::voxels::voxel::Voxel;

pub struct Battle {
    bodies: Vec<Body>,
    projectiles: Vec<Projectile>,
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

impl Battle {
    pub fn new() -> Self {
        Self {
            bodies: vec![],
            projectiles: vec![],
        }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn add_projectile(&mut self, projectile: Projectile) {
        self.projectiles.push(projectile);
    }

    pub fn bodies(&self) -> &Vec<Body> {
        &self.bodies
    }

    pub fn bodies_mut(&mut self) -> &mut Vec<Body> {
        &mut self.bodies
    }

    pub fn projectiles(&self) -> &Vec<Projectile> {
        &self.projectiles
    }

    pub fn update(&mut self, elapsed_time: f32) {
        for body in &mut self.bodies {
            body.apply_movement(elapsed_time);
        }

        self.projectiles.retain_mut(|projectile| {
            let segment_start = projectile.position();
            projectile.moove(elapsed_time);
            let segment_end = projectile.position();
            let mut hit = false;
            if !projectile.is_out_of_max_range() {
                let segment = Segm3f::new(segment_start, segment_end);
                for body in &mut self.bodies {
                    hit |= body.for_first_voxel_in_segment(segment, |voxel: &mut Option<Voxel>, _coords| {
                        voxel.as_mut().unwrap().life -= projectile.damage();
                    });
                }
            }
            !hit && !projectile.is_out_of_max_range()
        });

        let mut new_bodies = vec![];
        for body in &mut self.bodies {
            let mut destroyed_coords = vec![];
            body.structure_mut().for_each_voxel_mut(|coords, voxel: &mut Option<Voxel>| {
                if voxel.unwrap().life <= 0.0 {
                    *voxel = None;
                    destroyed_coords.push(coords);
                }
            });

            let coords_to_check = {
                let mut result = vec![];
                for coord in destroyed_coords {
                    for coord_to_check in close_coords(coord) {
                        if body.structure().has_voxel_on_coords(coord_to_check) {
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
                        if body.structure().has_voxel_on_coords(close_coords) && !explored_coords.contains(&close_coords) {
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
                            let voxel = body.structure_mut().remove_voxel(coords);
                            new_structure.add_voxel(coords, voxel);
                        }
                        let new_center = new_structure.recenter();

                        let translation = Vect3f::new([new_center[0] as f32, new_center[1] as f32, new_center[2] as f32]);
                        let new_repere = body.repere().clone() * Mat4f::translation(translation);
                        let mut new_body = Body::new_from_other(new_structure, new_repere, body);
                        // Debug to see result
                        new_body.add_to_movement(translation.normalize());
                        new_bodies.push(new_body);
                    }
                }
            }
        }
        self.bodies.append(&mut new_bodies);
    }
}
