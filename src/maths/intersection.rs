use super::boxes::Box1f;
use super::boxes::Box3f;
use super::matrix::Mat4f;
use super::vector::Vect3f;

pub type OBBAxis = [Vect3f; 15];

fn projection_intersects(corners_a: &Vec<Vect3f>, corners_b: &Vec<Vect3f>, axis: Vect3f) -> bool {
    if axis == Vect3f::zero() {
        return true;
    }
    let mut range_a = Box1f::new();
    let mut range_b = Box1f::new();
    for i in 0..8 {
        range_a.add_value(Vect3f::dot(corners_a[i], axis));
        range_b.add_value(Vect3f::dot(corners_b[i], axis));
    }
    range_a.intersects(&range_b)
}

pub fn obb_intersect_with_axis(box_a: Box3f, repere_a: &Mat4f, box_b: Box3f, repere_b: &Mat4f, axis: &OBBAxis) -> bool {
    let corners_a = box_a.corners().iter().map(|corner| repere_a.clone() * *corner).collect();
    let corners_b = box_b.corners().iter().map(|corner| repere_b.clone() * *corner).collect();
    for ax in axis {
        if !projection_intersects(&corners_a, &corners_b, *ax) {
            return false;
        }
    }
    true
}

pub fn boxes_projection_axis(repere_a: &Mat4f, repere_b: &Mat4f) -> OBBAxis {
    let repere_a_without_translation = repere_a.without_translation();
    let repere_b_without_translation = repere_b.without_translation();
    let axis_x_a = repere_a_without_translation.clone() * Vect3f::new([1.0, 0.0, 0.0]);
    let axis_y_a = repere_a_without_translation.clone() * Vect3f::new([0.0, 1.0, 0.0]);
    let axis_z_a = repere_a_without_translation.clone() * Vect3f::new([0.0, 0.0, 1.0]);
    let axis_x_b = repere_b_without_translation.clone() * Vect3f::new([1.0, 0.0, 0.0]);
    let axis_y_b = repere_b_without_translation.clone() * Vect3f::new([0.0, 1.0, 0.0]);
    let axis_z_b = repere_b_without_translation.clone() * Vect3f::new([0.0, 0.0, 1.0]);
    [
        axis_x_a,
        axis_y_a,
        axis_z_a,
        axis_x_b,
        axis_y_b,
        axis_z_b,
        Vect3f::cross(axis_x_a, axis_x_b),
        Vect3f::cross(axis_x_a, axis_y_b),
        Vect3f::cross(axis_x_a, axis_z_b),
        Vect3f::cross(axis_y_a, axis_x_b),
        Vect3f::cross(axis_y_a, axis_y_b),
        Vect3f::cross(axis_y_a, axis_z_b),
        Vect3f::cross(axis_z_a, axis_x_b),
        Vect3f::cross(axis_z_a, axis_y_b),
        Vect3f::cross(axis_z_a, axis_z_b),
    ]
}

#[allow(dead_code)]
pub fn obb_intersect(box_a: Box3f, repere_a: &Mat4f, box_b: Box3f, repere_b: &Mat4f) -> bool {
    let axis = boxes_projection_axis(repere_a, repere_b);
    obb_intersect_with_axis(box_a, repere_a, box_b, repere_b, &axis)
}
