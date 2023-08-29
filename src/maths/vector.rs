use core::ops::Index;

trait_set::trait_set! {
    pub trait MathsUsable =
        Copy +
        From<f32> +
        Into<f32> +
        std::cmp::PartialEq<Self> +
        std::ops::Neg<Output = Self> +
        std::ops::Sub<Self, Output = Self> +
        std::ops::Mul<Self, Output = Self> +
        std::ops::Div<Self, Output = Self> +
        std::ops::AddAssign<Self> +
        std::ops::SubAssign<Self> +
        std::ops::DivAssign<Self>
}

#[derive(Clone)]
pub struct Vect<const N: usize, T: MathsUsable> {
    data: [T; N],
}

impl<const N: usize, T: MathsUsable> Vect<N, T> {
    pub fn new(data: [T; N]) -> Vect<N, T> {
        Vect::<N, T> { data: data }
    }

    pub fn dot(u: Self, v: Self) -> T {
        let mut result: T = 0.0.try_into().unwrap();
        for i in 0..N {
            result += u.data[i] * v.data[i];
        }
        result
    }

    pub fn length_sq(&self) -> T {
        let mut result: T = 0.0.try_into().unwrap();
        for d in self.data {
            result += d * d;
        }
        result
    }

    pub fn length(&self) -> T {
        // TODO we should avoid casting here...
        self.length_sq().into().sqrt().try_into().unwrap()
    }

    pub fn normalize(&self) -> Vect<N, T> {
        let length = self.length();
        assert!(length != 0.0.try_into().unwrap());
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


impl<const N: usize, T: MathsUsable> Copy for Vect<N, T> {}

impl<const N: usize, T: MathsUsable> Index<usize> for Vect<N, T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
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

pub type Vect3f = Vect<3, f32>;
