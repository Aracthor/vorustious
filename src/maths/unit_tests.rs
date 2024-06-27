use super::boxes::Box;
use super::matrix::Mat3f;
use super::matrix::Mat4f;
use super::vector::Vect;
use super::vector::Vect3f;

fn equals_with_delta(a: f32, b: f32, delta: f32) -> bool {
    (a - b).abs() < delta
}

fn vec_equals_with_delta<const N: usize>(u: Vect<N, f32>, v: Vect<N, f32>, delta: f32) -> bool {
    for i in 0..N {
        if !equals_with_delta(u[i], v[i], delta) {
            return false;
        }
    }
    true
}

fn mat_equals_with_delta(u: Mat4f, v: Mat4f, delta: f32) -> bool {
    for y in 0..4 {
        if !vec_equals_with_delta(u[y], v[y], delta) {
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
    assert!(vec_equals_with_delta(u + v, Vect3f::new([5.0, -5.2, 2.6]), 0.0001));
    assert!(vec_equals_with_delta(u - v, Vect3f::new([-1.0, 3.2, 7.4]), 0.0001));
    assert!(vec_equals_with_delta(u * 2.0, Vect3f::new([4.0, -2.0, 10.0]), 0.0001));
    assert!(vec_equals_with_delta(u / 2.0, Vect3f::new([1.0, -0.5, 2.5]), 0.0001));
}

#[test]
fn vector_funcs() {
    let u = Vect3f::new([3.0, -3.0, 1.0]);
    let v = Vect3f::new([4.0, 9.0, 2.0]);
    assert!(Vect3f::dot(u, v) == -13.0);
    assert!(vec_equals_with_delta(Vect3f::cross(u, v), Vect3f::new([-15.0, -2.0, 39.0]), 0.0001));
}

#[test]
fn box_funcs() {
    let mut boxe = Box::<3, f32>::new();
    boxe.add(Vect3f::new([1.0, -2.0, 3.5]));
    boxe.add(Vect3f::new([-1.0, 0.0, 1.5]));
    boxe.add(Vect3f::new([-3.0, 4.2, 4.0]));

    assert!(boxe.min() == Vect3f::new([-3.0, -2.0, 1.5]));
    assert!(boxe.max() == Vect3f::new([1.0, 4.2, 4.0]));
    assert!(boxe.extent() == Vect3f::new([4.0, 6.2, 2.5]));
    assert!(boxe.contains(Vect3f::new([-3.0, -2.0, 1.5])) == true);
    assert!(boxe.contains(Vect3f::new([0.0, 0.0, 0.0])) == false);
    assert!(boxe.contains(Vect3f::new([-1.0, 1.0, 2.0])) == true);
}

#[test]
fn matrix_determinant() {
    let mat3 = Mat3f::from_data([
        1.0, 4.0, -4.2,
        2.0, -1.0, 2.2,
        -2.1, 4.3, 10.0,
    ]);
    assert!(equals_with_delta(mat3.determinant(), -145.24, 0.001));

    let mat4 = Mat4f::from_data([
        1.0, 4.0, -4.2, 3.0,
        2.0, -1.0, 2.2, 2.1,
        -2.1, 4.3, 10.0, 0.0,
        0.0, 42.0, -1.0, 3.2,
    ]);
    assert!(equals_with_delta(mat4.determinant(), 2526.11, 0.01));
}

#[test]
fn matrix_inverse() {
    let matrix = Mat4f::from_data([
        1.0, 4.0, -4.2, 3.0,
        2.0, -1.0, 2.2, 2.1,
        -2.1, 4.3, 10.0, 0.0,
        0.0, 42.0, -1.0, 3.2,
    ]);
    let expected = Mat4f::from_data([
        -0.37738, 0.430349, -0.246039, 0.071377,
        -0.0329337, 0.00398875, -0.0118839, 0.0282577,
        -0.0650883, 0.0886582, 0.0534419, 0.00283836,
        0.411915, -0.0246466, 0.172677, -0.0574956,
    ]);
    assert!(mat_equals_with_delta(matrix.inverse(), expected, 0.0001));
}

#[test]
fn matrix_translation() {
    let translation = Vect3f::new([3.0, -3.0, 1.0]);
    let matrix = Mat4f::translation(translation);
    let expected = Mat4f::from_data([
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        translation[0], translation[1], translation[2], 1.0,
    ]);
    assert!(mat_equals_with_delta(matrix, expected, 0.0001));
}

#[test]
fn matrix_rotation() {
    let angle = 30.0_f32.to_radians();
    {
        let matrix_around_x = Mat4f::rotation_around_x(angle);
        let expected = Mat4f::from_data([
            1.0, 0.0, 0.0, 0.0,
            0.0, angle.cos(), angle.sin(), 0.0,
            0.0, -angle.sin(), angle.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]);
        assert!(mat_equals_with_delta(matrix_around_x, expected, 0.0001));
    }
    {
        let matrix_around_y = Mat4f::rotation_around_y(angle);
        let expected = Mat4f::from_data([
            angle.cos(), 0.0, -angle.sin(), 0.0,
            0.0, 1.0, 0.0, 0.0,
            angle.sin(), 0.0, angle.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]);
        assert!(mat_equals_with_delta(matrix_around_y, expected, 0.0001));
    }
    {
        let matrix_around_z = Mat4f::rotation_around_z(angle);
        let expected = Mat4f::from_data([
            angle.cos(), angle.sin(), 0.0, 0.0,
            -angle.sin(), angle.cos(), 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]);
        assert!(mat_equals_with_delta(matrix_around_z, expected, 0.0001));
    }
}

#[test]
fn matrix_op() {
    let mat1 = Mat4f::from_data([
        1.0, 4.0, -4.2, 3.0,
        2.0, 0.0, 2.2, 2.1,
        -2.1, 4.3, 10.0, 0.0,
        0.0, 42.0, -1.0, 3.2,
    ]);
    let mat2 = Mat4f::from_data([
        4.2, 1.0, -0.1, 2.0,
        6.2, 5.4, -1.0, 4.1,
        2.9, 6.2, -3.0, 2.4,
        -2.4, 0.0, 4.2, -0.5,
    ]);
    {
        let result = mat1.clone() * Vect3f::new([42.0, 4.2, -42.0]);
        let expected = Vect3f::new([138.6, 29.4, -588.16]);
        assert!(vec_equals_with_delta(result, expected, 0.0001));
    }
    {
        let result = mat1.clone() * Vect::<4, f32>::new([42.0, 4.2, -42.0, 1.0]);
        let expected = Vect::<4, f32>::new([138.6, 29.4, -588.16, 138.02]);
        assert!(vec_equals_with_delta(result, expected, 0.0001));
    }
    {
        let result = mat1.clone() * mat2;
        let expected = Mat4f::from_data([
            6.41, 100.37, -18.44, 21.1,
            19.1, 192.7, -28.26, 43.06,
            21.6, 99.5, -30.94, 29.4,
            -11.22, -12.54, 52.58, -8.8,
        ]);
        assert!(mat_equals_with_delta(result, expected, 0.0001));
    }
}

#[test]
fn orthographic_matrix() {
    let left = 0.0;
    let right = 800.0;
    let bottom = 0.0;
    let top = 600.0;
    let matrix = Mat4f::orthographic(left, right, bottom, top);

    let expected = Mat4f::from_data([
        2.0 / 800.0, 0.0, 0.0, 0.0,
        0.0, 2.0 / 600.0, 0.0, 0.0,
        0.0, 0.0, -1.0, 0.0,
        -1.0, -1.0, 0.0, 1.0,
    ]);

    assert!(mat_equals_with_delta(matrix, expected, 0.0001));
}

#[test]
fn orthographic_with_z_matrix() {
    let left = 0.0;
    let right = 800.0;
    let bottom = 0.0;
    let top = 600.0;
    let z_near = 0.1;
    let z_far = 1000.0;
    let matrix = Mat4f::orthographic_with_z(left, right, bottom, top, z_near, z_far);

    let expected = Mat4f::from_data([
        2.0 / 800.0, 0.0, 0.0, 0.0,
        0.0, 2.0 / 600.0, 0.0, 0.0,
        0.0, 0.0, -2.0 / 999.9, 0.0,
        -1.0, -1.0, -1.0002, 1.0,
    ]);

    assert!(mat_equals_with_delta(matrix, expected, 0.0001));
}

#[test]
fn perspective_matrix() {
    let fov = 80.0_f32.to_radians();
    let aspect_ratio: f32 = 8.0 / 6.0;
    let z_near = 0.1;
    let z_far = 1000.0;
    let matrix = Mat4f::perspective(fov, aspect_ratio, z_near, z_far);

    let expected = Mat4f::from_data([
        0.893815, 0.0, 0.0, 0.0,
        0.0, 1.19175, 0.0, 0.0,
        0.0, 0.0, -1.0002, -1.0,
        0.0, 0.0, -0.20002, 0.0,
    ]);

    assert!(mat_equals_with_delta(matrix, expected, 0.0001));
}

#[test]
fn view_matrix() {
    let eye = Vect3f::new([1.0, 0.0, 0.0]);
    let target = Vect3f::new([0.0, 0.0, 0.0]);
    let up = Vect3f::new([0.0, 1.0, 0.0]);
    let matrix = Mat4f::look_at(eye, target, up);

    let expected = Mat4f::from_data([
        0.0, 0.0, 1.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        -1.0, 0.0, 0.0, 0.0,
        0.0, 0.0, -1.0, 1.0,
    ]);

    assert!(mat_equals_with_delta(matrix, expected, 0.0001));
}
