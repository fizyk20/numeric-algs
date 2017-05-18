use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait State: Clone {
    type Derivative: StateDerivative;
    fn shift(&self, dir: Self::Derivative, amount: f64) -> Self;
}

pub trait StateDerivative
    : Clone + Sized + Add<Self, Output = Self> + Sub<Self, Output = Self> + Mul<f64, Output=Self> +
      Div<f64, Output=Self> + Neg<Output=Self>
{
    fn abs(&self) -> f64;
}
