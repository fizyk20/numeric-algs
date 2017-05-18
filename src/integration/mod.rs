mod rk4;
mod dormand_prince;
use traits::State;

pub trait DiffEq<S: State> {
    fn derivative(&self, state: S) -> S::Derivative;
}

pub enum StepSize {
    UseDefault,
    Step(f64),
}

pub trait Integrator<S: State> {
    fn propagate<D>(&mut self, start: S, diff_eq: D, step: StepSize) -> S where D: DiffEq<S>;
}

pub use self::dormand_prince::DPIntegrator;
pub use self::rk4::RK4Integrator;
