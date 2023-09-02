use super::traits::MathsUsable;
use super::vector::Vect;

#[derive(Clone, Copy)]
pub struct Segment<const N: usize, T: MathsUsable> {
    pub start: Vect<N, T>,
    pub end: Vect<N, T>,
}

impl<const N: usize, T: MathsUsable> Segment<N, T> {
    pub fn new(start: Vect<N, T>, end: Vect<N, T>) -> Self {
        Self {
            start: start,
            end: end,
        }
    }

    pub fn direction(&self) -> Vect<N, T> {
        self.end - self.start
    }
}

pub type Segm3f = Segment<3, f32>;
