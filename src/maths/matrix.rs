use super::traits::MathsUsable;
use super::vector::Vect;

use core::ops::Index;
use core::ops::IndexMut;

#[derive(Clone, Copy)]
pub struct Mat<const N: usize, T: MathsUsable> {
    data: [Vect<N, T>; N],
}

impl<const N: usize, T: MathsUsable> Mat<N, T> {
    #[allow(dead_code)]
    pub fn from_data<const N2: usize>(data: [T; N2]) -> Self {
        assert!(N2 == N * N);
        let mut result = Self::identity();
        for y in 0..N {
            result[y] = Vect::<N, T>::from_slice(&data[y * N..(y + 1) * N]);
        }
        result
    }

    pub fn identity() -> Self {
        let mut result = Self { data: [Vect::<N, T>::zero(); N] };
        for i in 0..N {
            result[i][i] = 1.0.into();
        }
        result
    }

    pub fn data_as_ptr(&self) -> *const T {
        assert!(std::mem::size_of_val(self) == std::mem::size_of::<f32>() * 16);
        self.data.as_ptr().cast()
    }
}

impl<T: MathsUsable> Mat<4, T> {
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
}

// Specific to f32 type because of cos(), sin() and tan() function that can be called only from floating types...
impl<const N: usize> Mat<N, f32> {
    #[allow(dead_code)]
    pub fn rotation_around_x(angle: f32) -> Self {
        assert!(N >= 3);
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
        assert!(N >= 3);
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
        assert!(N >= 3);
        let cos = angle.cos();
        let sin = angle.sin();
        let mut result = Self::identity();
        result[0][0] = cos;
        result[0][1] = sin;
        result[1][0] = -sin;
        result[1][1] = cos;
        result
    }
}

impl Mat<4, f32> {
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

impl<const N: usize, T: MathsUsable> Index<usize> for Mat<N, T> {
    type Output = Vect<N, T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const N: usize, T: MathsUsable> IndexMut<usize> for Mat<N, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const N: usize, T: MathsUsable> std::ops::Mul<Self> for Mat<N, T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut product = Self::identity();
        for y in 0..4 {
            for x in 0..4 {
                let mut result: T = 0.0.into();
                for i in 0..4 {
                    result += self[i][y] * rhs[x][i];
                }
                product[x][y] = result;
            }
        }
        product
    }
}

impl<const N: usize, const VN: usize, T: MathsUsable> std::ops::Mul<Vect<VN, T>> for Mat<N, T> {
    type Output = Vect<VN, T>;

    fn mul(self, rhs: Vect<VN, T>) -> Self::Output {
        assert!(N >= VN);
        let mut result = Self::Output::zero();
        for y in 0..VN {
            for x in 0..N {
                let r_value = if x >= VN { 1.0.into() } else { rhs[x] };
                result[y] += self[x][y] * r_value;
            }
        }
        result
    }
}

pub type Mat4f = Mat<4, f32>;
