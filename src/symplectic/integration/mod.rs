mod neri;

use super::State;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StepSize {
    UseDefault,
    Step(f64),
}

pub trait Integrator<S: State> {
    fn propagate_in_place<D1, D2>(
        &mut self,
        start: &mut S,
        pos_diff_eq: D1,
        momentum_diff_eq: D2,
        step: StepSize,
    ) where
        D1: Fn(&S) -> S::PositionDerivative,
        D2: Fn(&S) -> S::MomentumDerivative;

    fn propagate<D1, D2>(
        &mut self,
        start: &S,
        pos_diff_eq: D1,
        momentum_diff_eq: D2,
        step: StepSize,
    ) -> S
    where
        D1: Fn(&S) -> S::PositionDerivative,
        D2: Fn(&S) -> S::MomentumDerivative,
    {
        let mut result = start.clone();
        self.propagate_in_place(&mut result, pos_diff_eq, momentum_diff_eq, step);
        result
    }
}

pub use neri::*;
