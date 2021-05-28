use super::{super::State, Integrator, StepSize};

pub struct NeriIntegrator {
    default_step: f64,
}

impl NeriIntegrator {
    pub fn new(step_size: f64) -> Self {
        Self {
            default_step: step_size,
        }
    }

    pub fn set_default_step(&mut self, step: f64) {
        self.default_step = step;
    }
}

const CBRT2: f64 = 1.25992104989487316; // cube root of 2

// Algorithm constants
const C1: f64 = 0.5 / (2.0 - CBRT2);
const C2: f64 = (1.0 - CBRT2) / 2.0 / (2.0 - CBRT2);
const C3: f64 = (1.0 - CBRT2) / 2.0 / (2.0 - CBRT2);
const C4: f64 = 0.5 / (2.0 - CBRT2);

const D1: f64 = 1.0 / (2.0 - CBRT2);
const D2: f64 = -CBRT2 / (2.0 - CBRT2);
const D3: f64 = 1.0 / (2.0 - CBRT2);

impl<S: State> Integrator<S> for NeriIntegrator {
    fn propagate_in_place<DF1, DF2>(
        &mut self,
        start: &mut S,
        pos_diff_eq: DF1,
        momentum_diff_eq: DF2,
        step_size: StepSize,
    ) where
        DF1: Fn(&S) -> S::PositionDerivative,
        DF2: Fn(&S) -> S::MomentumDerivative,
    {
        let h = match step_size {
            StepSize::UseDefault => self.default_step,
            StepSize::Step(x) => x,
        };

        start.shift_position_in_place(&pos_diff_eq(start), h * C1);
        start.shift_momentum_in_place(&momentum_diff_eq(start), h * D1);
        start.shift_position_in_place(&pos_diff_eq(start), h * C2);
        start.shift_momentum_in_place(&momentum_diff_eq(start), h * D2);
        start.shift_position_in_place(&pos_diff_eq(start), h * C3);
        start.shift_momentum_in_place(&momentum_diff_eq(start), h * D3);
        start.shift_position_in_place(&pos_diff_eq(start), h * C4);
    }
}
