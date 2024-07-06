use super::traits::MathsUsable;
use super::vector::Vect;

#[derive(Clone, Hash)]
pub struct Box<const N: usize, T: MathsUsable> {
    min: Vect<N, T>,
    max: Vect<N, T>,
}

impl<const N: usize, T: MathsUsable> Box<N, T> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            min: Vect::<N, T>::from_slice(vec![T::max_value(); N].as_slice()),
            max: Vect::<N, T>::from_slice(vec![T::min_value(); N].as_slice()),
        }
    }

    pub fn zero() -> Self {
        Self {
            min: Vect::<N, T>::zero(),
            max: Vect::<N, T>::zero(),
        }
    }

    pub fn from_min_max(min: Vect<N, T>, max: Vect<N, T>) -> Self {
        Self {
            min: min,
            max: max,
        }
    }

    pub fn is_empty(&self) -> bool {
        *self == Self::new()
    }

    pub fn min(&self) -> Vect<N, T> {
        self.min
    }

    pub fn max(&self) -> Vect<N, T> {
        self.max
    }

    pub fn center(&self) -> Vect<N, T> {
        assert!(!self.is_empty());
        (self.min + self.max) / T::from(2)
    }

    pub fn extent(&self) -> Vect<N, T> {
        assert!(!self.is_empty());
        self.max - self.min
    }

    // TODO Should return an array, not a Vec, since size is static and can be deduced from N,
    // But I can't figure how to write array size with something that is not "currently not permitted"...
    pub fn corners(&self) -> Vec<Vect<N, T>> {
        let corners_count = 2_u32.pow(N as u32) as usize;
        let mut result = vec![Vect::<N, T>::zero(); corners_count];
        for n in 0..N {
            for i in 0..corners_count {
                result[i][n] = if i / 2_usize.pow(n as u32) % 2 == 0 { self.min[n] } else { self.max[n] };
            }
        }
        result
    }

    pub fn subdivide(&self) -> Vec<Box<N, T>> {
        let center = self.center();
        let subdivision_count = 2_u32.pow(N as u32) as usize;
        let mut result = vec![Box::<N, T>::zero(); subdivision_count];
        for n in 0..N {
            for i in 0..subdivision_count {
                let low = i / 2_usize.pow(n as u32) % 2 == 0;
                result[i].min[n] = if low { self.min[n] } else { center[n] };
                result[i].max[n] = if low { center[n] } else { self.max[n] };
            }
        }
        result

    }

    pub fn contains(&self, point: Vect<N, T>) -> bool {
        for i in 0..N {
            if point[i] < self.min[i] || point[i] > self.max[i] {
                return false;
            }
        }
        true
    }

    pub fn contains_box(&self, other: &Box<N, T>) -> bool {
        self.contains(other.min()) && self.contains(other.max())
    }

    pub fn intersects(&self, other: &Box<N, T>) -> bool {
        for i in 0..N {
            if self.min[i] > other.max[i] || self.max[i] < other.min[i] {
                return false;
            }
        }
        true
    }

    pub fn add(&mut self, point: Vect<N, T>) {
        for i in 0..N {
            self.min[i] = num_traits::clamp_max(self.min[i], point[i]);
            self.max[i] = num_traits::clamp_min(self.max[i], point[i]);
        }
    }
}

impl<T: MathsUsable> Box<1, T> {
    pub fn add_value(&mut self, value: T) {
        self.add(Vect::<1, T>::new([value]));
    }
}

impl<const N: usize, T: MathsUsable> Eq for Box<N, T> {}
impl<const N: usize, T: MathsUsable> PartialEq for Box<N, T> {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.max == other.max
    }
}

pub type Box1f = Box<1, f32>;
pub type Box3f = Box<3, f32>;
pub type Box3i = Box<3, i32>;
