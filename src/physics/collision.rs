use super::body::Body;
use crate::maths::boxes::Box3f;
use crate::maths::boxes::Box3i;
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

pub fn intersection(body_a: &Body, body_b: &Body) -> Vec<(Vect3i, Vect3i)> {
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
    let intersections = intersection(body_a, body_b);
    if !intersections.is_empty() {
        let momentum_a = body_a.momentum();
        let momentum_b = body_b.momentum();
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