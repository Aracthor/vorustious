pub trait Limited {
    fn min() -> Self;
    fn max() -> Self;
}

macro_rules! limited_impl {
    ($type:ty, $min:expr, $max:expr) => {
        impl Limited for $type {
            fn min() -> Self { $min }
            fn max() -> Self { $max }
        }
    };
}

limited_impl!(f32, f32::MIN, f32::MAX);
limited_impl!(i32, i32::MIN, i32::MAX);

pub trait MathsUsable:
    Copy +
    From<i8> +
    Limited +
    std::cmp::PartialEq<Self> +
    std::cmp::PartialOrd<Self> +
    std::ops::Neg<Output = Self> +
    std::ops::Add<Self, Output = Self> +
    std::ops::Sub<Self, Output = Self> +
    std::ops::Mul<Self, Output = Self> +
    std::ops::Div<Self, Output = Self> +
    std::ops::AddAssign<Self> +
    std::ops::SubAssign<Self> +
    std::ops::MulAssign<Self> +
    std::ops::DivAssign<Self> +
    std::fmt::Display
{}

impl<T:
    Copy +
    From<i8> +
    Limited +
    std::cmp::PartialEq<Self> +
    std::cmp::PartialOrd<Self> +
    std::ops::Neg<Output = Self> +
    std::ops::Add<Self, Output = Self> +
    std::ops::Sub<Self, Output = Self> +
    std::ops::Mul<Self, Output = Self> +
    std::ops::Div<Self, Output = Self> +
    std::ops::AddAssign<Self> +
    std::ops::SubAssign<Self> +
    std::ops::MulAssign<Self> +
    std::ops::DivAssign<Self> +
    std::fmt::Display> MathsUsable
for T {}
