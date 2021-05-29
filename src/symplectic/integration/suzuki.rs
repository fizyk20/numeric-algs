use super::{super::State, Integrator, StepSize};

pub struct SuzukiIntegrator {
    default_step: f64,
}

impl SuzukiIntegrator {
    pub fn new(step_size: f64) -> Self {
        Self {
            default_step: step_size,
        }
    }

    pub fn set_default_step(&mut self, step: f64) {
        self.default_step = step;
    }
}

const CBRT4: f64 = 1.58740105196819947; // cube root of 4

// Algorithm constants
const L: f64 = 1.0 / (4.0 - CBRT4);
const C1: f64 = 0.5 * L;
const C2: f64 = L;
const C3: f64 = 0.5 * (1.0 - 3.0 * L);
const C4: f64 = 0.5 * (1.0 - 3.0 * L);
const C5: f64 = L;
const C6: f64 = 0.5 * L;

const D1: f64 = L;
const D2: f64 = L;
const D3: f64 = 1.0 - 4.0 * L;
const D4: f64 = L;
const D5: f64 = L;

impl<S: State> Integrator<S> for SuzukiIntegrator {
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
        start.shift_momentum_in_place(&momentum_diff_eq(start), h * D4);
        start.shift_position_in_place(&pos_diff_eq(start), h * C5);
        start.shift_momentum_in_place(&momentum_diff_eq(start), h * D5);
        start.shift_position_in_place(&pos_diff_eq(start), h * C6);
    }
}
