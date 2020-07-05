use super::{Integrator, StepSize};
use crate::traits::State;

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
    fn propagate_in_place<D>(&mut self, start: &mut S, diff_eq: D, step_size: StepSize)
    where
        D: Fn(&S) -> S::Derivative,
    {
        let h = match step_size {
            StepSize::UseDefault => self.default_step,
            StepSize::Step(x) => x,
        };

        let k1 = diff_eq(start);
        let k2 = diff_eq(&start.shift(&k1, h / 2.0));
        let k3 = diff_eq(&start.shift(&k2, h / 2.0));
        let k4 = diff_eq(&start.shift(&k3, h));

        start.shift_in_place(&(k1 + k2 * 2.0 + k3 * 2.0 + k4), h / 6.0);
    }
}
