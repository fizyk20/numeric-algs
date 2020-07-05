use super::{Integrator, StepSize};
use crate::traits::{State, StateDerivative};
use std::mem;

pub struct DPIntegrator<S: State> {
    default_step: f64,
    original_default_step: f64,
    max_err: f64,
    min_step: f64,
    max_step: f64,
    last_derivative: Option<S::Derivative>,
}

impl<S: State> DPIntegrator<S> {
    pub fn new(default_step: f64, min_step: f64, max_step: f64, max_err: f64) -> Self {
        DPIntegrator {
            default_step: default_step,
            original_default_step: default_step,
            min_step: min_step,
            max_step: max_step,
            max_err: max_err,
            last_derivative: None,
        }
    }

    pub fn reset_default_step(&mut self) {
        self.default_step = self.original_default_step;
    }

    pub fn reset(&mut self) {
        self.last_derivative = None;
        self.reset_default_step();
    }
}

impl<S: State> Integrator<S> for DPIntegrator<S> {
    fn propagate_in_place<D>(&mut self, start: &mut S, diff_eq: D, step_size: StepSize)
    where
        D: Fn(&S) -> S::Derivative,
    {
        let h = match step_size {
            StepSize::UseDefault => self.default_step,
            StepSize::Step(x) => x,
        };

        let k1 = if let Some(last_derivative) = mem::replace(&mut self.last_derivative, None) {
            last_derivative
        } else {
            diff_eq(start)
        };
        let k2 = diff_eq(&start.shift(&(k1.clone() / 5.0), h));
        let k3 = diff_eq(&start.shift(&(k1.clone() * 3.0 / 40.0 + k2.clone() * 9.0 / 40.0), h));
        let k4 = diff_eq(&start.shift(
            &(k1.clone() * 44.0 / 45.0 - k2.clone() * 56.0 / 15.0 + k3.clone() * 32.0 / 9.0),
            h,
        ));
        let k5 = diff_eq(&start.shift(
            &(k1.clone() * 19372.0 / 6561.0 - k2.clone() * 25360.0 / 2187.0
                + k3.clone() * 64448.0 / 6561.0
                - k4.clone() * 212.0 / 729.0),
            h,
        ));
        let k6 = diff_eq(&start.shift(
            &(k1.clone() * 9017.0 / 3168.0 - k2.clone() * 355.0 / 33.0
                + k3.clone() * 46732.0 / 5247.0
                + k4.clone() * 49.0 / 176.0
                - k5.clone() * 5103.0 / 18656.0),
            h,
        ));

        let new_state = start.shift(
            &(k1.clone() * 35.0 / 384.0 + k3.clone() * 500.0 / 1113.0 + k4.clone() * 125.0 / 192.0
                - k5.clone() * 2187.0 / 6784.0
                + k6.clone() * 11.0 / 84.0),
            h,
        );

        let k7 = diff_eq(&new_state);

        let error = ((k1 * 71.0 / 57600.0 - k3 * 71.0 / 16695.0 + k4 * 71.0 / 1920.0
            - k5 * 17253.0 / 339200.0
            + k6 * 22.0 / 525.0
            - k7.clone() / 40.0)
            * h)
            .abs();

        if error != 0.0 {
            self.default_step = h * (self.max_err / error).powf(0.25);
        } else {
            self.default_step = self.max_step;
        }

        if self.default_step < self.min_step {
            self.default_step = self.min_step;
        }
        if self.default_step > self.max_step {
            self.default_step = self.max_step;
        }

        // If the step adjustment makes it much smaller, repeat the calculation to avoid larger
        // errors
        if self.default_step < 0.8 * h && step_size == StepSize::UseDefault {
            self.propagate_in_place(start, diff_eq, step_size);
            return;
        }

        *start = new_state;

        //for optimization
        self.last_derivative = Some(k7);
    }
}
