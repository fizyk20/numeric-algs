use super::{Integrator, StepSize};
use crate::traits::State;

pub struct RK8Integrator {
    default_step: f64,
}

impl RK8Integrator {
    pub fn new(step_size: f64) -> Self {
        RK8Integrator {
            default_step: step_size,
        }
    }

    pub fn set_default_step(&mut self, step: f64) {
        self.default_step = step;
    }
}

impl<S: State> Integrator<S> for RK8Integrator {
    fn propagate_in_place<D>(&mut self, start: &mut S, diff_eq: D, step_size: StepSize)
    where
        D: Fn(&S) -> S::Derivative,
    {
        let h = match step_size {
            StepSize::UseDefault => self.default_step,
            StepSize::Step(x) => x,
        };

        let mut f = vec![diff_eq(start)];

        let a: [(f64, f64); 9] = [
            (4.0, 27.0),
            (2.0, 9.0),
            (1.0, 3.0),
            (1.0, 2.0),
            (2.0, 3.0),
            (1.0, 6.0),
            (1.0, 1.0),
            (5.0, 6.0),
            (1.0, 1.0),
        ];

        let b: [(f64, Vec<f64>); 9] = [
            (1.0, vec![1.0]),
            (4.0, vec![1.0, 3.0]),
            (4.0, vec![1.0, 0.0, 3.0]),
            (4.0, vec![1.0, 0.0, 0.0, 3.0]),
            (36.0, vec![13.0, 0.0, -27.0, 42.0, 8.0]),
            (720.0, vec![389.0, 0.0, -54.0, 966.0, -824.0, 243.0]),
            (20.0, vec![-231.0, 0.0, 81.0, -1164.0, 656.0, -122.0, 800.0]),
            (
                240.0,
                vec![-127.0, 0.0, 18.0, -678.0, 456.0, -9.0, 576.0, 4.0],
            ),
            (
                820.0,
                vec![
                    1481.0, 0.0, -81.0, 7104.0, -3376.0, 72.0, -5040.0, -60.0, 720.0,
                ],
            ),
        ];

        for i in 0..9 {
            assert_eq!(f.len(), b[i].1.len());
            let k = f
                .iter()
                .zip(b[i].1.iter())
                .map(|(f, b)| f.clone() * *b)
                .reduce(|v1, v2| v1 + v2)
                .unwrap()
                / b[i].0;
            let a = a[i];
            let new_f = diff_eq(&start.shift(&k, h * a.0 / a.1));
            f.push(new_f);
        }

        let c: [f64; 10] = [41.0, 0.0, 0.0, 27.0, 272.0, 27.0, 216.0, 0.0, 216.0, 41.0];

        assert_eq!(f.len(), 10);

        let shift = f
            .iter()
            .zip(c.iter())
            .map(|(f, c)| f.clone() * *c)
            .reduce(|v1, v2| v1 + v2)
            .unwrap()
            / 840.0;

        start.shift_in_place(&shift, h);
    }
}
