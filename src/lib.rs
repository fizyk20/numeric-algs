pub mod integration;
mod traits;

pub use crate::traits::{State, StateDerivative};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
