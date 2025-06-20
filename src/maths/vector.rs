use core::ops::Index;
use core::ops::IndexMut;
use std::iter::Sum;

use super::traits::MathsUsable;

#[derive(Clone, Copy, Hash)]
pub struct Vect<const N: usize, T: MathsUsable> {
    data: [T; N],
}

impl<const N: usize, T: MathsUsable> Vect<N, T> {
    pub fn new(data: [T; N]) -> Self {
        Self { data: data }
    }

    pub fn all(value: T) -> Self {
        Self { data: [value; N] }
    }

    pub fn from_slice(data: &[T]) -> Self {
        let mut result = Self::zero();
        for i in 0..N {
            result[i] = data[i];
        }
        result
    }

    pub fn zero() -> Self {
        Self {data: [T::from(0); N] }
    }

    pub fn dot(u: Self, v: Self) -> T {
        let mut result: T = T::from(0);
        for i in 0..N {
            result += u.data[i] * v.data[i];
        }
        result
    }

    pub fn length_sq(&self) -> T {
        let mut result = T::from(0);
        for d in self.data {
            result += d * d;
        }
        result
    }

    pub fn data_as_ptr(&self) -> *const T {
        assert!(std::mem::size_of_val(self) == std::mem::size_of::<T>() * N);
        self.data.as_ptr().cast()
    }
}

impl<const N: usize> Vect<N, f32> {

    pub fn length(&self) -> f32 {
        self.length_sq().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        assert!(length != 0.0);
        *self / length
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

impl<const N: usize, T: MathsUsable> Sum for Vect<N, T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |a, b| a + b)
    }
}

impl<'a, const N: usize, T: MathsUsable> Sum<&'a Vect<N, T>> for Vect<N, T> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |a, b| a + *b)
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

impl<const N: usize, T: MathsUsable> std::ops::MulAssign<T> for Vect<N, T> {
    fn mul_assign(&mut self, rhs: T) {
        for d in &mut self.data {
            *d *= rhs;
        }
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

pub type Vect2f = Vect<2, f32>;
pub type Vect3i = Vect<3, i32>;
pub type Vect3f = Vect<3, f32>;
pub type Vect4f = Vect<4, f32>;
