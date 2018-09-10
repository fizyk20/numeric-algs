use super::{DiffEq, Integrator, StepSize};
use traits::State;

pub struct RK4Integrator {
    default_step: f64,
}

impl RK4Integrator {
    pub fn new(step_size: f64) -> Self {
        RK4Integrator {
            default_step: step_size,
        }
    }

    pub fn set_default_step(&mut self, step: f64) {
        self.default_step = step;
    }
}

impl<S: State> Integrator<S> for RK4Integrator {
    fn propagate<D>(&mut self, start: S, diff_eq: D, step_size: StepSize) -> S
        where D: DiffEq<S>
    {
        let h = match step_size {
            StepSize::UseDefault => self.default_step,
            StepSize::Step(x) => x,
        };

        let k1 = diff_eq.derivative(start.clone());
        let k2 = diff_eq.derivative(start.shift(k1.clone(), h / 2.0));
        let k3 = diff_eq.derivative(start.shift(k2.clone(), h / 2.0));
        let k4 = diff_eq.derivative(start.shift(k3.clone(), h));

        start.shift(k1 + k2 * 2.0 + k3 * 2.0 + k4, h / 6.0)
    }
}
