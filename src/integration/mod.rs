mod dormand_prince;
mod rk4;
use traits::State;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StepSize {
    UseDefault,
    Step(f64),
}

pub trait Integrator<S: State> {
    fn propagate_in_place<D>(&mut self, start: &mut S, diff_eq: D, step: StepSize)
    where
        D: Fn(&S) -> S::Derivative;

    fn propagate<D>(&mut self, start: &S, diff_eq: D, step: StepSize) -> S
    where
        D: Fn(&S) -> S::Derivative,
    {
        let mut result = start.clone();
        self.propagate_in_place(&mut result, diff_eq, step);
        result
    }
}

pub use self::dormand_prince::DPIntegrator;
pub use self::rk4::RK4Integrator;
