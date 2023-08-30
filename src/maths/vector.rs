use core::ops::Index;
use core::ops::IndexMut;

use super::traits::MathsUsable;

#[derive(Clone, Copy)]
pub struct Vect<const N: usize, T: MathsUsable> {
    data: [T; N],
}

impl<const N: usize, T: MathsUsable> Vect<N, T> {
    pub fn new(data: [T; N]) -> Vect<N, T> {
        Self { data: data }
    }

    pub fn from_slice(data: &[T]) -> Vect<N, T> {
        let mut result = Self::zero();
        for i in 0..N {
            result[i] = data[i];
        }
        result
    }

    pub fn zero() -> Self {
        Self {data: [T::from(0.0); N] }
    }

    pub fn dot(u: Self, v: Self) -> T {
        let mut result: T = 0.0.into();
        for i in 0..N {
            result += u.data[i] * v.data[i];
        }
        result
    }

    pub fn length_sq(&self) -> T {
        let mut result: T = 0.0.into();
        for d in self.data {
            result += d * d;
        }
        result
    }

    pub fn length(&self) -> T {
        // TODO we should avoid casting here...
        self.length_sq().into().sqrt().into()
    }

    pub fn normalize(&self) -> Vect<N, T> {
        let length = self.length();
        assert!(length != 0.0.into());
        return *self / length;
    }
}

impl<T: MathsUsable> Vect<3, T> {
    pub fn cross(u: Self, v: Self) -> Self{
        Self {data: [
            u[1] * v[2] - v[1] * u[2],
            u[2] * v[0] - v[2] * u[0],
            u[0] * v[1] - v[0] * u[1],
        ]}
    }
}


impl<const N: usize, T: MathsUsable> Index<usize> for Vect<N, T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const N: usize, T: MathsUsable> IndexMut<usize> for Vect<N, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const N: usize, T: MathsUsable> Eq for Vect<N, T> {}
impl<const N: usize, T: MathsUsable> PartialEq for Vect<N, T> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..N {
            if self.data[i] != other.data[i] {
                return false;
            }
        }
        true
    }
}

impl<const N: usize, T: MathsUsable> std::ops::Neg for Vect<N, T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut result = self;
        for d in &mut result.data {
            *d = -*d;
        }
        result
    }
}

impl<const N: usize, T: MathsUsable> std::ops::AddAssign<Self> for Vect<N, T> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.data[i] += rhs.data[i];
        }
    }
}

impl<const N: usize, T: MathsUsable> std::ops::SubAssign<Self> for Vect<N, T> {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.data[i] -= rhs.data[i];
        }
    }
}

impl<const N: usize, T: MathsUsable> std::ops::Add<Self> for Vect<N, T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut result = self;
        for i in 0..N {
            result.data[i] += other.data[i];
        }
        result
    }
}

impl<const N: usize, T: MathsUsable> std::ops::Sub<Self> for Vect<N, T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut result = self;
        for i in 0..N {
            result.data[i] -= other.data[i];
        }
        result
    }
}

impl<const N: usize, T: MathsUsable> std::ops::Mul<T> for Vect<N, T> {
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        let mut result = self;
        for d in &mut result.data {
            *d *= other;
        }
        result
    }
}

impl<const N: usize, T: MathsUsable> std::ops::Div<T> for Vect<N, T> {
    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        let mut result = self;
        for d in &mut result.data {
            *d /= other;
        }
        result
    }
}

impl<const N: usize, T: MathsUsable> std::fmt::Display for Vect<N, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        assert!(N > 0);
        let mut index = 0;
        let mut result = write!(f, "{}", self.data[index]);
        while index + 1 < N && !result.is_err() {
            index += 1;
            result = write!(f, ", {}", self.data[index]);
        }
        result
    }
}

pub type Vect3f = Vect<3, f32>;
