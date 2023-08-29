use super::matrix::Mat4f;
use super::vector::Vect3f;

fn equals_with_delta(u: Vect3f, v: Vect3f, delta: f32) -> bool {
    (u[0] - v[0]).abs() < delta && (u[1] - v[1]).abs() < delta && (u[2] - v[2]).abs() < delta
}

fn mat_equals_with_delta(u: Mat4f, v: Mat4f, delta: f32) -> bool {
    for i in 0..16 {
        if (u[i] - v[i]).abs() >= delta {
            return false;
        }
    }
    true
}

#[test]
fn vector_length() {
    let u = Vect3f::new([0.0, 0.0, 0.0]);
    assert!(u.length_sq() == 0.0);
    assert!(u.length() == 0.0);

    let u = Vect3f::new([3.0, 0.0, 4.0]);
    assert!(u.length_sq() == 25.0);
    assert!(u.length() == 5.0);
    assert!(u.normalize() == Vect3f::new([0.6, 0.0, 0.8]));
}

#[test]
fn vector_op() {
    let u = Vect3f::new([2.0, -1.0, 5.0]);
    let v = Vect3f::new([3.0, -4.2, -2.4]);

    assert!(-u == Vect3f::new([-2.0, 1.0, -5.0]));
    assert!(equals_with_delta(u - v, Vect3f::new([-1.0, 3.2, 7.4]), 0.0001));
    assert!(equals_with_delta(u / 2.0, Vect3f::new([1.0, -0.5, 2.5]), 0.0001));
}

#[test]
fn vector_funcs() {
    let u = Vect3f::new([3.0, -3.0, 1.0]);
    let v = Vect3f::new([4.0, 9.0, 2.0]);
    assert!(Vect3f::dot(u, v) == -13.0);
    assert!(equals_with_delta(Vect3f::cross(u, v), Vect3f::new([-15.0, -2.0, 39.0]), 0.0001));
}

#[test]
fn projection_matrix() {
    let left = 0.0;
    let right = 800.0;
    let bottom = 0.0;
    let top = 600.0;
    let z_near = 0.1;
    let z_far = 1000.0;
    let matrix = Mat4f::orthographic(left, right, bottom, top, z_near, z_far);

    let expected = {
        let mut expected = Mat4f::identity();
        expected.set(0, 0, 2.0 / 800.0);
        expected.set(1, 1, 2.0 / 600.0);
        expected.set(2, 2, -2.0 / 999.9);
        expected.set(0, 3, -1.0);
        expected.set(1, 3, -1.0);
        expected.set(2, 3, -1.0002);
        expected
    };

    assert!(mat_equals_with_delta(matrix, expected, 0.0001));
}
