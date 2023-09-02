use super::traits::MathsUsable;
use super::vector::Vect;

use core::ops::Index;
use core::ops::IndexMut;

#[derive(Clone)]
pub struct Mat<const N: usize, T: MathsUsable> {
    data: [Vect<N, T>; N],
}

impl<const N: usize, T: MathsUsable> Mat<N, T> {
    pub fn from_data<const N2: usize>(data: [T; N2]) -> Self {
        assert!(N2 == N * N);
        let mut result = Self::identity();
        for y in 0..N {
            result[y] = Vect::<N, T>::from_slice(&data[y * N..(y + 1) * N]);
        }
        result
    }

    pub fn zero() -> Self {
        Self { data: [Vect::<N, T>::zero(); N] }
    }

    pub fn identity() -> Self {
        let mut result = Self::zero();
        for i in 0..N {
            result[i][i] = T::from(1);
        }
        result
    }

    pub fn data_as_ptr(&self) -> *const T {
        assert!(std::mem::size_of_val(self) == std::mem::size_of::<f32>() * 16);
        self.data.as_ptr().cast()
    }
}

impl<T: MathsUsable> Mat<3, T> {
    pub fn determinant(&self) -> T {
        self[0][0] * self[1][1] * self[2][2] +
        self[0][1] * self[1][2] * self[2][0] +
        self[0][2] * self[1][0] * self[2][1] -
        self[0][2] * self[1][1] * self[2][0] -
        self[0][1] * self[1][0] * self[2][2] -
        self[0][0] * self[1][2] * self[2][1]
    }
}

impl<T: MathsUsable> Mat<4, T> {
    pub fn determinant(&self) -> T {
        let det1 = Mat::<3, T>::from_data([
            self[1][1], self[1][2], self[1][3],
            self[2][1], self[2][2], self[2][3],
            self[3][1], self[3][2], self[3][3],
        ]).determinant() * self[0][0];
        let det2 = Mat::<3, T>::from_data([
            self[0][1], self[0][2], self[0][3],
            self[2][1], self[2][2], self[2][3],
            self[3][1], self[3][2], self[3][3],
        ]).determinant() * self[1][0];
        let det3 = Mat::<3, T>::from_data([
            self[0][1], self[0][2], self[0][3],
            self[1][1], self[1][2], self[1][3],
            self[3][1], self[3][2], self[3][3],
        ]).determinant() * self[2][0];
        let det4 = Mat::<3, T>::from_data([
            self[0][1], self[0][2], self[0][3],
            self[1][1], self[1][2], self[1][3],
            self[2][1], self[2][2], self[2][3],
        ]).determinant() * self[3][0];

        det1 - det2 + det3 - det4
    }

    #[allow(dead_code)]
    pub fn inverse(&self) -> Self {
        let oo_det: T = T::from(1) / self.determinant();
        let mut sign: T = T::from(1);
        let mut result = Self::zero();
        for x in 0..4 {
            for y in 0..4 {
                let mut submatrix = Mat::<3, T>::zero();
                for sub_x in 0..3 {
                    for sub_y in 0..3 {
                        let x_to_get = if sub_x >= x { sub_x + 1 } else { sub_x };
                        let y_to_get = if sub_y >= y { sub_y + 1 } else { sub_y };
                        submatrix[sub_x][sub_y] = self[x_to_get][y_to_get];
                    }
                }
                result[y][x] = oo_det * submatrix.determinant() * sign;
                sign *= T::from(-1);
            }
            sign *= T::from(-1);
        }
        result
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
        let two_as_t: T = T::from(2);
        result[0][0] = two_as_t / (right - left);
        result[1][1] = two_as_t / (top - bottom);
        result[2][2] = -two_as_t / (z_far - z_near);
        result[3][0] = -(right + left) / (right - left);
        result[3][1] = -(top + bottom) / (top - bottom);
        result[3][2] = -(z_far + z_near) / (z_far - z_near);
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

    pub fn look_at(eye: Vect<3, f32>, target: Vect<3, f32>, up: Vect<3, f32>) -> Self {
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
                let mut result: T = T::from(0);
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
                let r_value = if x >= VN { T::from(1) } else { rhs[x] };
                result[y] += self[x][y] * r_value;
            }
        }
        result
    }
}

#[allow(dead_code)]
pub type Mat3f = Mat<3, f32>;
pub type Mat4f = Mat<4, f32>;
