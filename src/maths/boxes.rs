use super::traits::MathsUsable;
use super::vector::Vect;

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

    pub fn from_min_max(min: Vect<N, T>, max: Vect<N, T>) -> Self {
        Self {
            min: min,
            max: max,
        }
    }

    pub fn min(&self) -> Vect<N, T> {
        self.min
    }

    pub fn max(&self) -> Vect<N, T> {
        self.max
    }

    pub fn extent(&self) -> Vect<N, T> {
        self.max - self.min
    }

    pub fn contains(&self, point: Vect<N, T>) -> bool {
        for i in 0..N {
            if point[i] < self.min[i] || point[i] > self.max[i] {
                return false;
            }
        }
        true
    }

    #[cfg(test)]
    pub fn add(&mut self, point: Vect<N, T>) {
        for i in 0..N {
            self.min[i] = num_traits::clamp_max(self.min[i], point[i]);
            self.max[i] = num_traits::clamp_min(self.max[i], point[i]);
        }
    }
}

impl<const N: usize, T: MathsUsable> Eq for Box<N, T> {}
impl<const N: usize, T: MathsUsable> PartialEq for Box<N, T> {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.max == other.max
    }
}
