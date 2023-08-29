use super::vector::MathsUsable;

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
