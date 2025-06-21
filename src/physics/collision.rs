use std::cmp::Ordering;

use super::body::Body;
use crate::maths::boxes::Box3i;
use crate::maths::boxes::Box3f;
use crate::maths::vector::Vect3f;
use crate::maths::vector::Vect3i;
use crate::maths::intersection;
use crate::maths::intersection::OBBAxis;

fn taking_box(body: &Body) -> Box3f {
    let mut result = Box3f::from_min_max(Vect3f::all(-1.0), Vect3f::all(1.0));
    let structure_box = body.structure().get_box();
    let min_to_take = Vect3f::new([structure_box.min()[0] as f32 + 0.5, structure_box.min()[1] as f32 + 0.5, structure_box.min()[2] as f32 + 0.5]);
    let max_to_take = Vect3f::new([structure_box.max()[0] as f32 + 0.5, structure_box.max()[1] as f32 + 0.5, structure_box.max()[2] as f32 + 0.5]);
    let box_to_take_in = Box3f::from_min_max(min_to_take, max_to_take);
    while !result.contains_box(&box_to_take_in) {
        result = Box3f::from_min_max(result.min() * 2.0, result.max() * 2.0);
    }
    result
}

fn voxels_intersection(body_a: &Body, box_a: Box3f, body_b: &Body, box_b: Box3f, axis: &OBBAxis)-> Vec<(Vect3i, Vect3i)> {
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
        if body_a.structure().has_voxel_on_coords(voxel_a) && body_b.structure().has_voxel_on_coords(voxel_b) {
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
            if body_b.structure().octtree().has_box(subbox_i32) {
                result.extend(voxels_intersection(body_a, box_a.clone(), body_b, subbox, axis));
            }
        }
    } else {
        for subbox in box_a.subdivide() {
            let min = subbox.min() + Vect3f::new([0.5, 0.5, 0.5]);
            let max = subbox.max() - Vect3f::new([0.5, 0.5, 0.5]);
            let subbox_i32 = Box3i::from_min_max(Vect3i::new([min[0] as i32, min[1] as i32, min[2] as i32]), Vect3i::new([max[0] as i32, max[1] as i32, max[2] as i32]));
            if body_a.structure().octtree().has_box(subbox_i32) {
                result.extend(voxels_intersection(body_a, subbox, body_b, box_b.clone(), axis));
            }
        }
    }
    result
}

pub fn intersection_high_precision(body_a: &Body, body_b: &Body) -> Vec<(Vect3i, Vect3i)> {
    let axis = intersection::boxes_projection_axis(body_a.repere(), body_b.repere());
    let box_a = body_a.get_box();
    let box_b = body_b.get_box();
    if !box_a.intersects(&box_b) {
        return Default::default();
    }
    let englobing_box_a = taking_box(body_a);
    let englobing_box_b = taking_box(body_b);
    voxels_intersection(body_a, englobing_box_a, body_b, englobing_box_b, &axis)
}

pub fn apply_collision_if_any(body_a: &mut Body, body_b: &mut Body, restitution: f32) {
    let intersections = intersection_low_precision(body_a, body_b);
    if !intersections.is_empty() {
        let (intersections_a, intersections_b): (Vec<Vect3i>, Vec<Vect3i>) = intersections.into_iter().unzip();
        let average_intersection_a = intersections_a.iter().sum::<Vect3i>() / intersections_a.len() as i32;
        let average_intersection_b = intersections_b.iter().sum::<Vect3i>() / intersections_b.len() as i32;
        let momentum_a = body_a.momentum(average_intersection_a);
        let momentum_b = body_b.momentum(average_intersection_b);
        let mass_a = body_a.structure().mass();
        let mass_b = body_b.structure().mass();
        let velocity_a = body_a.velocity();
        let velocity_b = body_b.velocity();
        let total_mass = mass_a + mass_b;

        let new_velocity_a = (momentum_a + momentum_b + (velocity_b - velocity_a) * mass_b * restitution) / total_mass;
        let new_velocity_b = (momentum_a + momentum_b + (velocity_a - velocity_b) * mass_a * restitution) / total_mass;

        body_a.set_velocity(new_velocity_a);
        body_b.set_velocity(new_velocity_b);
    }
}

fn compare_coords(a: Vect3i, b: Vect3i) -> Ordering {
    if a[0] != b[0] {
        return a[0].cmp(&b[0]);
    }
    if a[1] != b[1] {
        return a[1].cmp(&b[1]);
    }
    if a[2] != b[2] {
        return a[2].cmp(&b[2]);
    }
    Ordering::Equal
}

fn body_structure_as_coords(body: &Body) -> Vec<(Vect3i, Vect3i)> {
    let mut result = vec![];
    body.structure().for_each_voxel(|coords, _voxel|{
        let coord_f = Vect3f::new([coords[0] as f32, coords[1] as f32, coords[2] as f32]);
        let transformed_coord = body.repere().clone() * coord_f;
        result.push((coords, Vect3i::new([transformed_coord[0].round() as i32, transformed_coord[1].round() as i32, transformed_coord[2].round() as i32])));
    });
    result.sort_by(|a, b| compare_coords(a.1, b.1));
    result
}

pub fn intersection_low_precision(body_a: &Body, body_b: &Body) -> Vec<(Vect3i, Vect3i)> {
    let box_a = body_a.get_box();
    let box_b = body_b.get_box();
    if !box_a.intersects(&box_b) {
        return Default::default();
    }

    let body_a_coords = body_structure_as_coords(body_a);
    let body_b_coords = body_structure_as_coords(body_b);

    // TODO there must be some STL-way to have intersection between vectors from a specific compare...
    let mut result = vec![];
    let mut idx_a = 0;
    let mut idx_b = 0;
    while idx_a < body_a_coords.len() && idx_b < body_b_coords.len() {
        let coord_a = body_a_coords[idx_a];
        let coord_b = body_b_coords[idx_b];
        let compare = compare_coords(coord_a.1, coord_b.1);
        match compare {
            Ordering::Less => {
                idx_a += 1;
            }
            Ordering::Equal => {
                result.push((coord_a.0, coord_b.0));
                idx_a += 1;
            }
            Ordering::Greater => {
                idx_b += 1;
            }
        }
    }
    result
}
