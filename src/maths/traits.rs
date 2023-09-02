pub trait MathsUsable:
    Copy +
    From<i8> +
    num_traits::Bounded +
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
    num_traits::Bounded +
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
