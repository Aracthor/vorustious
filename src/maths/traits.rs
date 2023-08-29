trait_set::trait_set! {
    pub trait MathsUsable =
        Copy +
        From<f32> +
        Into<f32> +
        std::cmp::PartialEq<Self> +
        std::ops::Neg<Output = Self> +
        std::ops::Add<Self, Output = Self> +
        std::ops::Sub<Self, Output = Self> +
        std::ops::Mul<Self, Output = Self> +
        std::ops::Div<Self, Output = Self> +
        std::ops::AddAssign<Self> +
        std::ops::SubAssign<Self> +
        std::ops::DivAssign<Self>
}
