use super::traits::MathsUsable;
use super::vector::Vect;

use core::ops::Index;
use core::ops::IndexMut;

pub struct Mat4<T: MathsUsable> {
    data: [Vect<4, T>; 4],
}

impl<T: MathsUsable> Mat4<T> {
    pub fn from_data(data: [T; 16]) -> Mat4<T> {
        Self {
            data: [
                Vect::new([data[0], data[1], data[2], data[3]]),
                Vect::new([data[4], data[5], data[6], data[7]]),
                Vect::new([data[8], data[9], data[10], data[11]]),
                Vect::new([data[12], data[13], data[14], data[15]]),
            ],
        }
    }

    pub fn identity() -> Self {
        let identity_data: [T; 16] = [
            1.0.into(), 0.0.into(), 0.0.into(), 0.0.into(),
            0.0.into(), 1.0.into(), 0.0.into(), 0.0.into(),
            0.0.into(), 0.0.into(), 1.0.into(), 0.0.into(),
            0.0.into(), 0.0.into(), 0.0.into(), 1.0.into(),
        ];
        Self::from_data(identity_data)
    }

    #[allow(dead_code)]
    pub fn translation(translate: Vect<3, T>) -> Self {
        let mut result = Self::identity();
        result[3][0] = translate[0];
        result[3][1] = translate[1];
        result[3][2] = translate[2];
        result
    }

    #[allow(dead_code)]
    pub fn orthographic(left: T, right: T, bottom: T, top: T, z_near: T, z_far: T) -> Self {
        let mut result = Self::identity();
        let two_as_t: T = 2.0.into();
        result[0][0] = two_as_t / (right - left);
        result[1][1] = two_as_t / (top - bottom);
        result[2][2] = -two_as_t / (z_far - z_near);
        result[3][0] = -(right + left) / (right - left);
        result[3][1] = -(top + bottom) / (top - bottom);
        result[3][2] = -(z_far + z_near) / (z_far - z_near);
        result
    }

    pub fn look_at(eye: Vect<3, T>, target: Vect<3, T>, up: Vect<3, T>) -> Self {
        assert!(eye != target);
        let zaxis = (target - eye).normalize();
        let xaxis = Vect::cross(zaxis, up).normalize();
        let yaxis = Vect::cross(xaxis, zaxis);

        let mut result = Self::identity();
        result[0][0] = xaxis[0];
        result[1][0] = xaxis[1];
        result[2][0] = xaxis[2];
        result[0][1] = yaxis[0];
        result[1][1] = yaxis[1];
        result[2][1] = yaxis[2];
        result[0][2] = -zaxis[0];
        result[1][2] = -zaxis[1];
        result[2][2] = -zaxis[2];
        result[3][0] = -Vect::dot(xaxis, eye);
        result[3][1] = -Vect::dot(yaxis, eye);
        result[3][2] =  Vect::dot(zaxis, eye);
        result
    }

    pub fn data_as_ptr(&self) -> *const T {
        assert!(std::mem::size_of_val(self) == std::mem::size_of::<f32>() * 16);
        self.data.as_ptr().cast()
    }
}

// Specific to f32 type because of cos(), sin() and tan() function that can be called only from floating types...
impl Mat4<f32> {
    #[allow(dead_code)]
    pub fn rotation_around_x(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        let mut result = Self::identity();
        result[1][1] = cos;
        result[1][2] = sin;
        result[2][1] = -sin;
        result[2][2] = cos;
        result
    }

    #[allow(dead_code)]
    pub fn rotation_around_y(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        let mut result = Self::identity();
        result[0][0] = cos;
        result[0][2] = -sin;
        result[2][0] = sin;
        result[2][2] = cos;
        result
    }

    #[allow(dead_code)]
    pub fn rotation_around_z(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        let mut result = Self::identity();
        result[0][0] = cos;
        result[0][1] = sin;
        result[1][0] = -sin;
        result[1][1] = cos;
        result
    }

    pub fn perspective(fov: f32, aspect: f32, z_near: f32, z_far: f32) -> Self {
        assert!(fov > 0.0);
        assert!(aspect > 0.0);
        assert!(z_near > 0.0);
        assert!(z_near < z_far);
        let half_fov_tan = (fov / 2.0).tan();

        let mut result = Self::identity();
        result[0][0] = 1.0 / (aspect * half_fov_tan);
        result[1][1] = 1.0 / half_fov_tan;
        result[2][2] = -(z_far + z_near) / (z_far - z_near);
        result[2][3] = -1.0;
        result[3][2] = -(2.0 * z_near * z_far) / (z_far - z_near);
        result[3][3] = 0.0;
        result
    }
}

impl<T: MathsUsable> Index<usize> for Mat4<T> {
    type Output = Vect<4, T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: MathsUsable> IndexMut<usize> for Mat4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

pub type Mat4f = Mat4<f32>;
