
#[derive(PartialEq)]
pub enum PrimeResult {
    Prime,
    Composite,
}

pub mod trial_division;

pub use trial_division::{*};
