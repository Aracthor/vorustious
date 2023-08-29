use super::traits::MathsUsable;
use super::vector::Vect;

use core::ops::Index;
pub struct Mat4<T: MathsUsable> {
    data: [T; 16],
}

impl<T: MathsUsable> Mat4<T> {
    pub fn identity() -> Self {
        Self {
            data: [
                1.0.try_into().unwrap(), 0.0.try_into().unwrap(), 0.0.try_into().unwrap(), 0.0.try_into().unwrap(),
                0.0.try_into().unwrap(), 1.0.try_into().unwrap(), 0.0.try_into().unwrap(), 0.0.try_into().unwrap(),
                0.0.try_into().unwrap(), 0.0.try_into().unwrap(), 1.0.try_into().unwrap(), 0.0.try_into().unwrap(),
                0.0.try_into().unwrap(), 0.0.try_into().unwrap(), 0.0.try_into().unwrap(), 1.0.try_into().unwrap(),
            ],
        }
    }

    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, z_near: f32, z_far: f32) -> Self {
        let mut result = Self::identity();
        result.set(0, 0, (2.0 / (right - left)).try_into().unwrap());
		result.set(1, 1, (2.0 / (top - bottom)).try_into().unwrap());
		result.set(2, 2, (-2.0 / (z_far - z_near)).try_into().unwrap());
		result.set(0, 3, (-(right + left) / (right - left)).try_into().unwrap());
		result.set(1, 3, (-(top + bottom) / (top - bottom)).try_into().unwrap());
		result.set(2, 3, (-(z_far + z_near) / (z_far - z_near)).try_into().unwrap());
        result
    }

    pub fn look_at(eye: Vect<3, T>, target: Vect<3, T>, up: Vect<3, T>) -> Self {
        let f = (target - eye).normalize();
        let s = Vect::cross(f, up).normalize();
        let u = Vect::cross(s, f);

        let mut result = Self::identity();
        result.set(0, 0, s[0]);
        result.set(0, 1, s[1]);
        result.set(0, 2, s[2]);
        result.set(1, 0, u[0]);
        result.set(1, 1, u[1]);
        result.set(1, 2, u[2]);
        result.set(2, 0, -f[0]);
        result.set(2, 1, -f[1]);
        result.set(2, 2, -f[2]);
        result.set(0, 3, -Vect::dot(s, eye));
        result.set(1, 3, -Vect::dot(u, eye));
        result.set(2, 3,  Vect::dot(f, eye));
        result
    }

    pub fn set(&mut self, x: usize, y: usize, data: T) {
        assert!(x <= 3);
        assert!(y <= 3);
        self.data[y * 4 + x] = data;
    }

    pub fn data_as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }
}

impl<T: MathsUsable> Index<usize> for Mat4<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

pub type Mat4f = Mat4<f32>;
