use super::matrix::Mat4f;
use super::vector::Vect3f;

fn equals_with_delta(u: Vect3f, v: Vect3f, delta: f32) -> bool {
    (u[0] - v[0]).abs() < delta && (u[1] - v[1]).abs() < delta && (u[2] - v[2]).abs() < delta
}

fn mat_equals_with_delta(u: Mat4f, v: Mat4f, delta: f32) -> bool {
    for x in 0..4 {
        for y in 0..4 {
            if (u[x][y] - v[x][y]).abs() >= delta {
                return false;
            }
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
    assert!(equals_with_delta(u + v, Vect3f::new([5.0, -5.2, 2.6]), 0.0001));
    assert!(equals_with_delta(u - v, Vect3f::new([-1.0, 3.2, 7.4]), 0.0001));
    assert!(equals_with_delta(u * 2.0, Vect3f::new([4.0, -2.0, 10.0]), 0.0001));
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
    let result = mat1 * mat2;
    let expected = Mat4f::from_data([
        6.41, 100.37, -18.44, 21.1,
        19.1, 192.7, -28.26, 43.06,
        21.6, 99.5, -30.94, 29.4,
        -11.22, -12.54, 52.58, -8.8,
    ]);
    assert!(mat_equals_with_delta(result, expected, 0.0001));
}

#[test]
fn orthographic_matrix() {
    let left = 0.0;
    let right = 800.0;
    let bottom = 0.0;
    let top = 600.0;
    let z_near = 0.1;
    let z_far = 1000.0;
    let matrix = Mat4f::orthographic(left, right, bottom, top, z_near, z_far);

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
