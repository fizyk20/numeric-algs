use crate::StateDerivative;

pub trait State: Clone {
    type PositionDerivative: StateDerivative;
    type MomentumDerivative: StateDerivative;

    fn shift_position(&self, dir: &Self::PositionDerivative, amount: f64) -> Self {
        let mut result = self.clone();
        result.shift_position_in_place(dir, amount);
        result
    }

    fn shift_position_in_place(&mut self, dir: &Self::PositionDerivative, amount: f64);

    fn shift_momentum(&self, dir: &Self::MomentumDerivative, amount: f64) -> Self {
        let mut result = self.clone();
        result.shift_momentum_in_place(dir, amount);
        result
    }

    fn shift_momentum_in_place(&mut self, dir: &Self::MomentumDerivative, amount: f64);
}
