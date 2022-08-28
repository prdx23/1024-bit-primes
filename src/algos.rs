
#[derive(PartialEq)]
pub enum PrimeResult {
    Prime,
    Composite,
    Unknown,
}

pub mod trial_division;

pub use trial_division::{*};
