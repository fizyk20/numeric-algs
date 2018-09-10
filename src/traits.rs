use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait State: Clone {
    type Derivative: StateDerivative;
    fn shift(&self, dir: &Self::Derivative, amount: f64) -> Self {
        let mut result = self.clone();
        result.shift_in_place(dir, amount);
        result
    }

    fn shift_in_place(&mut self, dir: &Self::Derivative, amount: f64);
}

pub trait StateDerivative:
    Clone
    + Sized
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<f64, Output = Self>
    + Div<f64, Output = Self>
    + Neg<Output = Self>
{
    fn abs(&self) -> f64;
}
