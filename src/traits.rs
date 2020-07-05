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

#[cfg(feature = "nalgebra")]
mod nalgebra_impl {
    use std::ops::{Add, Div, Mul, Neg, Sub};

    use super::StateDerivative;

    use nalgebra::{storage::Storage, Dim, Vector};

    impl<D: Dim, S> StateDerivative for Vector<f64, D, S>
    where
        Self: Clone
            + Add<Self, Output = Self>
            + Sub<Self, Output = Self>
            + Mul<f64, Output = Self>
            + Div<f64, Output = Self>
            + Neg<Output = Self>,
        S: Storage<f64, D>,
    {
        fn abs(&self) -> f64 {
            self.dot(self).sqrt()
        }
    }
}
