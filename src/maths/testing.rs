use super::matrix::Mat4f;
use super::vector::Vect;

pub fn equals_with_delta(a: f32, b: f32, delta: f32) -> bool {
    (a - b).abs() < delta
}

pub fn vec_equals_with_delta<const N: usize>(u: Vect<N, f32>, v: Vect<N, f32>, delta: f32) -> bool {
    for i in 0..N {
        if !equals_with_delta(u[i], v[i], delta) {
            return false;
        }
    }
    true
}

pub fn mat_equals_with_delta(u: Mat4f, v: Mat4f, delta: f32) -> bool {
    for y in 0..4 {
        if !vec_equals_with_delta(u[y], v[y], delta) {
            return false;
        }
    }
    true
}

